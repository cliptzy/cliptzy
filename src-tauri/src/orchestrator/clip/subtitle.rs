use super::models::{ClipPayload, SegmentTranscriptCacheEntry};
use super::ClipVideoUseCase;
use crate::error::CliptzyError;
use crate::orchestrator::job_cache::{
    cache_file, fingerprint, is_fingerprint_valid, read_json_cache, sanitize_cache_token,
    write_json_cache,
};
use crate::orchestrator::pipeline::{emit_progress, ProgressEvent};
use crate::processing::cropper::OutputConfig;
use crate::transcription::models::SubtitleConfig;
use std::path::{Path, PathBuf};

impl ClipVideoUseCase {
    pub(super) async fn subtitle_phase(
        &self,
        payload: &ClipPayload,
        current_video: &Path,
        idx: u32,
        hw_accel: &crate::processing::ffmpeg::hwaccel::HwAccel,
        out_config: &OutputConfig,
        total_duration: f64,
    ) -> Result<PathBuf, CliptzyError> {
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

        if self.ctx.config.burn_subtitle {
            let (ass_path, sub_config) = self
                .prepare_subtitles(payload, current_video, idx, out_config)
                .await?;
            ass_path_opt = Some(ass_path);
            sub_config_opt = Some(sub_config);
        }

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

        let subbed_video = self.ctx.job_dir.join(format!("subbed_{}.mp4", idx));
        let watermark_path = if self.ctx.config.burn_watermark {
            self.ctx.config.watermark_image.clone()
        } else {
            None
        };
        let burn_config = crate::processing::burner::VideoBurnerConfig {
            ass_path: ass_path_opt,
            vfx_overlay_path: None,
            normalize_audio: true,
            config: sub_config_opt,
            watermark_path,
            watermark_position: self.ctx.config.watermark_position.clone(),
            hw_accel: hw_accel.clone(),
            debug_ass_path: None,
        };
        crate::processing::burner::burn_video_effects(
            current_video,
            &subbed_video,
            &burn_config,
            Some((&self.ctx.app_handle, total_duration)),
        )
        .await?;

        Ok(subbed_video)
    }

    async fn prepare_subtitles(
        &self,
        payload: &ClipPayload,
        current_video: &Path,
        idx: u32,
        out_config: &OutputConfig,
    ) -> Result<(String, SubtitleConfig), CliptzyError> {
        let whisper_model = if self.ctx.config.subtitle.whisper_model.is_empty() {
            "tiny".to_string()
        } else {
            self.ctx.config.subtitle.whisper_model.clone()
        };
        let transcript_cache_path = cache_file(
            &self.ctx.job_dir,
            &format!(
                "transcript_{}_{}.json",
                idx,
                sanitize_cache_token(&whisper_model)
            ),
        );
        let ass_path = self.ctx.job_dir.join(format!("subtitles_{}.ass", idx));

        let transcript = self
            .load_or_transcribe_segment(
                payload,
                current_video,
                idx,
                &whisper_model,
                &transcript_cache_path,
            )
            .await?;

        if ass_path.exists() {
            log::info!("Menggunakan subtitle ASS dari cache: {:?}", ass_path);
            emit_progress(
                &self.ctx.app_handle,
                &ProgressEvent {
                    stage: "subtitle".into(),
                    label: "Menggunakan subtitle dari cache...".into(),
                    current: 75,
                    total: 100,
                    detail: None,
                },
            );
        } else {
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
            let sub_config = crate::transcription::ass_writer::build_render_config(
                &self.ctx.config.subtitle,
                out_config.height,
            );
            crate::transcription::ass_writer::generate_ass_file(
                &transcript,
                &ass_path,
                &sub_config,
                (out_config.width, out_config.height),
            )?;
            return Ok((ass_path.to_string_lossy().to_string(), sub_config));
        }

        let sub_config = crate::transcription::ass_writer::build_render_config(
            &self.ctx.config.subtitle,
            out_config.height,
        );
        Ok((ass_path.to_string_lossy().to_string(), sub_config))
    }

    async fn load_or_transcribe_segment(
        &self,
        payload: &ClipPayload,
        current_video: &Path,
        idx: u32,
        whisper_model: &str,
        transcript_cache_path: &Path,
    ) -> Result<Vec<crate::transcription::models::TranscriptionSegment>, CliptzyError> {
        if let (Some(cached), Some(_)) = (
            read_json_cache::<SegmentTranscriptCacheEntry>(transcript_cache_path),
            fingerprint(current_video),
        ) {
            if cached.whisper_model == whisper_model
                && is_fingerprint_valid(&cached.source_fingerprint, current_video)
            {
                log::info!(
                    "Menggunakan transkripsi segmen dari cache: {:?}",
                    transcript_cache_path
                );
                emit_progress(
                    &self.ctx.app_handle,
                    &ProgressEvent {
                        stage: "subtitle".into(),
                        label: "Menggunakan transkripsi dari cache...".into(),
                        current: 68,
                        total: 100,
                        detail: None,
                    },
                );
                return Ok(cached.segments);
            }
        }

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

        let audio_wav = self.ctx.job_dir.join(format!("audio_16k_{}.wav", idx));
        let duration = payload.end - payload.start;

        if !audio_wav.exists() {
            crate::transcription::audio::extract_audio_segment(
                &current_video.to_string_lossy(),
                0.0,
                duration,
                &audio_wav,
                None,
                &self.ctx.deps.ytdlp,
            )
            .await?;
        } else {
            log::info!("Menggunakan audio segmen dari cache: {:?}", audio_wav);
        }

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

        let model_path = crate::transcription::whisper::ensure_model_exists(whisper_model).await?;
        let transcriber = crate::transcription::whisper::WhisperTranscriber::new(&model_path)?;

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
        let segments = transcriber.transcribe(&audio_wav).await?;

        if let Some(source_fp) = fingerprint(current_video) {
            let _ = write_json_cache(
                transcript_cache_path,
                &SegmentTranscriptCacheEntry {
                    whisper_model: whisper_model.to_string(),
                    source_fingerprint: source_fp,
                    segments: segments.clone(),
                },
            );
        }

        Ok(segments)
    }
}
