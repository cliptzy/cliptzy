use super::models::EmotionCacheEntry;
use super::ClipVideoUseCase;
use crate::analysis::fusion::EmotionFusion;
use crate::error::CliptzyError;
use crate::orchestrator::job_cache::{
    cache_file, fingerprint, is_fingerprint_valid, read_json_cache, write_json_cache,
};
use crate::orchestrator::pipeline::{emit_progress, ProgressEvent};
use std::path::Path;

impl ClipVideoUseCase {
    pub(super) async fn emotion_phase(
        &self,
        source_video: &Path,
        idx: u32,
    ) -> Result<(), CliptzyError> {
        let ai = &self.ctx.config.ai;
        let any_enabled = ai.use_emotion_detection
            || ai.use_audio_analysis
            || ai.use_voice_analysis
            || ai.use_text_analysis;

        if !any_enabled {
            return Ok(());
        }

        let emotion_cache_path = cache_file(&self.ctx.job_dir, &format!("emotions_{}.json", idx));
        let mut use_cached_emotions = false;

        if source_video.exists() {
            if let (Some(cached), Some(_)) = (
                read_json_cache::<EmotionCacheEntry>(&emotion_cache_path),
                fingerprint(source_video),
            ) {
                if is_fingerprint_valid(&cached.source_fingerprint, source_video) {
                    log::info!(
                        "Menggunakan analisis emosi dari cache: {:?}",
                        emotion_cache_path
                    );
                    use_cached_emotions = true;
                }
            }
        }

        if use_cached_emotions {
            return Ok(());
        }

        emit_progress(
            &self.ctx.app_handle,
            &ProgressEvent {
                stage: "analyze".into(),
                label: "Mengekstrak audio untuk analisis multi-modal...".into(),
                current: 25,
                total: 100,
                detail: None,
            },
        );

        // Extract WAV for audio analysis
        let audio_wav_path = self.ctx.job_dir.join(format!("audio_{}.wav", idx));
        if !audio_wav_path.exists() {
            // Using ffmpeg to extract wav
            let status = tokio::process::Command::new("ffmpeg")
                .args(&[
                    "-y",
                    "-i",
                    source_video.to_string_lossy().as_ref(),
                    "-vn",
                    "-acodec",
                    "pcm_s16le",
                    "-ar",
                    "16000",
                    "-ac",
                    "1",
                    audio_wav_path.to_string_lossy().as_ref(),
                ])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()
                .await
                .map_err(|e| CliptzyError::Internal(format!("Failed to spawn ffmpeg: {}", e)))?;

            if !status.success() {
                log::warn!("Failed to extract audio for emotion analysis");
            }
        }

        // Ideally transcript is created before this. For now, we look for it if it exists.
        // It might be generated in subtitle phase, so text sentiment could fail gracefully.
        let transcript_path = self
            .ctx
            .job_dir
            .join(format!("transcript_{}_tiny.json", idx));

        let fusion = EmotionFusion::new();
        match fusion
            .analyze_fusion(
                source_video,
                &audio_wav_path,
                &transcript_path,
                ai,
                &self.ctx.cancel_token,
                &self.ctx.progress_tx,
            )
            .await
        {
            Ok(timeline) => {
                if let Some(source_fp) = fingerprint(source_video) {
                    let _ = write_json_cache(
                        &emotion_cache_path,
                        &EmotionCacheEntry {
                            source_fingerprint: source_fp,
                            segments: timeline.segments.clone(),
                        },
                    );
                }
                log::info!(
                    "Emotion Fusion selesai, tersimpan di {:?}",
                    emotion_cache_path
                );
            }
            Err(e) => {
                log::warn!("Emotion Fusion gagal (dilewati): {}", e);
            }
        }

        Ok(())
    }
}
