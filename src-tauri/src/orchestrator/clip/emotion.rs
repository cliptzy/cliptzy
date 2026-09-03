use super::models::EmotionCacheEntry;
use super::ClipVideoUseCase;
use crate::analysis::EmotionAnalyzer;
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
        if !self.ctx.config.ai.use_emotion_detection {
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
                label: "Menganalisa emosi visual wajah (ONNX)...".into(),
                current: 30,
                total: 100,
                detail: None,
            },
        );

        let analyzer = crate::analysis::visual::VisualEmotionAnalyzer::new();
        match analyzer
            .analyze(source_video, &self.ctx.cancel_token, &self.ctx.progress_tx)
            .await
        {
            Ok(segments) => {
                if let Some(source_fp) = fingerprint(source_video) {
                    let _ = write_json_cache(
                        &emotion_cache_path,
                        &EmotionCacheEntry {
                            source_fingerprint: source_fp,
                            segments: segments.clone(),
                        },
                    );
                }
                log::info!(
                    "Emotion analysis selesai, tersimpan di {:?}",
                    emotion_cache_path
                );
            }
            Err(e) => {
                log::warn!("Visual Emotion Analyzer gagal (dilewati): {}", e);
            }
        }

        Ok(())
    }
}
