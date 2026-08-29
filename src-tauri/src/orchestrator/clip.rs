use crate::error::CliptzyError;
use crate::orchestrator::pipeline::{emit_progress, PipelineContext, ProgressEvent};
use crate::processing::cropper::{create_crop_strategy, OutputConfig};
use crate::processing::stacker::{stack_video, StackerConfig};
use crate::processing::thumbnail::generate_thumbnail;
use crate::video::downloader::download_segment;
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
    pub segment_index: u32,
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

    pub async fn execute(&mut self, mut payload: ClipPayload) -> Result<ClipResult, CliptzyError> {
        // Apply global min_duration & padding dynamically
        let min_dur = self.ctx.config.min_duration as f64;
        let padding = self.ctx.config.padding as f64;

        let duration = payload.end - payload.start;
        if duration > 0.0 && duration < min_dur {
            let deficit = min_dur - duration;
            payload.start -= deficit / 2.0;
            payload.end += deficit / 2.0;
        }

        payload.start -= padding;
        payload.end += padding;

        if payload.start < 0.0 {
            let underflow = 0.0 - payload.start;
            payload.start = 0.0;
            payload.end += underflow;
        }

        let job_dir = &self.ctx.job_dir;
        std::fs::create_dir_all(job_dir)?;

        let idx = payload.segment_index;
        let source_video = job_dir.join(format!("source_{}.mp4", idx));
        let cropped_video = job_dir.join(format!("cropped_{}.mp4", idx));
        let final_video = job_dir.join(format!("final_{}.mp4", idx));
        let thumb_path = job_dir.join(format!("thumbnail_{}.jpg", idx));

        // 1. Download Segment
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
        } else {
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
                &source_video,
                payload.cookies_path.clone(),
                &self.ctx.deps.ytdlp,
                Some(&self.ctx.app_handle),
                self.ctx.cancel_token.clone(),
            )
            .await?;
        }

        // 2. Removed Probe Video due to rust_ffprobe parsing bugs

        // 3. Crop Video
        emit_progress(
            &self.ctx.app_handle,
            &ProgressEvent {
                stage: "crop".into(),
                label: "Memotong & menyesuaikan rasio video...".into(),
                current: 40,
                total: 100,
                detail: None,
            },
        );

        let mut keyframes = None;
        if payload.crop_mode == "full_face" || payload.crop_mode == "center_face" {
            emit_progress(
                &self.ctx.app_handle,
                &ProgressEvent {
                    stage: "crop".into(),
                    label: "Menganalisa wajah (AI Tracking)...".into(),
                    current: 45,
                    total: 100,
                    detail: None,
                },
            );
            let tracking_mode = self.ctx.config.face_tracking_mode.clone();
            match crate::face::tracker::get_face_keyframes(
                &source_video,
                1.0,
                tracking_mode,
                Some(self.ctx.app_handle.clone()),
                self.ctx.cancel_token.clone(),
            )
            .await
            {
                Ok(kfs) => keyframes = Some(kfs),
                Err(e) => {
                    log::warn!("Face tracking failed: {}. Fallback to center.", e);
                }
            }
        }

        let cropper = create_crop_strategy(&payload.crop_mode);
        let hw_accel =
            crate::processing::ffmpeg::hwaccel::HwAccel::detect(Some(&self.ctx.config.hw_accel));
        let out_config = OutputConfig {
            hw_accel: hw_accel.clone(),
            ..OutputConfig::default()
        };
        let total_duration = payload.end - payload.start;
        let handle_clone = self.ctx.app_handle.clone();

        let crop_cmd = cropper
            .build_command(
                &source_video,
                &cropped_video,
                &out_config,
                keyframes.as_deref(),
            )?
            .on_progress(move |prog| {
                if let Some(time) = prog.time {
                    let current_sec = time.as_secs_f64();
                    if total_duration > 0.0 {
                        let mut pct = (current_sec / total_duration) * 100.0;
                        if pct > 99.9 {
                            pct = 99.9;
                        }
                        emit_progress(
                            &handle_clone,
                            &ProgressEvent {
                                stage: "crop".into(),
                                label: format!(
                                    "Memotong & menyesuaikan rasio video... ({:.1}%)",
                                    pct
                                ),
                                current: pct as u32,
                                total: 100,
                                detail: None,
                            },
                        );
                    }
                }
            });

        let crop_process = crop_cmd.spawn().await.map_err(|e| CliptzyError::FFmpeg {
            code: -1,
            message: format!("Spawn failed: {}", e),
        })?;
        crop_process
            .wait()
            .await
            .map_err(|e| CliptzyError::FFmpeg {
                code: -1,
                message: format!("Crop failed: {}", e),
            })?;

        // 4. Transcription & Subtitle Burn (Optional)
        let mut current_video = cropped_video.clone();

        let has_watermark = self
            .ctx
            .config
            .watermark_image
            .as_ref()
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false);
        if payload.use_subtitle || has_watermark {
            emit_progress(
                &self.ctx.app_handle,
                &ProgressEvent {
                    stage: "subtitle".into(),
                    label: "Menambahkan efek visual/teks ke video...".into(),
                    current: 60,
                    total: 100,
                    detail: None,
                },
            );

            let mut ass_path_opt = None;
            let mut sub_config_opt = None;

            if payload.use_subtitle {
                // Extract audio for Whisper using existing audio module
                emit_progress(
                    &self.ctx.app_handle,
                    &ProgressEvent {
                        stage: "subtitle".into(),
                        label: "Mengekstrak audio untuk AI Transcription...".into(),
                        current: 62,
                        total: 100,
                        detail: None,
                    },
                );

                let audio_wav = job_dir.join(format!("audio_16k_{}.wav", idx));
                let duration = payload.end - payload.start;
                crate::transcription::audio::extract_audio_segment(
                    &current_video.to_string_lossy(),
                    0.0,
                    duration,
                    &audio_wav,
                    None,
                    &self.ctx.deps.ytdlp,
                )
                .await?;

                // Transcribe
                emit_progress(
                    &self.ctx.app_handle,
                    &ProgressEvent {
                        stage: "subtitle".into(),
                        label: "Menyiapkan AI Whisper...".into(),
                        current: 65,
                        total: 100,
                        detail: None,
                    },
                );
                let whisper_model = if self.ctx.config.subtitle.whisper_model.is_empty() {
                    "tiny".to_string()
                } else {
                    self.ctx.config.subtitle.whisper_model.clone()
                };
                let model_path =
                    crate::transcription::whisper::ensure_model_exists(&whisper_model).await?;
                let transcriber =
                    crate::transcription::whisper::WhisperTranscriber::new(&model_path)?;

                emit_progress(
                    &self.ctx.app_handle,
                    &ProgressEvent {
                        stage: "subtitle".into(),
                        label: "Menjalankan Transkripsi Teks (Whisper)...".into(),
                        current: 70,
                        total: 100,
                        detail: None,
                    },
                );
                let transcript = transcriber.transcribe(&audio_wav).await?;

                // Generate ASS
                emit_progress(
                    &self.ctx.app_handle,
                    &ProgressEvent {
                        stage: "subtitle".into(),
                        label: "Menyusun format Subtitle (ASS)...".into(),
                        current: 75,
                        total: 100,
                        detail: None,
                    },
                );
                let ass_path = job_dir.join(format!("subtitles_{}.ass", idx));
                let mut sub_config = crate::transcription::models::SubtitleConfig::default();
                if !self.ctx.config.subtitle.font.is_empty() {
                    sub_config.font = self.ctx.config.subtitle.font.clone();
                }
                if self.ctx.config.subtitle.font_size > 0 {
                    sub_config.font_size = self.ctx.config.subtitle.font_size;
                }
                if !self.ctx.config.subtitle.color.is_empty() {
                    sub_config.primary_color = self.ctx.config.subtitle.color.clone();
                }
                if !self.ctx.config.subtitle.bg_color.is_empty() {
                    sub_config.back_color = self.ctx.config.subtitle.bg_color.clone();
                }
                if self.ctx.config.subtitle.border_style > 0 {
                    sub_config.border_style = self.ctx.config.subtitle.border_style;
                }
                if !self.ctx.config.subtitle.animation.is_empty() {
                    sub_config.animation = self.ctx.config.subtitle.animation.clone();
                }
                if self.ctx.config.subtitle.max_words > 0 {
                    sub_config.max_words_per_line = self.ctx.config.subtitle.max_words as usize;
                }
                sub_config.alignment = match self.ctx.config.subtitle.location.as_str() {
                    "top" => 8,
                    "center" => 5,
                    "bottom" => 2,
                    _ => 2,
                };

                // Calculate dynamic margin_v based on height to match the UI's bottom-24 visual placement
                sub_config.margin_v = (out_config.height as f32 * 0.12) as u32;

                // Apply hardcoded overrides if Brutalist Box (border_style == 3) is selected
                if sub_config.border_style == 3 {
                    sub_config.font = "Courier New".to_string();
                    sub_config.primary_color = "&H00FFFFFF".to_string(); // White text
                    sub_config.outline_color = "&H002626DC".to_string(); // Red 600 background block
                    sub_config.back_color = "&H00000000".to_string(); // Black shadow
                    sub_config.outline = 4; // Padding
                    sub_config.shadow = 4; // Shadow offset
                }

                crate::transcription::ass_writer::generate_ass_file(
                    &transcript,
                    &ass_path,
                    &sub_config,
                    (out_config.width, out_config.height),
                )?;

                ass_path_opt = Some(ass_path.to_string_lossy().to_string());
                sub_config_opt = Some(sub_config);
            }

            // Burn Subtitle (and optionally Watermark)
            emit_progress(
                &self.ctx.app_handle,
                &ProgressEvent {
                    stage: "subtitle".into(),
                    label: "Mempersiapkan proses rendering efek...".into(),
                    current: 78,
                    total: 100,
                    detail: None,
                },
            );

            let subbed_video = job_dir.join(format!("subbed_{}.mp4", idx));
            let burn_config = crate::processing::burner::VideoBurnerConfig {
                ass_path: ass_path_opt,
                vfx_overlay_path: None,
                normalize_audio: true,
                config: sub_config_opt,
                watermark_path: self.ctx.config.watermark_image.clone(),
                watermark_position: self.ctx.config.watermark_position.clone(),
                hw_accel: hw_accel.clone(),
            };
            crate::processing::burner::burn_video_effects(
                &current_video,
                &subbed_video,
                &burn_config,
                Some((&self.ctx.app_handle, total_duration)),
            )
            .await?;
            current_video = subbed_video;
        }

        // 5. Stacker (Optional Intro/Outro)
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

        let resolve_path = |p: Option<String>| -> Option<std::path::PathBuf> {
            p.map(|path_str| {
                if path_str.starts_with("assets/") || path_str.starts_with("assets\\") {
                    crate::paths::app_data_dir().join(path_str)
                } else {
                    std::path::PathBuf::from(path_str)
                }
            })
        };

        let stack_config = StackerConfig {
            intro_path: resolve_path(self.ctx.config.intro_video.clone()),
            outro_path: resolve_path(self.ctx.config.outro_video.clone()),
            watermark_path: None, // Watermark is handled by subtitle_burner
        };

        stack_video(&current_video, &final_video, &stack_config).await?;

        // 6. Generate Thumbnail
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

        generate_thumbnail(&final_video, &thumb_path, 1.0).await?;

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
            output_path: final_video.to_string_lossy().to_string(),
            thumbnail_path: thumb_path.to_string_lossy().to_string(),
        })
    }
}
