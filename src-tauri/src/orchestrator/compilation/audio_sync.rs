use super::audio_extraction::ensure_main_audio_segments_cached;
use super::models::{moments_hash, EpicMoment, RestreamerClip, SyncCacheEntry};
use crate::error::CliptzyError;
use crate::orchestrator::job_cache::{
    fingerprint, is_fingerprint_valid, read_json_cache, write_json_cache,
};
use crate::orchestrator::pipeline::PipelineContext;
use crate::orchestrator::restreamer_cache::{
    migrate_legacy_job_assets, restreamer_audio_m4a, restreamer_audio_wav,
    restreamer_fingerprint_bin, restreamer_sync_cache_path,
};

pub async fn sync_restreamer_audio(
    ctx: &PipelineContext,
    main_audio_path: String,
    restreamer_url: String,
    moments: Vec<EpicMoment>,
) -> Result<Vec<RestreamerClip>, CliptzyError> {
    log::info!(
        "Memulai Sinkronisasi Audio Fingerprinting (Phase 5) untuk {}",
        restreamer_url
    );

    let cookies_path = ctx.config.browser.as_deref().map(|s| s.to_string());
    let job_dir = &ctx.job_dir;
    let main_video_id = &ctx.video_id;

    let restreamer_id = super::models::extract_youtube_video_id(&restreamer_url);
    migrate_legacy_job_assets(job_dir, &restreamer_id);

    let restr_m4a = restreamer_audio_m4a(&restreamer_id);
    let restr_wav = restreamer_audio_wav(&restreamer_id);
    let current_moments_hash = moments_hash(&moments);
    let sync_cache_path =
        restreamer_sync_cache_path(&restreamer_id, main_video_id, &current_moments_hash);

    if restr_wav.exists() {
        if let Some(cached) = read_json_cache::<SyncCacheEntry>(&sync_cache_path) {
            if cached.restreamer_id == restreamer_id
                && cached.moments_hash == current_moments_hash
                && is_fingerprint_valid(&cached.restr_audio_fingerprint, &restr_wav)
                && !cached.clips.is_empty()
            {
                log::info!(
                    "Menggunakan hasil sinkronisasi dari cache global ({} klip): {:?}",
                    cached.clips.len(),
                    sync_cache_path
                );
                return Ok(cached.clips);
            }
        }

        log::info!(
            "Cache sinkronisasi tidak cocok untuk video {} — hanya mencocokkan momen baru (fingerprint DB tetap dipakai dari cache global).",
            main_video_id
        );
    }

    if let Some(parent) = restr_m4a.parent() {
        std::fs::create_dir_all(parent)?;
    }

    if !restr_wav.exists() {
        if !restr_m4a.exists() {
            log::info!("Mengunduh audio restreamer ke {:?}", restr_m4a);
            let mut cmd = tokio::process::Command::new(&ctx.deps.ytdlp);
            cmd.arg("-f")
                .arg("bestaudio[ext=m4a]/bestaudio")
                .arg("-o")
                .arg(restr_m4a.to_string_lossy().to_string())
                .arg("--extractor-args")
                .arg("youtube:player-client=android,web,default")
                .arg("--remote-components")
                .arg("ejs:github");

            if let Some(browser) = &cookies_path {
                if !browser.is_empty() {
                    cmd.arg("--cookies-from-browser").arg(browser);
                }
            }
            cmd.arg(&restreamer_url);

            let mut stage =
                crate::processing::ffmpeg::runner::PipelineStage::new("yt-dlp Restreamer", cmd);
            stage.execute(ctx.cancel_token.clone()).await.map_err(|e| {
                log::error!(
                    "[Compilation] Gagal unduh audio restreamer {}: {}",
                    restreamer_url,
                    e
                );
                e
            })?;
        }

        log::info!("Meresample audio restreamer ke {:?}", restr_wav);
        let mut cmd = tokio::process::Command::new(&ctx.deps.ffmpeg);
        cmd.arg("-i")
            .arg(restr_m4a.to_string_lossy().to_string())
            .arg("-ar")
            .arg("16000")
            .arg("-ac")
            .arg("1")
            .arg("-c:a")
            .arg("pcm_s16le")
            .arg("-y")
            .arg(restr_wav.to_string_lossy().to_string());

        let mut stage = crate::processing::ffmpeg::runner::PipelineStage::new(
            "FFmpeg Resample Restreamer",
            cmd,
        );
        stage.execute(ctx.cancel_token.clone()).await.map_err(|e| {
            log::error!(
                "[Compilation] Gagal resample audio restreamer {}: {}",
                restreamer_url,
                e
            );
            e
        })?;
    } else {
        log::info!(
            "Menggunakan audio restreamer dari cache global: {:?}",
            restr_wav
        );
    }

    log::info!("Mencocokkan momen epik via audio fingerprinting (background thread)...");

    let prepared_segments = ensure_main_audio_segments_cached(job_dir, &main_audio_path, &moments)?;

    if prepared_segments.is_empty() {
        return Err(CliptzyError::Config(
            "Tidak ada segmen audio utama yang valid untuk sinkronisasi.".into(),
        ));
    }

    let restr_wav_str = restr_wav.to_string_lossy().to_string();
    let fingerprint_cache_path = restreamer_fingerprint_bin(&restreamer_id);
    let url_clone = restreamer_url.clone();

    let restreamer_clips =
        tokio::task::spawn_blocking(move || -> Result<Vec<RestreamerClip>, String> {
            let restr_rate = hound::WavReader::open(&restr_wav_str)
                .map_err(|e| e.to_string())?
                .spec()
                .sample_rate;

            let fingerprint_db = crate::orchestrator::audio_fingerprint::build_or_load_fingerprint_db(
                std::path::Path::new(&restr_wav_str),
                &fingerprint_cache_path,
            )?;
            log::info!(
                "Fingerprint database siap: {} hash, {} sampel preprocessed.",
                fingerprint_db.hash_count(),
                fingerprint_db.preprocessed_len()
            );

            let mut results = Vec::new();

            for segment in prepared_segments {
                let moment = segment.moment;
                let (moment_samples, moment_rate) =
                    crate::transcription::audio::decode_wav(&segment.wav_path)?;

                if moment_rate != restr_rate {
                    log::warn!(
                        "Sample rate segmen '{}' ({}) tidak cocok dengan restreamer ({}), melompat...",
                        moment.description,
                        moment_rate,
                        restr_rate
                    );
                    continue;
                }

                let Some(match_result) =
                    crate::orchestrator::audio_fingerprint::find_match_in_db(
                        &fingerprint_db,
                        &moment_samples,
                        restr_rate,
                    )
                else {
                    log::warn!(
                        "Tidak ditemukan kecocokan audio untuk momen [{}] (fingerprint + envelope), melompat...",
                        moment.description
                    );
                    continue;
                };

                let matched_start_time = match_result.start_time_secs;
                let moment_duration = moment.end - moment.start;
                let matched_end_time = matched_start_time + moment_duration;
                let offset_diff = matched_start_time - moment.start;

                log::info!(
                    "Moment [{}] cocok di {:.2}s via {} (skor: {}, selisih: {:.2}s)",
                    moment.description,
                    matched_start_time,
                    match_result.method,
                    match_result.score,
                    offset_diff
                );

                results.push(RestreamerClip {
                    restreamer_url: url_clone.clone(),
                    offset: offset_diff,
                    start: matched_start_time,
                    end: matched_end_time,
                    description: moment.description,
                });
            }

            Ok(results)
        })
        .await
        .map_err(|e| {
            log::error!("[Compilation] spawn_blocking panic: {}", e);
            CliptzyError::Internal(format!("Panic spawn_blocking: {}", e))
        })?
        .map_err(|e| {
            log::error!("[Compilation] Audio fingerprinting gagal: {}", e);
            CliptzyError::Internal(e)
        })?;

    log::info!(
        "Berhasil mensinkronisasikan {} momen dengan VOD restreamer.",
        restreamer_clips.len()
    );

    if let Some(restr_fp) = fingerprint(&restr_wav) {
        write_json_cache(
            &sync_cache_path,
            &SyncCacheEntry {
                restreamer_id: restreamer_id.clone(),
                restr_audio_fingerprint: restr_fp,
                moments_hash: current_moments_hash,
                clips: restreamer_clips.clone(),
            },
        )?;
    }

    Ok(restreamer_clips)
}
