use crate::config::models::AppConfig;
use crate::error::CliptzyError;
use crate::orchestrator::clip::{ClipPayload, ClipResult, ClipVideoUseCase};
use crate::orchestrator::compilation::{
    EpicMoment, ExecuteCompilationUseCase, PrepareCompilationResult, PrepareCompilationUseCase,
};
use crate::orchestrator::pipeline::PipelineContext;
use std::collections::HashMap;

#[tauri::command]
pub async fn analyze_video(
    url: String,
    cookies_path: Option<String>,
) -> Result<serde_json::Value, String> {
    let deps = crate::utils::AppDependencies::check()?;
    let result =
        crate::video::youtube::analyze_youtube_video(&url, cookies_path, &deps.ytdlp).await?;
    Ok(serde_json::to_value(result).unwrap_or(serde_json::json!({})))
}

#[tauri::command]
pub async fn clip_video(
    app: tauri::AppHandle,
    state: tauri::State<'_, crate::AppState>,
    payload: ClipPayload,
) -> Result<ClipResult, CliptzyError> {
    log::info!(
        "Menerima permintaan clip_video untuk Video ID: {}",
        payload.video_id
    );
    let cancel_token = tokio_util::sync::CancellationToken::new();
    *state.cancel_token.lock().await = Some(cancel_token.clone());
    let (progress_tx, _) = tokio::sync::broadcast::channel(100);

    let config = AppConfig::load().unwrap_or_default();
    let app_dir = crate::paths::app_data_dir();
    let job_dir = app_dir.join("jobs").join(payload.video_id.clone());
    let deps = crate::utils::AppDependencies::check().map_err(|e| CliptzyError::Download(e))?;

    let ctx = PipelineContext {
        job_dir,
        video_id: payload.video_id.clone(),
        config,
        cancel_token,
        progress_tx,
        app_handle: app.clone(),
        metadata: HashMap::new(),
        deps,
    };

    let mut use_case = ClipVideoUseCase::new(ctx);
    let result = use_case.execute(payload).await?;

    Ok(result)
}

#[derive(serde::Serialize)]
pub struct SegmentAnalysisResult {
    pub transcript: Vec<crate::transcription::models::TranscriptionSegment>,
    pub ai_effects: Vec<serde_json::Value>,
}

#[tauri::command]
pub async fn analyze_segment_audio(
    _url: String,
    start: f64,
    end: f64,
    _stream_url: Option<String>,
) -> Result<SegmentAnalysisResult, CliptzyError> {
    let app_dir = crate::paths::app_data_dir();
    let temp_dir = app_dir.join("temp");
    std::fs::create_dir_all(&temp_dir).ok();

    let file_name = format!(
        "seg_{}_{}_{}.wav",
        uuid::Uuid::new_v4()
            .to_string()
            .chars()
            .take(8)
            .collect::<String>(),
        start,
        end
    );
    let audio_wav_path = temp_dir.join(file_name);
    log::info!(
        "Analisis segmen audio dimulai | Start: {}s, End: {}s",
        start,
        end
    );

    let config = crate::config::models::AppConfig::load().unwrap_or_default();
    let cookies_path = config.browser.as_deref().filter(|s| !s.is_empty());
    let deps = crate::utils::AppDependencies::check().map_err(|e| CliptzyError::Download(e))?;

    log::info!("Tahap 1: Ekstraksi WAV melalui yt-dlp/FFmpeg...");
    crate::transcription::audio::extract_audio_segment(
        &_url,
        start,
        end,
        &audio_wav_path,
        cookies_path,
        &deps.ytdlp,
    )
    .await?;

    let whisper_model = if config.subtitle.whisper_model.is_empty() {
        "tiny".to_string()
    } else {
        config.subtitle.whisper_model.clone()
    };
    log::info!(
        "Tahap 2: Memeriksa dan memuat model Whisper ({})...",
        whisper_model
    );
    let model_path = crate::transcription::whisper::ensure_model_exists(&whisper_model).await?;

    log::info!("Tahap 3: Menjalankan transkripsi Whisper (local)...");
    let transcriber = crate::transcription::whisper::WhisperTranscriber::new(&model_path)?;
    let transcript = transcriber.transcribe(&audio_wav_path).await?;

    log::info!("Tahap 4: Membersihkan file audio sementara...");
    let _ = std::fs::remove_file(&audio_wav_path);

    log::info!(
        "Analisis segmen audio selesai. Ditemukan {} blok teks.",
        transcript.len()
    );

    Ok(SegmentAnalysisResult {
        transcript,
        ai_effects: vec![],
    })
}

#[tauri::command]
pub async fn prepare_compilation(
    app: tauri::AppHandle,
    state: tauri::State<'_, crate::AppState>,
    video_url: String,
    video_id: String,
    search_keywords: Option<String>,
) -> Result<PrepareCompilationResult, CliptzyError> {
    log::info!(
        "Menerima permintaan prepare_compilation untuk Video ID: {}",
        video_id
    );

    let cancel_token = tokio_util::sync::CancellationToken::new();
    *state.cancel_token.lock().await = Some(cancel_token.clone());
    let (progress_tx, _) = tokio::sync::broadcast::channel(100);

    let config = AppConfig::load().unwrap_or_default();
    let app_dir = crate::paths::app_data_dir();
    let job_dir = app_dir.join("jobs").join(video_id.clone());
    let deps = crate::utils::AppDependencies::check().map_err(|e| CliptzyError::Download(e))?;

    let ctx = PipelineContext {
        job_dir,
        video_id: video_id.clone(),
        config,
        cancel_token,
        progress_tx,
        app_handle: app.clone(),
        metadata: HashMap::new(),
        deps,
    };

    let mut use_case = PrepareCompilationUseCase::new(ctx);
    use_case
        .execute(video_url, search_keywords)
        .await
        .map_err(|e| {
            log::error!(
                "prepare_compilation gagal untuk Video ID {}: {}",
                video_id,
                e
            );
            e
        })
}

#[tauri::command]
pub async fn execute_compilation(
    app: tauri::AppHandle,
    state: tauri::State<'_, crate::AppState>,
    video_id: String,
    main_audio_path: String,
    restreamer_urls: Vec<String>,
    moments: Vec<EpicMoment>,
    output_filename: String,
) -> Result<String, CliptzyError> {
    log::info!(
        "Menerima permintaan execute_compilation untuk Video ID: {}",
        video_id
    );

    let cancel_token = tokio_util::sync::CancellationToken::new();
    *state.cancel_token.lock().await = Some(cancel_token.clone());
    let (progress_tx, _) = tokio::sync::broadcast::channel(100);

    let config = AppConfig::load().unwrap_or_default();
    let app_dir = crate::paths::app_data_dir();
    let job_dir = app_dir.join("jobs").join(video_id.clone());
    let deps = crate::utils::AppDependencies::check().map_err(|e| CliptzyError::Download(e))?;

    let ctx = PipelineContext {
        job_dir,
        video_id: video_id.clone(),
        config,
        cancel_token,
        progress_tx,
        app_handle: app.clone(),
        metadata: HashMap::new(),
        deps,
    };

    let mut use_case = ExecuteCompilationUseCase::new(ctx);
    use_case
        .execute(
            main_audio_path,
            restreamer_urls,
            moments,
            output_filename,
        )
        .await
        .map_err(|e| {
            log::error!(
                "execute_compilation gagal untuk Video ID {}: {}",
                video_id,
                e
            );
            e
        })
}
