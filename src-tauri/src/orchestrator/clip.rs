use crate::error::CliptzyError;
use crate::orchestrator::pipeline::{PipelineContext, ProgressEvent, emit_progress};
use crate::video::downloader::download_segment;
use crate::processing::cropper::{create_crop_strategy, OutputConfig};
use crate::processing::stacker::{stack_video, StackerConfig};
use crate::processing::thumbnail::generate_thumbnail;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ClipPayload {
    pub url: String,
    pub video_id: String,
    pub start: f64,
    pub end: f64,
    pub crop_mode: String,
    pub use_subtitle: bool,
    pub cookies_path: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ClipResult {
    pub success: bool,
    pub output_path: String,
    pub thumbnail_path: String,
}

pub struct ClipVideoUseCase {
    ctx: PipelineContext,
}

impl ClipVideoUseCase {
    pub fn new(ctx: PipelineContext) -> Self {
        Self { ctx }
    }

    pub async fn execute(&mut self, payload: ClipPayload) -> Result<ClipResult, CliptzyError> {
        let job_dir = &self.ctx.job_dir;
        std::fs::create_dir_all(job_dir)?;
        
        let source_video = job_dir.join("source.mp4");
        let cropped_video = job_dir.join("cropped.mp4");
        let final_video = job_dir.join("final.mp4");
        let thumb_path = job_dir.join("thumbnail.jpg");

        // 1. Download Segment
        emit_progress(&self.ctx.app_handle, &ProgressEvent {
            stage: "download".into(),
            label: "Mendownload segmen video...".into(),
            current: 10,
            total: 100,
            detail: None,
        });
        
        download_segment(
            &payload.url,
            payload.start,
            payload.end,
            &source_video,
            payload.cookies_path.clone(),
            Some(&self.ctx.app_handle),
            self.ctx.cancel_token.clone(),
        ).await?;

        // 2. Probe Video
        let probe = crate::video::local::probe_local_video(&source_video).await?;

        // 3. Crop Video
        emit_progress(&self.ctx.app_handle, &ProgressEvent {
            stage: "crop".into(),
            label: "Memotong & menyesuaikan rasio video...".into(),
            current: 40,
            total: 100,
            detail: None,
        });
        
        let cropper = create_crop_strategy(&payload.crop_mode);
        let out_config = OutputConfig::default();
        let crop_cmd = cropper.build_command(&source_video, &cropped_video, &probe, &out_config)?;
        
        let crop_process = crop_cmd.spawn().await
            .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("Spawn failed: {}", e) })?;
        crop_process.wait().await
            .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("Crop failed: {}", e) })?;

        // 4. Transcription & Subtitle Burn (Optional)
        let current_video = cropped_video.clone();
        
        if payload.use_subtitle {
            emit_progress(&self.ctx.app_handle, &ProgressEvent {
                stage: "subtitle".into(),
                label: "Menghasilkan dan memasang subtitle otomatis...".into(),
                current: 60,
                total: 100,
                detail: None,
            });
            
            // Simplified transcription step for now
            // Assumes ASS file is generated
            // let subbed_video = job_dir.join("subbed.mp4");
            // burn_subtitle(&current_video, &subbed_video, &SubtitleBurnerConfig { ... }).await?;
            // current_video = subbed_video;
        }

        // 5. Stacker (Optional Intro/Outro)
        emit_progress(&self.ctx.app_handle, &ProgressEvent {
            stage: "stack".into(),
            label: "Menambahkan intro/outro jika ada...".into(),
            current: 80,
            total: 100,
            detail: None,
        });
        
        let stack_config = StackerConfig {
            intro_path: None,
            outro_path: None,
            watermark_path: None, // Can be extended to read from config
        };
        
        stack_video(&current_video, &final_video, &stack_config).await?;

        // 6. Generate Thumbnail
        emit_progress(&self.ctx.app_handle, &ProgressEvent {
            stage: "thumbnail".into(),
            label: "Membuat thumbnail...".into(),
            current: 90,
            total: 100,
            detail: None,
        });
        
        generate_thumbnail(&final_video, &thumb_path, 1.0).await?;

        emit_progress(&self.ctx.app_handle, &ProgressEvent {
            stage: "done".into(),
            label: "Selesai!".into(),
            current: 100,
            total: 100,
            detail: None,
        });

        Ok(ClipResult {
            success: true,
            output_path: final_video.to_string_lossy().to_string(),
            thumbnail_path: thumb_path.to_string_lossy().to_string(),
        })
    }
}
