use super::models::ClipPayload;
use super::ClipVideoUseCase;
use crate::error::CliptzyError;
use crate::orchestrator::job_cache::cache_file;
use crate::orchestrator::pipeline::{emit_progress, ProgressEvent};
use crate::processing::cropper::{create_crop_strategy, OutputConfig};
use std::path::{Path, PathBuf};

impl ClipVideoUseCase {
    pub(super) async fn crop_phase(
        &self,
        payload: &ClipPayload,
        source_video: &Path,
        cropped_video: &Path,
        total_duration: f64,
    ) -> Result<PathBuf, CliptzyError> {
        if payload.crop_mode == "none" {
            emit_progress(
                &self.ctx.app_handle,
                &ProgressEvent {
                    stage: "crop".into(),
                    label: "Mempertahankan resolusi asli (tanpa crop)...".into(),
                    current: 50,
                    total: 100,
                    detail: None,
                },
            );
            log::info!("crop_mode=none, melewati proses crop untuk {:?}", source_video);
            return Ok(source_video.to_path_buf());
        }

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

        let keyframes = self.resolve_face_keyframes(payload, source_video).await;
        let debug_ass_path = self
            .resolve_debug_ass_path(source_video, payload.segment_index)
            .await;

        let hw_accel =
            crate::processing::ffmpeg::hwaccel::HwAccel::detect(Some(&self.ctx.config.hw_accel));
        let crop_out_config = OutputConfig {
            hw_accel: hw_accel.clone(),
            debug_ass_path,
            ..OutputConfig::default()
        };

        let cropper = create_crop_strategy(&payload.crop_mode);
        let handle_clone = self.ctx.app_handle.clone();

        let crop_cmd = cropper
            .build_command(
                source_video,
                cropped_video,
                &crop_out_config,
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

        #[allow(unused_mut)]
        let mut crop_process = crop_cmd.spawn().await.map_err(|e| CliptzyError::FFmpeg {
            code: -1,
            message: format!("Spawn failed: {}", e),
        })?;

        tokio::select! {
            status = crop_process.wait() => {
                status.map_err(|e| CliptzyError::FFmpeg {
                    code: -1,
                    message: format!("Crop failed: {}", e),
                })?;
            }
            _ = self.ctx.cancel_token.cancelled() => {
                log::warn!("Membatalkan proses crop...");
                crate::utils::kill_processes(&["ffmpeg", "yt-dlp"]);
                return Err(CliptzyError::Config("Proses crop dibatalkan oleh pengguna".into()));
            }
        }

        Ok(cropped_video.to_path_buf())
    }

    async fn resolve_face_keyframes(
        &self,
        payload: &ClipPayload,
        source_video: &Path,
    ) -> Option<Vec<crate::face::models::FaceKeyframe>> {
        if payload.crop_mode != "full_face" && payload.crop_mode != "center_face" {
            return None;
        }

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
            source_video,
            1.0,
            tracking_mode,
            Some(self.ctx.app_handle.clone()),
            self.ctx.cancel_token.clone(),
            None,
        )
        .await
        {
            Ok((kfs, _)) => Some(kfs),
            Err(e) => {
                log::warn!("Face tracking failed: {}. Fallback to center.", e);
                None
            }
        }
    }

    async fn resolve_debug_ass_path(
        &self,
        source_video: &Path,
        idx: u32,
    ) -> Option<String> {
        if !self.ctx.config.debug_mode {
            return None;
        }

        let emotion_cache_path = cache_file(&self.ctx.job_dir, &format!("emotions_{}.json", idx));
        let ass_out = self.ctx.job_dir.join(format!("debug_boxes_{}.ass", idx));

        crate::transcription::ass_writer::try_generate_emotion_debug_ass(
            source_video,
            &emotion_cache_path,
            &ass_out,
        )
        .await
        .map(|p| p.to_string_lossy().to_string())
    }
}
