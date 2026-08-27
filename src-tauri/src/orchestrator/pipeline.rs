use serde::Serialize;
use tauri::Emitter;

#[derive(Serialize, Clone, Debug)]
pub struct ProgressEvent {
    pub stage: String,
    pub label: String,
    pub current: u32,
    pub total: u32,
    pub detail: Option<String>,
}

pub fn emit_progress(handle: &tauri::AppHandle, event: &ProgressEvent) {
    let _ = handle.emit("clip-progress", event);
}

use crate::config::models::AppConfig;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

pub type ProgressTx = broadcast::Sender<ProgressEvent>;

pub struct PipelineContext {
    pub job_dir: PathBuf,
    pub video_id: String,
    pub config: AppConfig,
    pub cancel_token: CancellationToken,
    pub progress_tx: ProgressTx,
    pub app_handle: tauri::AppHandle,
    pub metadata: HashMap<String, serde_json::Value>,
    pub deps: crate::utils::AppDependencies,
}

#[async_trait::async_trait]
pub trait PipelineStage: Send + Sync {
    fn name(&self) -> &str;
    async fn execute(&self, ctx: &mut PipelineContext) -> Result<(), crate::error::CliptzyError>;
    fn can_skip(&self, _ctx: &PipelineContext) -> bool {
        false
    }
}
