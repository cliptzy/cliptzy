mod audio_extraction;
mod audio_sync;
mod clipping;
mod helpers;
mod models;
mod moment_detection;
mod restreamer_search;

pub use audio_extraction::extract_main_audio;
pub use audio_sync::sync_restreamer_audio;
pub use clipping::clip_and_label_restreamers;
pub use models::{
    EpicMoment, MainAudioExtractionResult, PrepareCompilationResult, RestreamerClip,
    RestreamerInfo,
};
pub use moment_detection::detect_epic_moments;
pub use restreamer_search::search_restreamers;

use crate::error::CliptzyError;
use crate::orchestrator::pipeline::PipelineContext;
use helpers::emit_stage;

pub struct PrepareCompilationUseCase {
    ctx: PipelineContext,
}

impl PrepareCompilationUseCase {
    pub fn new(ctx: PipelineContext) -> Self {
        Self { ctx }
    }

    pub async fn execute(
        &mut self,
        video_url: String,
        search_keywords: Option<String>,
    ) -> Result<PrepareCompilationResult, CliptzyError> {
        let video_id = self.ctx.video_id.clone();
        log::info!(
            "PrepareCompilationUseCase: memulai persiapan kompilasi untuk {}",
            video_id
        );

        emit_stage(&self.ctx, "init", "Memulai persiapan kompilasi...", 1, 100);

        let audio_res = extract_main_audio(&self.ctx, video_url)
            .await
            .map_err(|e| {
                log::error!(
                    "[PrepareCompilation] Gagal fase ekstraksi audio untuk {}: {}",
                    video_id,
                    e
                );
                e
            })?;

        let moments = detect_epic_moments(
            &self.ctx,
            audio_res.main_audio_16k_path.clone(),
            &audio_res.video_info,
        )
        .await
        .map_err(|e| {
            log::error!(
                "[PrepareCompilation] Gagal fase deteksi momen untuk {}: {}",
                video_id,
                e
            );
            e
        })?;

        let restreamers = search_restreamers(
            &self.ctx,
            &audio_res.video_info,
            search_keywords,
            Some(60),
        )
        .await
        .map_err(|e| {
            log::error!(
                "[PrepareCompilation] Gagal fase pencarian restreamer untuk {}: {}",
                video_id,
                e
            );
            e
        })?;

        emit_stage(&self.ctx, "done", "Persiapan kompilasi selesai!", 100, 100);

        log::info!(
            "PrepareCompilationUseCase selesai untuk {} — {} momen, {} restreamer.",
            video_id,
            moments.len(),
            restreamers.len()
        );

        Ok(PrepareCompilationResult {
            video_info: audio_res.video_info,
            main_audio_16k_path: audio_res.main_audio_16k_path,
            epic_moments: moments,
            restreamers,
        })
    }
}

pub struct ExecuteCompilationUseCase {
    ctx: PipelineContext,
}

impl ExecuteCompilationUseCase {
    pub fn new(ctx: PipelineContext) -> Self {
        Self { ctx }
    }

    pub async fn execute(
        &mut self,
        main_audio_path: String,
        restreamer_urls: Vec<String>,
        moments: Vec<EpicMoment>,
        output_filename: String,
    ) -> Result<String, CliptzyError> {
        let video_id = self.ctx.video_id.clone();
        log::info!(
            "ExecuteCompilationUseCase: memulai eksekusi kompilasi untuk {}",
            video_id
        );

        emit_stage(&self.ctx, "init", "Memulai rendering kompilasi...", 1, 100);

        let total_restr = restreamer_urls.len().max(1);
        let mut all_clips = Vec::new();

        for (i, url) in restreamer_urls.into_iter().enumerate() {
            let pct = 10 + (i as u32 * 30 / total_restr as u32);
            emit_stage(
                &self.ctx,
                "sync",
                &format!("Sinkronisasi restreamer {}/{}...", i + 1, total_restr),
                pct,
                100,
            );

            match sync_restreamer_audio(
                &self.ctx,
                main_audio_path.clone(),
                url.clone(),
                moments.clone(),
            )
            .await
            {
                Ok(mut clips) => {
                    log::info!(
                        "Sinkronisasi berhasil untuk {} — {} klip.",
                        url,
                        clips.len()
                    );
                    all_clips.append(&mut clips);
                }
                Err(e) => {
                    log::error!(
                        "[ExecuteCompilation] Gagal sinkronisasi restreamer {}: {}",
                        url,
                        e
                    );
                }
            }
        }

        if all_clips.is_empty() {
            let msg = "Tidak ada klip restreamer yang berhasil disinkronisasi".to_string();
            log::error!("[ExecuteCompilation] {}", msg);
            return Err(CliptzyError::Config(msg));
        }

        emit_stage(
            &self.ctx,
            "clip",
            "Memotong dan melabeli klip restreamer...",
            45,
            100,
        );

        let clip_paths = clip_and_label_restreamers(&self.ctx, all_clips.clone())
            .await
            .map_err(|e| {
                log::error!(
                    "[ExecuteCompilation] Gagal fase clipping untuk {}: {}",
                    video_id,
                    e
                );
                e
            })?;

        if clip_paths.is_empty() {
            let msg = "Tidak ada klip yang berhasil dipotong".to_string();
            log::error!("[ExecuteCompilation] {}", msg);
            return Err(CliptzyError::Config(msg));
        }

        emit_stage(
            &self.ctx,
            "concat",
            "Menggabungkan video kompilasi...",
            75,
            100,
        );

        let sequences = vec![crate::orchestrator::compile::CompilationSequence {
            main_moment_path: String::new(),
            reaction_paths: clip_paths,
        }];

        let hwaccel =
            crate::processing::ffmpeg::hwaccel::HwAccel::detect(Some(&self.ctx.config.hw_accel));
        let deps = crate::deps::AppDependencies {
            ytdlp: self.ctx.deps.ytdlp.clone(),
            ffmpeg: self.ctx.deps.ffmpeg.clone(),
        };
        let mut use_case = crate::orchestrator::compile::CompileVideoUseCase::new(
            self.ctx.job_dir.clone(),
            hwaccel,
            deps,
        );

        let result = use_case
            .execute(sequences, &output_filename, self.ctx.cancel_token.clone())
            .await
            .map_err(|e| {
                log::error!(
                    "[ExecuteCompilation] Gagal fase concat untuk {}: {}",
                    video_id,
                    e
                );
                e
            })?;

        emit_stage(&self.ctx, "done", "Kompilasi selesai!", 100, 100);

        log::info!(
            "ExecuteCompilationUseCase selesai untuk {} — output: {}",
            video_id,
            result.output_path
        );

        let _ = tauri_plugin_opener::reveal_item_in_dir(result.output_path.clone());

        Ok(result.output_path)
    }
}
