use super::models::ClipPayload;
use super::ClipVideoUseCase;
use crate::error::CliptzyError;
use crate::orchestrator::pipeline::{emit_progress, ProgressEvent};
use crate::video::downloader::download_segment;
use std::path::Path;

impl ClipVideoUseCase {
    pub(super) async fn download_phase(
        &self,
        payload: &ClipPayload,
        source_video: &Path,
    ) -> Result<(), CliptzyError> {
        if source_video.exists() {
            emit_progress(
                &self.ctx.app_handle,
                &ProgressEvent {
                    stage: "download".into(),
                    label: "Menggunakan video dari cache...".into(),
                    current: 100,
                    total: 100,
                    detail: None,
                },
            );
            log::info!("Using cached source video: {:?}", source_video);
            return Ok(());
        }

        emit_progress(
            &self.ctx.app_handle,
            &ProgressEvent {
                stage: "download".into(),
                label: "Mendownload segmen video...".into(),
                current: 10,
                total: 100,
                detail: None,
            },
        );

        download_segment(
            &payload.url,
            payload.start,
            payload.end,
            source_video,
            payload.cookies_path.clone(),
            &self.ctx.deps.ytdlp,
            Some(&self.ctx.app_handle),
            self.ctx.cancel_token.clone(),
        )
        .await
    }
}
