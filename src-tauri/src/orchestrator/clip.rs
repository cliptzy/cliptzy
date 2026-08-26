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

        // 2. Removed Probe Video due to rust_ffprobe parsing bugs

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
        let crop_cmd = cropper.build_command(&source_video, &cropped_video, &out_config)?;
        
        let crop_process = crop_cmd.spawn().await
            .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("Spawn failed: {}", e) })?;
        crop_process.wait().await
            .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("Crop failed: {}", e) })?;

        // 4. Transcription & Subtitle Burn (Optional)
        let mut current_video = cropped_video.clone();
        
        if payload.use_subtitle {
            emit_progress(&self.ctx.app_handle, &ProgressEvent {
                stage: "subtitle".into(),
                label: "Menghasilkan dan memasang subtitle otomatis...".into(),
                current: 60,
                total: 100,
                detail: None,
            });
            
            // Extract audio for Whisper using existing audio module
            let audio_wav = job_dir.join("audio_16k.wav");
            let duration = payload.end - payload.start;
            crate::transcription::audio::extract_audio_segment(
                &current_video.to_string_lossy(),
                0.0,
                duration,
                &audio_wav,
                None,
            ).await?;

            // Transcribe
            let whisper_model = if self.ctx.config.subtitle.whisper_model.is_empty() {
                "tiny".to_string()
            } else {
                self.ctx.config.subtitle.whisper_model.clone()
            };
            let model_path = crate::transcription::whisper::ensure_model_exists(&whisper_model).await?;
            let transcriber = crate::transcription::whisper::WhisperTranscriber::new(&model_path)?;
            let transcript = transcriber.transcribe(&audio_wav).await?;
            
            // Generate ASS
            let ass_path = job_dir.join("subtitles.ass");
            let mut sub_config = crate::transcription::models::SubtitleConfig::default();
            if !self.ctx.config.subtitle.font.is_empty() {
                sub_config.font = self.ctx.config.subtitle.font.clone();
            }
            if self.ctx.config.subtitle.font_size > 0 {
                sub_config.font_size = self.ctx.config.subtitle.font_size;
            }
            
            crate::transcription::ass_writer::generate_ass_file(
                &transcript, 
                &ass_path, 
                &sub_config, 
                (out_config.width, out_config.height)
            )?;

            // Burn Subtitle
            let subbed_video = job_dir.join("subbed.mp4");
            let burn_config = crate::processing::subtitle_burner::SubtitleBurnerConfig {
                ass_path: ass_path.to_string_lossy().to_string(),
                vfx_overlay_path: None,
                normalize_audio: true,
            };
            crate::processing::subtitle_burner::burn_subtitle(&current_video, &subbed_video, &burn_config).await?;
            current_video = subbed_video;
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
