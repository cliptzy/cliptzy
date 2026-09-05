use super::models::{ClipPayload, EmotionCacheEntry};
use super::ClipVideoUseCase;
use crate::analysis::fusion::EmotionFusion;
use crate::error::CliptzyError;
use crate::orchestrator::job_cache::{
    cache_file, fingerprint, is_fingerprint_valid, read_json_cache, sanitize_cache_token,
    write_json_cache,
};
use crate::orchestrator::pipeline::{emit_progress, ProgressEvent};
use std::path::Path;

impl ClipVideoUseCase {
    pub(super) async fn emotion_phase(
        &self,
        payload: &ClipPayload,
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

        // Gunakan audio 16kHz mono standar yang juga dipakai pada fase transkripsi
        let audio_wav_path = self.ctx.job_dir.join(format!("audio_16k_{}.wav", idx));
        if !audio_wav_path.exists() {
            emit_progress(
                &self.ctx.app_handle,
                &ProgressEvent {
                    stage: "analyze".into(),
                    label: "Mengekstrak audio 16kHz untuk analisis AI...".into(),
                    current: 25,
                    total: 100,
                    detail: None,
                },
            );

            let duration = payload.end - payload.start;
            crate::transcription::audio::extract_audio_segment(
                &source_video.to_string_lossy(),
                0.0,
                duration,
                &audio_wav_path,
                None,
                &self.ctx.deps.ytdlp,
            )
            .await?;
        }

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

        // Jika analisis teks atau subtitle aktif, pastikan transkrip Whisper tersedia untuk Text Emotion Analyzer
        if ai.use_text_analysis || self.ctx.config.burn_subtitle {
            if let Err(e) = self
                .load_or_transcribe_segment(
                    payload,
                    source_video,
                    idx,
                    &whisper_model,
                    &transcript_cache_path,
                )
                .await
            {
                log::warn!("Transkripsi awal untuk analisis emosi teks gagal: {}", e);
            }
        }

        emit_progress(
            &self.ctx.app_handle,
            &ProgressEvent {
                stage: "analyze".into(),
                label: "Menjalankan Multi-Modal Emotion Fusion (Visual, Voice, Audio, Text)...".into(),
                current: 30,
                total: 100,
                detail: None,
            },
        );

        let fusion = EmotionFusion::new();
        match fusion
            .analyze_fusion(
                source_video,
                &audio_wav_path,
                &transcript_cache_path,
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
                            visual: timeline.visual.clone(),
                            audio: timeline.audio.clone(),
                            voice: timeline.voice.clone(),
                            text: timeline.text.clone(),
                            scheduled_effects: timeline.scheduled_effects.clone(),
                            scheduled_builtin_effects: timeline.scheduled_builtin_effects.clone(),
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
