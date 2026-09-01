use super::ClipVideoUseCase;
use crate::error::CliptzyError;
use crate::orchestrator::pipeline::{emit_progress, ProgressEvent};
use crate::processing::stacker::{stack_video, StackerConfig};
use crate::processing::thumbnail::generate_thumbnail;
use std::path::{Path, PathBuf};

impl ClipVideoUseCase {
    pub(super) async fn stack_phase(
        &self,
        current_video: &Path,
        final_video: &Path,
    ) -> Result<(), CliptzyError> {
        emit_progress(
            &self.ctx.app_handle,
            &ProgressEvent {
                stage: "stack".into(),
                label: "Menambahkan intro/outro jika ada...".into(),
                current: 80,
                total: 100,
                detail: None,
            },
        );

        let resolve_path = |p: Option<String>| -> Option<PathBuf> {
            p.map(|path_str| {
                if path_str.starts_with("assets/") || path_str.starts_with("assets\\") {
                    crate::paths::app_data_dir().join(path_str)
                } else {
                    PathBuf::from(path_str)
                }
            })
        };

        let stack_config = StackerConfig {
            intro_path: resolve_path(self.ctx.config.intro_video.clone()),
            outro_path: resolve_path(self.ctx.config.outro_video.clone()),
            watermark_path: None,
        };

        stack_video(current_video, final_video, &stack_config).await
    }

    pub(super) async fn thumbnail_phase(
        &self,
        final_video: &Path,
        thumb_path: &Path,
    ) -> Result<(), CliptzyError> {
        emit_progress(
            &self.ctx.app_handle,
            &ProgressEvent {
                stage: "thumbnail".into(),
                label: "Membuat thumbnail...".into(),
                current: 90,
                total: 100,
                detail: None,
            },
        );

        generate_thumbnail(final_video, thumb_path, 1.0).await
    }
}
