#[tauri::command]
pub async fn analyze_video(
    url: String,
    cookies_path: Option<String>,
) -> Result<serde_json::Value, String> {
    let result = crate::video::youtube::analyze_youtube_video(&url, cookies_path).await?;
    Ok(serde_json::to_value(result).unwrap_or(serde_json::json!({})))
}

use crate::error::CliptzyError;
use crate::orchestrator::pipeline::PipelineContext;
use crate::orchestrator::clip::{ClipPayload, ClipVideoUseCase, ClipResult};
use crate::config::models::AppConfig;
use std::collections::HashMap;
use tokio_util::sync::CancellationToken;

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
    let job_dir = app_dir.join("jobs").join(uuid::Uuid::new_v4().to_string());
    
    let ctx = PipelineContext {
        job_dir,
        video_id: payload.video_id.clone(),
        config,
        cancel_token,
        progress_tx,
        app_handle: app.clone(),
        metadata: HashMap::new(),
    };
    
    let mut use_case = ClipVideoUseCase::new(ctx);
    let result = use_case.execute(payload).await?;
    
    Ok(result)
}
