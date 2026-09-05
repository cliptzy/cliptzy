mod crop;
mod download;
mod emotion;
mod finalize;
mod helpers;
pub mod models;
mod subtitle;

pub use models::{ClipPayload, ClipResult, EmotionCacheEntry};

use crate::error::CliptzyError;
use crate::orchestrator::pipeline::{emit_progress, PipelineContext, ProgressEvent};
use crate::processing::cropper::OutputConfig;
use helpers::{apply_segment_bounds, clip_paths, probe_output_dimensions, sanitize_title};

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
        self.emotion_phase(&payload, &paths.source, payload.segment_index)
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

        // Salin hasil final ke folder output dengan nama berdasarkan judul video.
        let output_dir = crate::paths::app_data_dir().join("output");
        std::fs::create_dir_all(&output_dir)?;
        let base_name = format!("{}_{}", sanitize_title(&payload.title), payload.segment_index);
        let final_output = output_dir.join(format!("{}.mp4", base_name));
        let thumb_output = output_dir.join(format!("{}.jpg", base_name));
        std::fs::copy(&paths.final_video, &final_output).map_err(|e| {
            CliptzyError::Internal(format!(
                "Gagal menyalin video final ke {:?}: {}",
                final_output, e
            ))
        })?;
        if paths.thumb.exists() {
            let _ = std::fs::copy(&paths.thumb, &thumb_output);
        }
        log::info!("Video final disalin ke {:?}", final_output);

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
            output_path: final_output.to_string_lossy().to_string(),
            thumbnail_path: thumb_output.to_string_lossy().to_string(),
        })
    }
}
