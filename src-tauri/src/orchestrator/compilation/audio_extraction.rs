use super::helpers::emit_stage;
use super::models::{
    EpicMoment, MainAudioExtractionResult, MainSegmentFile, MainSegmentsCacheEntry,
    PreparedMainSegment, VideoInfoCacheEntry,
};
use crate::error::CliptzyError;
use crate::orchestrator::job_cache::{cache_file, fingerprint, read_json_cache, write_json_cache};
use crate::orchestrator::pipeline::PipelineContext;

pub(crate) fn ensure_main_audio_segments_cached(
    job_dir: &std::path::Path,
    main_audio_path: &str,
    moments: &[EpicMoment],
) -> Result<Vec<PreparedMainSegment>, CliptzyError> {
    let audio_file = std::path::Path::new(main_audio_path);
    let source_fp = fingerprint(audio_file).ok_or_else(|| {
        CliptzyError::FileNotFound(format!("File audio tidak ditemukan: {}", main_audio_path))
    })?;
    let current_hash = super::models::moments_hash(moments);
    let cache_path = cache_file(job_dir, "main_segments.json");
    let segments_dir = cache_file(job_dir, "main_segments");

    if let Some(cached) = read_json_cache::<MainSegmentsCacheEntry>(&cache_path) {
        if cached.main_audio_fingerprint == source_fp
            && cached.moments_hash == current_hash
            && !cached.segments.is_empty()
            && cached
                .segments
                .iter()
                .all(|s| std::path::Path::new(&s.wav_path).exists())
        {
            log::info!(
                "Menggunakan segmen audio utama dari cache ({} segmen): {:?}",
                cached.segments.len(),
                cache_path
            );
            return Ok(cached
                .segments
                .into_iter()
                .map(|seg| PreparedMainSegment {
                    wav_path: seg.wav_path,
                    moment: EpicMoment {
                        start: seg.start,
                        end: seg.end,
                        description: seg.description,
                    },
                })
                .collect());
        }

        log::info!("Cache segmen audio utama tidak valid, mengekstrak ulang...");
    }

    std::fs::create_dir_all(&segments_dir)?;

    let (main_samples, sample_rate) =
        crate::transcription::audio::decode_wav(main_audio_path).map_err(CliptzyError::Internal)?;

    let mut segment_files = Vec::new();
    let mut prepared = Vec::new();

    for (index, moment) in moments.iter().enumerate() {
        let start_sample = (moment.start * sample_rate as f64) as usize;
        let end_sample = (moment.end * sample_rate as f64) as usize;

        if start_sample >= main_samples.len() {
            continue;
        }
        let end_sample_safe = end_sample.min(main_samples.len());
        let slice = &main_samples[start_sample..end_sample_safe];

        if slice.len() < crate::orchestrator::audio_fingerprint::WINDOW_SIZE {
            log::warn!(
                "Momen '{}' terlalu pendek untuk diekstrak ke cache ({} sampel), dilewati.",
                moment.description,
                slice.len()
            );
            continue;
        }

        let wav_path = segments_dir.join(format!("segment_{:03}.wav", index));
        crate::transcription::audio::write_wav_segment(&wav_path, slice, sample_rate)
            .map_err(CliptzyError::Internal)?;

        let wav_path_str = wav_path.to_string_lossy().to_string();
        segment_files.push(MainSegmentFile {
            index,
            start: moment.start,
            end: moment.end,
            description: moment.description.clone(),
            wav_path: wav_path_str.clone(),
        });
        prepared.push(PreparedMainSegment {
            wav_path: wav_path_str,
            moment: moment.clone(),
        });
    }

    write_json_cache(
        &cache_path,
        &MainSegmentsCacheEntry {
            main_audio_fingerprint: source_fp,
            moments_hash: current_hash,
            segments: segment_files,
        },
    )?;

    log::info!(
        "Mengekstrak {} segmen audio utama ke cache.",
        prepared.len()
    );
    Ok(prepared)
}

pub async fn extract_main_audio(
    ctx: &PipelineContext,
    video_url: String,
) -> Result<MainAudioExtractionResult, CliptzyError> {
    let video_id = ctx.video_id.clone();
    log::info!("Memulai Ekstraksi Audio Utama (Phase 2) untuk {}", video_id);

    emit_stage(
        ctx,
        "download",
        "Menganalisis metadata video utama...",
        5,
        100,
    );

    let cookies_path = ctx.config.browser.as_deref().map(|s| s.to_string());
    let job_dir = &ctx.job_dir;
    if !job_dir.exists() {
        std::fs::create_dir_all(job_dir)?;
    }

    let video_info_cache_path = cache_file(job_dir, "video_info.json");
    let video_info =
        if let Some(cached) = read_json_cache::<VideoInfoCacheEntry>(&video_info_cache_path) {
            if cached.video_id == video_id {
                if cached.info.upload_date.is_none() || cached.info.video_url.is_empty() {
                    log::info!(
                        "Cache metadata belum lengkap (upload_date/video_url), mengambil ulang..."
                    );
                    fetch_and_cache_video_info(
                        &video_url,
                        cookies_path.clone(),
                        ctx,
                        &video_info_cache_path,
                        &video_id,
                    )
                    .await?
                } else {
                    log::info!(
                        "Menggunakan metadata video dari cache: {:?}",
                        video_info_cache_path
                    );
                    emit_stage(
                        ctx,
                        "download",
                        "Menggunakan metadata video dari cache...",
                        8,
                        100,
                    );
                    cached.info
                }
            } else {
                log::info!("Cache metadata tidak cocok (video_id berbeda), mengambil ulang...");
                fetch_and_cache_video_info(
                    &video_url,
                    cookies_path.clone(),
                    ctx,
                    &video_info_cache_path,
                    &video_id,
                )
                .await?
            }
        } else {
            fetch_and_cache_video_info(
                &video_url,
                cookies_path.clone(),
                ctx,
                &video_info_cache_path,
                &video_id,
            )
            .await?
        };

    let m4a_path = job_dir.join("main_audio.m4a");
    let wav_16k_path = job_dir.join("main_audio_16k.wav");

    if !m4a_path.exists() {
        log::info!("Mengunduh audio asli ke {:?}", m4a_path);
        emit_stage(
            ctx,
            "download",
            "Mengunduh audio utama via yt-dlp...",
            15,
            100,
        );

        let mut cmd = tokio::process::Command::new(&ctx.deps.ytdlp);
        cmd.arg("-f")
            .arg("bestaudio[ext=m4a]/bestaudio")
            .arg("-o")
            .arg(m4a_path.to_string_lossy().to_string())
            .arg("--extractor-args")
            .arg("youtube:player-client=android,web,default")
            .arg("--remote-components")
            .arg("ejs:github");

        if let Some(browser) = &cookies_path {
            if !browser.is_empty() {
                cmd.arg("--cookies-from-browser").arg(browser);
            }
        }
        cmd.arg(&video_url);

        let mut stage =
            crate::processing::ffmpeg::runner::PipelineStage::new("yt-dlp Audio Utama", cmd);
        stage.execute(ctx.cancel_token.clone()).await.map_err(|e| {
            log::error!("[Compilation] Gagal unduh audio utama: {}", e);
            e
        })?;
    } else {
        log::info!("Audio asli sudah ada di cache: {:?}", m4a_path);
        emit_stage(
            ctx,
            "download",
            "Menggunakan audio utama dari cache...",
            20,
            100,
        );
    }

    if !wav_16k_path.exists() {
        log::info!("Meresample audio ke 16kHz mono: {:?}", wav_16k_path);
        emit_stage(
            ctx,
            "transcode",
            "Meresample audio ke 16kHz mono (Whisper)...",
            25,
            100,
        );

        let mut cmd = tokio::process::Command::new(&ctx.deps.ffmpeg);
        cmd.arg("-i")
            .arg(m4a_path.to_string_lossy().to_string())
            .arg("-ar")
            .arg("16000")
            .arg("-ac")
            .arg("1")
            .arg("-c:a")
            .arg("pcm_s16le")
            .arg("-y")
            .arg(wav_16k_path.to_string_lossy().to_string());

        let mut stage =
            crate::processing::ffmpeg::runner::PipelineStage::new("FFmpeg Resample 16k", cmd);
        stage.execute(ctx.cancel_token.clone()).await.map_err(|e| {
            log::error!("[Compilation] Gagal resample audio 16kHz: {}", e);
            e
        })?;
    } else {
        log::info!("Audio 16kHz sudah ada di cache: {:?}", wav_16k_path);
    }

    emit_stage(ctx, "download", "Ekstraksi audio utama selesai.", 30, 100);

    Ok(MainAudioExtractionResult {
        video_info,
        main_audio_16k_path: wav_16k_path.to_string_lossy().to_string(),
    })
}

async fn fetch_and_cache_video_info(
    video_url: &str,
    cookies_path: Option<String>,
    ctx: &PipelineContext,
    cache_path: &std::path::Path,
    video_id: &str,
) -> Result<crate::video::youtube::VideoAnalysisResult, CliptzyError> {
    let info =
        crate::video::youtube::analyze_youtube_video(video_url, cookies_path, &ctx.deps.ytdlp)
            .await
            .map_err(|e| {
                log::error!("[Compilation] Gagal analisis metadata YouTube: {}", e);
                CliptzyError::Download(e)
            })?;

    let _ = write_json_cache(
        cache_path,
        &VideoInfoCacheEntry {
            video_id: video_id.to_string(),
            info: info.clone(),
        },
    );

    Ok(info)
}
