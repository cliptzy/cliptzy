use crate::config::models::AppConfig;
use crate::error::CliptzyError;
use crate::orchestrator::clip::{ClipPayload, ClipResult, ClipVideoUseCase};
use crate::orchestrator::pipeline::PipelineContext;
use std::collections::HashMap;
use tokio_util::sync::CancellationToken;

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
    payload: ClipPayload,
) -> Result<ClipResult, CliptzyError> {
    let cancel_token = CancellationToken::new();
    let (progress_tx, _) = tokio::sync::broadcast::channel(100);

    // Load config
    let config = AppConfig::load().unwrap_or_default();

    // Setup job dir
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

    // Hash URL and timing to avoid re-downloading during same session
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
    tracing::info!(
        "Analisis segmen audio dimulai | Start: {}s, End: {}s",
        start,
        end
    );

    // Load config to check for youtube cookies
    let config = crate::config::models::AppConfig::load().unwrap_or_default();
    let cookies_path = config.browser.as_deref().filter(|s| !s.is_empty());
    let deps = crate::utils::AppDependencies::check().map_err(|e| CliptzyError::Download(e))?;

    // 1. Extract audio chunk (pass the original YouTube URL so yt-dlp can handle cookies/throttling)
    tracing::info!("Tahap 1: Ekstraksi WAV melalui yt-dlp/FFmpeg...");
    crate::transcription::audio::extract_audio_segment(
        &_url,
        start,
        end,
        &audio_wav_path,
        cookies_path,
        &deps.ytdlp,
    )
    .await?;

    // 2. Ensure model exists
    let whisper_model = if config.subtitle.whisper_model.is_empty() {
        "tiny".to_string()
    } else {
        config.subtitle.whisper_model.clone()
    };
    tracing::info!(
        "Tahap 2: Memeriksa dan memuat model Whisper ({})...",
        whisper_model
    );
    let model_path = crate::transcription::whisper::ensure_model_exists(&whisper_model).await?;

    // 3. Transcribe audio
    tracing::info!("Tahap 3: Menjalankan transkripsi Whisper (local)...");
    let transcriber = crate::transcription::whisper::WhisperTranscriber::new(&model_path)?;
    let transcript = transcriber.transcribe(&audio_wav_path).await?;

    // Clean up temporary audio file
    tracing::info!("Tahap 4: Membersihkan file audio sementara...");
    let _ = std::fs::remove_file(&audio_wav_path);

    tracing::info!(
        "Analisis segmen audio selesai. Ditemukan {} blok teks.",
        transcript.len()
    );

    Ok(SegmentAnalysisResult {
        transcript,
        ai_effects: vec![],
    })
}
