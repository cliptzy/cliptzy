mod crop;
mod download;
mod emotion;
mod finalize;
mod helpers;
mod models;
mod subtitle;

pub use models::{ClipPayload, ClipResult};

use crate::error::CliptzyError;
use crate::orchestrator::pipeline::{emit_progress, PipelineContext, ProgressEvent};
use crate::processing::cropper::OutputConfig;
use helpers::{apply_segment_bounds, clip_paths, probe_output_dimensions};

pub struct ClipVideoUseCase {
    pub(super) ctx: PipelineContext,
}

impl ClipVideoUseCase {
    pub fn new(ctx: PipelineContext) -> Self {
        Self { ctx }
    }

    pub async fn execute(&mut self, mut payload: ClipPayload) -> Result<ClipResult, CliptzyError> {
        apply_segment_bounds(&mut payload, &self.ctx.config);

        let job_dir = &self.ctx.job_dir;
        std::fs::create_dir_all(job_dir)?;

        let paths = clip_paths(job_dir, payload.segment_index);
        let total_duration = payload.end - payload.start;

        self.download_phase(&payload, &paths.source).await?;
        self.emotion_phase(&paths.source, payload.segment_index)
            .await?;

        let hw_accel =
            crate::processing::ffmpeg::hwaccel::HwAccel::detect(Some(&self.ctx.config.hw_accel));

        let mut current_video = self
            .crop_phase(&payload, &paths.source, &paths.cropped, total_duration)
            .await?;

        let mut out_config = OutputConfig {
            hw_accel: hw_accel.clone(),
            ..OutputConfig::default()
        };
        if payload.crop_mode == "none" {
            out_config = probe_output_dimensions(&current_video, out_config).await;
        }

        let has_watermark = self.ctx.config.burn_watermark
            && self
                .ctx
                .config
                .watermark_image
                .as_ref()
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false);

        if self.ctx.config.burn_subtitle || has_watermark {
            current_video = self
                .subtitle_phase(
                    &payload,
                    &current_video,
                    payload.segment_index,
                    &hw_accel,
                    &out_config,
                    total_duration,
                )
                .await?;
        }

        self.stack_phase(&current_video, &paths.final_video).await?;
        self.thumbnail_phase(&paths.final_video, &paths.thumb)
            .await?;

        emit_progress(
            &self.ctx.app_handle,
            &ProgressEvent {
                stage: "done".into(),
                label: "Selesai!".into(),
                current: 100,
                total: 100,
                detail: None,
            },
        );

        Ok(ClipResult {
            success: true,
            output_path: paths.final_video.to_string_lossy().to_string(),
            thumbnail_path: paths.thumb.to_string_lossy().to_string(),
        })
    }
}
