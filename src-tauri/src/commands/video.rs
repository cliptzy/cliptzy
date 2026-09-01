use crate::config::models::AppConfig;
use crate::deps::AppDependencies;
use crate::error::CliptzyError;
use crate::orchestrator::clip::{ClipPayload, ClipResult, ClipVideoUseCase};
use crate::orchestrator::compilation::{
    EpicMoment, ExecuteCompilationUseCase, PrepareCompilationResult, PrepareCompilationUseCase,
};
use crate::orchestrator::pipeline::PipelineContext;
use crate::orchestrator::segment_audio::{
    AnalyzeSegmentAudioUseCase, SegmentAudioAnalysisResult,
};
use std::collections::HashMap;

#[tauri::command]
pub async fn analyze_video(
    url: String,
    cookies_path: Option<String>,
) -> Result<serde_json::Value, String> {
    let deps = AppDependencies::check()?;
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

    let ctx = build_pipeline_context(
        app.clone(),
        cancel_token,
        progress_tx,
        payload.video_id.clone(),
    )
    .await?;

    let mut use_case = ClipVideoUseCase::new(ctx);
    use_case.execute(payload).await
}

#[tauri::command]
pub async fn analyze_segment_audio(
    url: String,
    start: f64,
    end: f64,
    _stream_url: Option<String>,
) -> Result<SegmentAudioAnalysisResult, CliptzyError> {
    AnalyzeSegmentAudioUseCase::execute(&url, start, end).await
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

    let ctx = build_pipeline_context(app.clone(), cancel_token, progress_tx, video_id.clone())
        .await?;

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

    let ctx = build_pipeline_context(app.clone(), cancel_token, progress_tx, video_id.clone())
        .await?;

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

async fn build_pipeline_context(
    app_handle: tauri::AppHandle,
    cancel_token: tokio_util::sync::CancellationToken,
    progress_tx: crate::orchestrator::pipeline::ProgressTx,
    video_id: String,
) -> Result<PipelineContext, CliptzyError> {
    let config = AppConfig::load().unwrap_or_default();
    let job_dir = crate::paths::app_data_dir().join("jobs").join(&video_id);
    let deps = AppDependencies::check().map_err(CliptzyError::Download)?;

    Ok(PipelineContext {
        job_dir,
        video_id,
        config,
        cancel_token,
        progress_tx,
        app_handle,
        metadata: HashMap::new(),
        deps,
    })
}
