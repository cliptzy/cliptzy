use crate::config::models::AIConfig;
use crate::error::CliptzyError;
use crate::orchestrator::job_cache::{
    self, ai_model_name, cache_file, fingerprint, hash_payload, is_fingerprint_valid,
    read_json_cache, write_json_cache, FileFingerprint,
};
use crate::orchestrator::pipeline::{emit_progress, PipelineContext, ProgressEvent};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MainAudioExtractionResult {
    pub video_info: crate::video::youtube::VideoAnalysisResult,
    pub main_audio_16k_path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EpicMoment {
    pub start: f64,
    pub end: f64,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RestreamerClip {
    pub restreamer_url: String,
    pub offset: f64,
    pub start: f64,
    pub end: f64,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RestreamerInfo {
    pub video_id: String,
    pub video_url: String,
    pub title: String,
    pub uploader: String,
    pub thumbnail: String,
    pub duration: f64,
    #[serde(default)]
    pub upload_date: Option<String>,
    #[serde(default)]
    pub view_count: Option<u64>,
}

#[derive(Serialize, Clone, Debug)]
pub struct PrepareCompilationResult {
    pub video_info: crate::video::youtube::VideoAnalysisResult,
    pub main_audio_16k_path: String,
    pub epic_moments: Vec<EpicMoment>,
    pub restreamers: Vec<RestreamerInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct VideoInfoCacheEntry {
    video_id: String,
    info: crate::video::youtube::VideoAnalysisResult,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct TranscriptCacheEntry {
    whisper_model: String,
    source_fingerprint: FileFingerprint,
    segments: Vec<crate::transcription::models::TranscriptionSegment>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct EpicMomentsCacheEntry {
    ai_provider: String,
    ai_model: String,
    transcript_hash: String,
    moments: Vec<EpicMoment>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct RestreamerSearchCacheEntry {
    query: String,
    min_duration_minutes: u32,
    #[serde(default)]
    main_upload_date: Option<String>,
    #[serde(default)]
    restreamers: Vec<RestreamerInfo>,
    /// Legacy cache field (URL-only); migrated on read.
    #[serde(default)]
    urls: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct MainSegmentFile {
    index: usize,
    start: f64,
    end: f64,
    description: String,
    wav_path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct MainSegmentsCacheEntry {
    main_audio_fingerprint: FileFingerprint,
    moments_hash: String,
    segments: Vec<MainSegmentFile>,
}

#[derive(Clone, Debug)]
struct PreparedMainSegment {
    wav_path: String,
    moment: EpicMoment,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SyncCacheEntry {
    restreamer_id: String,
    restr_audio_fingerprint: FileFingerprint,
    moments_hash: String,
    clips: Vec<RestreamerClip>,
}

fn moments_hash(moments: &[EpicMoment]) -> String {
    serde_json::to_string(moments)
        .map(|s| hash_payload(&s))
        .unwrap_or_else(|_| "invalid".to_string())
}

fn extract_youtube_video_id(url: &str) -> String {
    url.split("v=")
        .nth(1)
        .and_then(|s| s.split('&').next())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            url.split('/')
                .last()
                .unwrap_or("unknown")
                .split('?')
                .next()
                .unwrap_or("unknown")
                .to_string()
        })
}

fn migrate_cached_restreamers(cached: &RestreamerSearchCacheEntry) -> Vec<RestreamerInfo> {
    if !cached.restreamers.is_empty() {
        return cached.restreamers.clone();
    }

    cached
        .urls
        .iter()
        .map(|url| {
            let video_id = extract_youtube_video_id(url);
            RestreamerInfo {
                video_id: video_id.clone(),
                video_url: url.clone(),
                title: url.clone(),
                uploader: String::new(),
                thumbnail: format!("https://i.ytimg.com/vi/{}/hqdefault.jpg", video_id),
                duration: 0.0,
                upload_date: None,
                view_count: None,
            }
        })
        .collect()
}

fn parse_restreamer_entry(entry: &serde_json::Value) -> Option<RestreamerInfo> {
    let id = entry.get("id").and_then(|i| i.as_str()).unwrap_or("");
    if id.is_empty() {
        return None;
    }

    let entry_url = entry
        .get("webpage_url")
        .and_then(|u| u.as_str())
        .or_else(|| entry.get("url").and_then(|u| u.as_str()))
        .unwrap_or("");
    let video_url = if entry_url.starts_with("http") {
        entry_url.to_string()
    } else {
        format!("https://www.youtube.com/watch?v={}", id)
    };

    let thumbnail = entry
        .get("thumbnail")
        .and_then(|t| t.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("https://i.ytimg.com/vi/{}/hqdefault.jpg", id));

    Some(RestreamerInfo {
        video_id: id.to_string(),
        video_url,
        title: entry
            .get("title")
            .and_then(|t| t.as_str())
            .unwrap_or("Tanpa Judul")
            .to_string(),
        uploader: entry
            .get("uploader")
            .or_else(|| entry.get("channel"))
            .and_then(|u| u.as_str())
            .unwrap_or("")
            .to_string(),
        thumbnail,
        duration: entry
            .get("duration")
            .and_then(|d| d.as_f64())
            .unwrap_or(0.0),
        upload_date: entry
            .get("upload_date")
            .and_then(|d| d.as_str())
            .map(|s| s.to_string()),
        view_count: entry
            .get("view_count")
            .and_then(|v| v.as_u64()),
    })
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

fn add_days_yyyymmdd(date: &str, days: u32) -> Option<String> {
    if date.len() != 8 {
        return None;
    }
    let mut year: i32 = date[0..4].parse().ok()?;
    let mut month: u32 = date[4..6].parse().ok()?;
    let mut day: u32 = date[6..8].parse().ok()?;
    let mut remaining = days;

    while remaining > 0 {
        let dim = days_in_month(year, month);
        let days_left_in_month = dim.saturating_sub(day);
        if remaining <= days_left_in_month {
            day += remaining;
            remaining = 0;
        } else {
            remaining -= days_left_in_month + 1;
            day = 1;
            if month == 12 {
                month = 1;
                year += 1;
            } else {
                month += 1;
            }
        }
    }

    Some(format!("{:04}{:02}{:02}", year, month, day))
}

fn is_upload_within_reaction_window(entry_date: &str, main_date: &str, max_days_after: u32) -> bool {
    if entry_date.len() != 8 || main_date.len() != 8 {
        return false;
    }
    let max_date = add_days_yyyymmdd(main_date, max_days_after).unwrap_or_else(|| "99999999".to_string());
    entry_date >= main_date && entry_date <= max_date.as_str()
}

fn ensure_main_audio_segments_cached(
    job_dir: &std::path::Path,
    main_audio_path: &str,
    moments: &[EpicMoment],
) -> Result<Vec<PreparedMainSegment>, CliptzyError> {
    let audio_file = std::path::Path::new(main_audio_path);
    let source_fp = fingerprint(audio_file).ok_or_else(|| {
        CliptzyError::FileNotFound(format!("File audio tidak ditemukan: {}", main_audio_path))
    })?;
    let current_hash = moments_hash(moments);
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
        crate::orchestrator::audio_fingerprint::decode_wav(main_audio_path)
            .map_err(CliptzyError::Internal)?;

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
        crate::orchestrator::audio_fingerprint::write_wav_segment(&wav_path, slice, sample_rate)
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

    log::info!("Mengekstrak {} segmen audio utama ke cache.", prepared.len());
    Ok(prepared)
}

fn epic_moments_prompt(
    compilation: &crate::config::models::CompilationConfig,
    transcript: &str,
    video_info: &crate::video::youtube::VideoAnalysisResult,
    chunk_info: &str,
) -> String {
    let metadata_str = format!(
        "Video Title: {}\nVideo ID: {}",
        video_info.title, video_info.video_id
    );

    if compilation.compilation_type == "meme_shorts" {
        format!(
            "Kamu adalah editor meme shorts vertikal. Temukan momen lucu/viral terbaik dari transkrip video berikut.\n\
            Metadata Video:\n{}\n\
            Setiap segmen HARUS antara 15 hingga 180 detik (maksimal 3 menit per segmen).\n\
            Output HANYA dalam format JSON array objek. Contoh:\n\
            [{{\"start\": 12.5, \"end\": 45.0, \"description\": \"Momen lucu\"}}]\n\
            {}\n\
            Transkrip:\n{}",
            metadata_str, chunk_info, transcript
        )
    } else {
        format!(
            "You are a professional esports video editor specializing in highlight reels.\n\
            Your task is to identify the most epic IN-GAME gameplay moments from the provided transcript.\n\
            \n\
            Video Metadata:\n{}\n\
            \n\
            CRITICAL RULES:\n\
            1. ONLY select segments from the ACTUAL MATCH / GAMEPLAY.\n\
            2. STRICTLY IGNORE all non-gameplay segments. You must completely skip: Draft Picks, Hero Bans, caster desk analysis, interviews, pre-game intros, commercial breaks, and post-game celebrations.\n\
            3. Look for high-energy shoutcasting cues.\n\
            4. The duration of each segment CAN exceed 3 minutes if the full context of the team fight or momentum shift requires it. Do not cut the action abruptly; ensure the buildup and aftermath are included.\n\
            \n\
            Output YOUR RESPONSE STRICTLY as a valid JSON array of objects, and absolutely nothing else (no markdown formatting, no explanations). Example format:\n\
            [{{\"start\": 12.5, \"end\": 125.0, \"description\": \"Intense Lord contest leading to a RRQ Wiped Out\"}}]\n\
            \n\
            {}\n\
            Transcript:\n{}",
            metadata_str, chunk_info, transcript
        )
    }
}

fn apply_segment_duration_cap(moments: &mut [EpicMoment], max_secs: u32) {
    if max_secs == 0 {
        return;
    }
    let max = max_secs as f64;
    for moment in moments.iter_mut() {
        let duration = moment.end - moment.start;
        if duration > max {
            log::info!(
                "Memotong momen '{}' dari {:.1}s menjadi {:.1}s (max_segment_duration={}s)",
                moment.description,
                duration,
                max,
                max_secs
            );
            moment.end = moment.start + max;
        }
    }
}

fn emit_stage(ctx: &PipelineContext, stage: &str, label: &str, current: u32, total: u32) {
    emit_progress(
        &ctx.app_handle,
        &ProgressEvent {
            stage: stage.into(),
            label: label.into(),
            current,
            total,
            detail: None,
        },
    );
}

fn validate_ai_config(config: &AIConfig) -> Result<(), CliptzyError> {
    match config.provider.as_str() {
        "gemini" => {
            if config.gemini_key.trim().is_empty() {
                return Err(CliptzyError::Config(
                    "API Key Gemini belum dikonfigurasi. Buka Settings → AI untuk mengatur kunci API."
                        .into(),
                ));
            }
        }
        "openai" => {
            if config.openai_key.trim().is_empty() {
                return Err(CliptzyError::Config(
                    "API Key OpenAI belum dikonfigurasi. Buka Settings → AI untuk mengatur kunci API."
                        .into(),
                ));
            }
        }
        "ollama" => {
            if config.ollama_host.trim().is_empty() {
                return Err(CliptzyError::Config(
                    "Host Ollama belum dikonfigurasi. Buka Settings → AI untuk mengatur host Ollama."
                        .into(),
                ));
            }
        }
        other => {
            if config.gemini_key.trim().is_empty() {
                return Err(CliptzyError::Config(format!(
                    "Provider AI '{}' memerlukan konfigurasi API key yang valid di Settings.",
                    other
                )));
            }
        }
    }
    Ok(())
}

fn extract_json_array(text: &str) -> Result<String, CliptzyError> {
    let text = text.trim();
    if text.is_empty() {
        return Err(CliptzyError::AIProvider(
            "AI mengembalikan respons kosong. Periksa API key, model, dan koneksi jaringan.".into(),
        ));
    }

    if let Some(start) = text.find('[') {
        if let Some(end) = text.rfind(']') {
            if start <= end {
                return Ok(text[start..=end].to_string());
            }
        }
    }

    Err(CliptzyError::AIProvider(format!(
        "Tidak menemukan array JSON valid dalam respons AI. Cuplikan: {}",
        &text[..text.len().min(300)]
    )))
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

async fn load_or_transcribe_main_audio(
    ctx: &PipelineContext,
    audio_path: &str,
    whisper_model: &str,
) -> Result<Vec<crate::transcription::models::TranscriptionSegment>, CliptzyError> {
    let audio_file = std::path::Path::new(audio_path);
    let source_fingerprint = fingerprint(audio_file).ok_or_else(|| {
        CliptzyError::FileNotFound(format!("File audio tidak ditemukan: {}", audio_path))
    })?;

    let cache_path = cache_file(
        &ctx.job_dir,
        &format!(
            "main_transcript_{}.json",
            job_cache::sanitize_cache_token(whisper_model)
        ),
    );

    if let Some(cached) = read_json_cache::<TranscriptCacheEntry>(&cache_path) {
        if cached.whisper_model == whisper_model
            && is_fingerprint_valid(&cached.source_fingerprint, audio_file)
        {
            log::info!(
                "Menggunakan transkripsi dari cache ({} segmen): {:?}",
                cached.segments.len(),
                cache_path
            );
            emit_stage(
                ctx,
                "transcribe",
                &format!(
                    "Menggunakan transkripsi cache ({} segmen)...",
                    cached.segments.len()
                ),
                50,
                100,
            );
            return Ok(cached.segments);
        }

        log::info!(
            "Cache transkripsi tidak valid (model/audio berubah), menjalankan ulang Whisper..."
        );
    }

    log::info!("Memuat model Whisper: {}", whisper_model);
    let model_path = crate::transcription::whisper::ensure_model_exists(whisper_model).await?;
    let transcriber = crate::transcription::whisper::WhisperTranscriber::new(&model_path)?;

    log::info!("Menjalankan transkripsi audio utama...");
    emit_stage(
        ctx,
        "transcribe",
        &format!("Menjalankan transkripsi Whisper ({})...", whisper_model),
        45,
        100,
    );

    let transcript_segments = transcriber.transcribe(audio_file).await.map_err(|e| {
        log::error!("[Compilation] Gagal transkripsi Whisper: {}", e);
        e
    })?;

    write_json_cache(
        &cache_path,
        &TranscriptCacheEntry {
            whisper_model: whisper_model.to_string(),
            source_fingerprint,
            segments: transcript_segments.clone(),
        },
    )?;

    Ok(transcript_segments)
}

pub async fn detect_epic_moments(
    ctx: &PipelineContext,
    audio_path: String,
    video_info: &crate::video::youtube::VideoAnalysisResult,
) -> Result<Vec<EpicMoment>, CliptzyError> {
    let video_id = ctx.video_id.clone();
    log::info!(
        "Memulai Transkripsi & Deteksi Momen (Phase 3) untuk {}",
        video_id
    );

    emit_stage(
        ctx,
        "transcribe",
        "Memuat model Whisper untuk transkripsi...",
        35,
        100,
    );

    let config = &ctx.config;
    let whisper_model = if config.subtitle.whisper_model.is_empty() {
        "tiny".to_string()
    } else {
        config.subtitle.whisper_model.clone()
    };

    let transcript_segments =
        load_or_transcribe_main_audio(ctx, &audio_path, &whisper_model).await?;

    log::info!(
        "Transkripsi selesai. {} segmen ditemukan.",
        transcript_segments.len()
    );

    if transcript_segments.is_empty() {
        return Err(CliptzyError::Transcription(
            "Transkripsi kosong — tidak ada teks yang terdeteksi dari audio.".into(),
        ));
    }

    let is_local_ai = config.ai.provider.to_lowercase() == "ollama";
    let chunk_size = if is_local_ai { 150 } else { 1500 }; // segments per chunk (approx 1500 lines ~ 8000 words)
    let chunks: Vec<Vec<_>> = transcript_segments
        .chunks(chunk_size)
        .map(|c| c.to_vec())
        .collect();

    let full_transcript = transcript_segments
        .iter()
        .map(|s| format!("[{:.2} - {:.2}]: {}", s.start, s.end, s.text))
        .collect::<Vec<_>>()
        .join("\n");
    let transcript_hash = hash_payload(&full_transcript);

    let ai_provider_name = config.ai.provider.clone();
    let ai_model = ai_model_name(&config.ai);
    let moments_cache_path = cache_file(
        &ctx.job_dir,
        &format!(
            "epic_moments_{}_{}.json",
            job_cache::sanitize_cache_token(&ai_provider_name),
            job_cache::sanitize_cache_token(&ai_model)
        ),
    );

    if let Some(cached) = read_json_cache::<EpicMomentsCacheEntry>(&moments_cache_path) {
        if cached.ai_provider == ai_provider_name
            && cached.ai_model == ai_model
            && cached.transcript_hash == transcript_hash
            && !cached.moments.is_empty()
        {
            log::info!(
                "Menggunakan momen epik dari cache ({} momen): {:?}",
                cached.moments.len(),
                moments_cache_path
            );
            emit_stage(
                ctx,
                "ai",
                &format!(
                    "Menggunakan kurasi AI dari cache ({} momen)...",
                    cached.moments.len()
                ),
                70,
                100,
            );
            return Ok(cached.moments);
        }
    }

    validate_ai_config(&config.ai)?;
    log::info!(
        "Mengirim {} chunk transkrip ke modul AI ({}) untuk ekstraksi momen epik...",
        chunks.len(),
        config.ai.provider
    );
    emit_stage(
        ctx,
        "ai",
        &format!("Menganalisis momen epik via AI ({})...", config.ai.provider),
        55,
        100,
    );

    let ai_provider = crate::ai::create_provider(&config.ai);
    let mut all_moments: Vec<EpicMoment> = Vec::new();

    for (idx, chunk) in chunks.iter().enumerate() {
        let mut transcript_text = String::new();
        for seg in chunk {
            transcript_text.push_str(&format!(
                "[{:.2} - {:.2}]: {}\n",
                seg.start, seg.end, seg.text
            ));
        }

        let chunk_info = if chunks.len() > 1 {
            format!("(IMPORTANT: This is part {} of {} of the total transcript. Analyze this part specifically.)", idx + 1, chunks.len())
        } else {
            "".to_string()
        };

        let prompt = epic_moments_prompt(
            &config.compilation,
            &transcript_text,
            video_info,
            &chunk_info,
        );
        log::info!(
            "Memproses chunk AI {}/{} ({} segmen)",
            idx + 1,
            chunks.len(),
            chunk.len()
        );

        match ai_provider.generate(&prompt, Some(&ctx.progress_tx)).await {
            Ok(response) => {
                let json_str = extract_json_array(&response).unwrap_or_else(|_| "[]".to_string());
                match serde_json::from_str::<Vec<EpicMoment>>(&json_str) {
                    Ok(mut parsed_moments) => {
                        apply_segment_duration_cap(
                            &mut parsed_moments,
                            config.compilation.max_segment_duration,
                        );
                        all_moments.extend(parsed_moments);
                    }
                    Err(e) => {
                        log::error!(
                            "[Compilation] Gagal parse JSON AI untuk chunk {}: {} | Raw: {}",
                            idx + 1,
                            e,
                            &response[..response.len().min(500)]
                        );
                    }
                }
            }
            Err(e) => {
                log::error!(
                    "[Compilation] Gagal memanggil AI provider untuk chunk {}: {}",
                    idx + 1,
                    e
                );
                // Continue to next chunk instead of failing entirely
            }
        }

        if idx < chunks.len() - 1 && !is_local_ai {
            emit_stage(
                ctx,
                "ai",
                &format!(
                    "Menunggu 30 detik (Anti-RateLimit Gemini) sebelum memproses part {}...",
                    idx + 2
                ),
                55,
                100,
            );
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        }
    }

    if all_moments.is_empty() {
        log::error!("[Compilation] AI tidak menemukan momen epik dalam transkrip.");
        return Err(CliptzyError::AIProvider(
            "AI tidak menemukan momen epik. Periksa konfigurasi provider/model AI.".into(),
        ));
    }

    log::info!("Ditemukan {} momen epik.", all_moments.len());
    emit_stage(
        ctx,
        "ai",
        &format!("Ditemukan {} momen epik.", all_moments.len()),
        70,
        100,
    );

    write_json_cache(
        &moments_cache_path,
        &EpicMomentsCacheEntry {
            ai_provider: ai_provider_name,
            ai_model,
            transcript_hash,
            moments: all_moments.clone(),
        },
    )?;

    Ok(all_moments)
}

pub async fn search_restreamers(
    ctx: &PipelineContext,
    original_title: String,
    _custom_keywords: Option<String>,
    min_duration_minutes: Option<u32>,
    main_upload_date: Option<String>,
) -> Result<Vec<RestreamerInfo>, CliptzyError> {
    log::info!(
        "Memulai Pencarian Restreamer / Reaksi (Phase 4) untuk judul: {}",
        original_title
    );

    emit_stage(
        ctx,
        "search",
        "Mencari VOD restreamer via yt-dlp...",
        75,
        100,
    );

    let cookies_path = ctx.config.browser.as_deref().map(|s| s.to_string());
    let min_duration = min_duration_minutes.unwrap_or(20);
    let search_cache_path = cache_file(&ctx.job_dir, "restreamer_search.json");

    if let Some(cached) = read_json_cache::<RestreamerSearchCacheEntry>(&search_cache_path) {
        if cached.query == original_title
            && cached.min_duration_minutes == min_duration
            && cached.main_upload_date == main_upload_date
        {
            let restreamers = migrate_cached_restreamers(&cached);
            if !restreamers.is_empty() {
                log::info!(
                    "Menggunakan hasil pencarian restreamer dari cache ({} item): {:?}",
                    restreamers.len(),
                    search_cache_path
                );
                emit_stage(
                    ctx,
                    "search",
                    &format!(
                        "Menggunakan cache pencarian ({} restreamer)...",
                        restreamers.len()
                    ),
                    90,
                    100,
                );
                return Ok(restreamers);
            }
        }

        log::info!("Cache pencarian restreamer tidak valid (kueri berubah), mencari ulang...");
    }

    // TODO:
    // Will be populated with custom keywords if provided
    let yt_search_query = format!("ytsearch40:'{}'", original_title);

    log::info!("Kueri yt-dlp: {}", yt_search_query);
    if let Some(ref upload_date) = main_upload_date {
        log::info!(
            "Filter tanggal upload restreamer: {} hingga +1 hari dari {}",
            upload_date,
            add_days_yyyymmdd(upload_date, 1).unwrap_or_else(|| "?".to_string())
        );
    }

    let mut cmd = tokio::process::Command::new(&ctx.deps.ytdlp);
    cmd.arg(&yt_search_query)
        .arg("--dump-single-json")
        .arg("--no-warnings")
        .arg("--extractor-args")
        .arg("youtube:player-client=android,web,default")
        .arg("--remote-components")
        .arg("ejs:github")
        .arg("--match-filter")
        .arg("duration > 3600");

    if let Some(browser) = &cookies_path {
        if !browser.is_empty() {
            cmd.arg("--cookies-from-browser").arg(browser);
        }
    }

    let output = cmd.output().await.map_err(|e| {
        log::error!("[Compilation] Gagal spawn yt-dlp search: {}", e);
        CliptzyError::FFmpeg {
            code: -1,
            message: format!("Gagal spawn yt-dlp search: {}", e),
        }
    })?;

    if !output.status.success() {
        let err_str = String::from_utf8_lossy(&output.stderr);
        log::error!("[Compilation] yt-dlp search error: {}", err_str);
        return Err(CliptzyError::FFmpeg {
            code: output.status.code().unwrap_or(-1),
            message: format!("yt-dlp search error: {}", err_str),
        });
    }

    let json_str = String::from_utf8_lossy(&output.stdout);
    let json_start = json_str.find('{').unwrap_or(0);
    let json_end = json_str.rfind('}').map(|i| i + 1).unwrap_or(json_str.len());
    let clean_json = if json_start < json_end {
        &json_str[json_start..json_end]
    } else {
        &json_str
    };

    let json_out: serde_json::Value = serde_json::from_str(clean_json).map_err(|e| {
        log::error!("[Compilation] Gagal parse output search JSON: {}", e);
        CliptzyError::Config(format!("Gagal parse output search JSON: {}", e))
    })?;

    let min_duration_sec = min_duration as f64 * 60.0;
    let mut potential_restreamers: Vec<RestreamerInfo> = Vec::new();
    let mut uploaders: Vec<String> = Vec::new();

    if let Some(entries) = json_out.get("entries").and_then(|e| e.as_array()) {
        for entry in entries {
            let Some(info) = parse_restreamer_entry(entry) else {
                continue;
            };

            let title = &info.title;
            let id = &info.video_id;
            let url = &info.video_url;
            let duration = info.duration;
            let entry_upload_date = info.upload_date.as_deref().unwrap_or("");

            let is_shorts_url = url.contains("/shorts/") || id.is_empty();
            let is_shorts_title = title.to_lowercase().contains("#shorts");

            let upload_date_ok = match (&main_upload_date, entry_upload_date.is_empty()) {
                (Some(main_date), false) => {
                    is_upload_within_reaction_window(entry_upload_date, main_date, 1)
                }
                (Some(_), true) => {
                    log::debug!(
                        "Lewati '{}' — tidak ada upload_date pada hasil pencarian.",
                        title
                    );
                    false
                }
                (None, _) => true,
            };

            if !is_shorts_url
                && !is_shorts_title
                && !uploaders.contains(&info.uploader)
                && duration >= min_duration_sec
                && upload_date_ok
            {
                uploaders.push(info.uploader.clone());
                potential_restreamers.push(info);
            }
        }
    }

    potential_restreamers.sort_by(|a, b| a.video_id.cmp(&b.video_id));
    potential_restreamers.dedup_by(|a, b| a.video_id == b.video_id);

    if potential_restreamers.is_empty() {
        log::warn!(
            "[Compilation] Tidak ada restreamer VOD ditemukan untuk kueri: {}",
            original_title
        );
    } else {
        log::info!(
            "Ditemukan {} URL Restreamer VOD berpotensi yang relevan.",
            potential_restreamers.len()
        );
    }

    emit_stage(
        ctx,
        "search",
        &format!(
            "Ditemukan {} restreamer potensial.",
            potential_restreamers.len()
        ),
        90,
        100,
    );

    write_json_cache(
        &search_cache_path,
        &RestreamerSearchCacheEntry {
            query: original_title,
            min_duration_minutes: min_duration,
            main_upload_date,
            restreamers: potential_restreamers.clone(),
            urls: Vec::new(),
        },
    )?;

    Ok(potential_restreamers)
}

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

    let restreamer_id = restreamer_url
        .split("v=")
        .nth(1)
        .and_then(|s| s.split('&').next())
        .unwrap_or_else(|| restreamer_url.split('/').last().unwrap_or("unknown"));

    let restr_m4a = job_dir.join(format!("restr_{}.m4a", restreamer_id));
    let restr_wav = job_dir.join(format!("restr_{}_16k.wav", restreamer_id));
    let sync_cache_path = cache_file(
        job_dir,
        &format!(
            "sync_{}.json",
            job_cache::sanitize_cache_token(restreamer_id)
        ),
    );
    let current_moments_hash = moments_hash(&moments);

    if restr_wav.exists() {
        if let Some(cached) = read_json_cache::<SyncCacheEntry>(&sync_cache_path) {
            if cached.restreamer_id == restreamer_id
                && cached.moments_hash == current_moments_hash
                && is_fingerprint_valid(&cached.restr_audio_fingerprint, &restr_wav)
                && !cached.clips.is_empty()
            {
                log::info!(
                    "Menggunakan hasil sinkronisasi dari cache ({} klip): {:?}",
                    cached.clips.len(),
                    sync_cache_path
                );
                return Ok(cached.clips);
            }
        }

        log::info!("Cache sinkronisasi tidak valid, menjalankan ulang audio fingerprinting...");
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
    }

    log::info!("Menganalisis peak match via Audio Fingerprinting (background thread)...");

    let prepared_segments =
        ensure_main_audio_segments_cached(job_dir, &main_audio_path, &moments)?;

    if prepared_segments.is_empty() {
        return Err(CliptzyError::Config(
            "Tidak ada segmen audio utama yang valid untuk sinkronisasi.".into(),
        ));
    }

    let restr_wav_str = restr_wav.to_string_lossy().to_string();
    let url_clone = restreamer_url.clone();

    let restreamer_clips =
        tokio::task::spawn_blocking(move || -> Result<Vec<RestreamerClip>, String> {
            let (restr_samples, restr_rate) =
                crate::orchestrator::audio_fingerprint::decode_wav(&restr_wav_str)?;

            let mut results = Vec::new();

            for segment in prepared_segments {
                let moment = segment.moment;
                let (moment_samples, moment_rate) =
                    crate::orchestrator::audio_fingerprint::decode_wav(&segment.wav_path)?;

                if moment_rate != restr_rate {
                    log::warn!(
                        "Sample rate segmen '{}' ({}) tidak cocok dengan restreamer ({}), melompat...",
                        moment.description,
                        moment_rate,
                        restr_rate
                    );
                    continue;
                }

                let Some(match_result) = crate::orchestrator::audio_fingerprint::find_audio_match(
                    &restr_samples,
                    &moment_samples,
                    restr_rate,
                ) else {
                    log::warn!(
                        "Tidak ditemukan kecocokan fingerprint untuk momen [{}], melompat...",
                        moment.description
                    );
                    continue;
                };

                let matched_start_time = match_result.start_time_secs;
                let moment_duration = moment.end - moment.start;
                let matched_end_time = matched_start_time + moment_duration;
                let offset_diff = matched_start_time - moment.start;

                log::debug!(
                    "Moment [{}] cocok di restr_time {:.2}s (skor: {} hash, offset frame: {}, selisih: {:.2}s)",
                    moment.description,
                    matched_start_time,
                    match_result.score,
                    match_result.frame_offset,
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
                restreamer_id: restreamer_id.to_string(),
                restr_audio_fingerprint: restr_fp,
                moments_hash: current_moments_hash,
                clips: restreamer_clips.clone(),
            },
        )?;
    }

    Ok(restreamer_clips)
}

pub async fn clip_and_label_restreamers(
    ctx: &PipelineContext,
    clips: Vec<RestreamerClip>,
) -> Result<Vec<String>, CliptzyError> {
    log::info!("Memulai Clipping & Labeling Restreamer (Phase 6)");

    let cookies_path = ctx.config.browser.as_deref().map(|s| s.to_string());
    let job_dir = &ctx.job_dir;
    std::fs::create_dir_all(job_dir)?;

    let app_dir = crate::paths::app_data_dir();
    let font_path = app_dir.join("assets").join("Geist-Black.ttf");
    let font_path_str = font_path
        .to_string_lossy()
        .to_string()
        .replace('\\', "/")
        .replace(":", "\\:");

    let mut output_paths = Vec::new();
    let hwaccel = crate::processing::ffmpeg::hwaccel::HwAccel::detect(Some(&ctx.config.hw_accel));
    let encoder = hwaccel.encoder();
    let encode_args = hwaccel.encode_args();

    for (i, clip) in clips.iter().enumerate() {
        log::info!(
            "Memproses klip {} ({} hingga {})",
            clip.description,
            clip.start,
            clip.end
        );

        let output_mp4 = job_dir.join(format!("restr_clip_{}.mp4", i));

        if output_mp4.exists() {
            log::info!("Klip {} sudah tersedia di cache", i);
            output_paths.push(output_mp4.to_string_lossy().to_string());
            continue;
        }

        let raw_mp4 = job_dir.join(format!("raw_restr_clip_{}.mp4", i));

        if !raw_mp4.exists() {
            log::info!("Mengunduh segmen mentah untuk klip {}...", i);
            let mut ytdlp_cmd = tokio::process::Command::new(&ctx.deps.ytdlp);
            ytdlp_cmd
                .arg("--download-sections")
                .arg(format!("*{}-{}", clip.start, clip.end))
                .arg("-f")
                .arg("bestvideo[height<=1080][ext=mp4]+bestaudio[ext=m4a]/best")
                .arg("-o")
                .arg(raw_mp4.to_string_lossy().to_string())
                .arg("--extractor-args")
                .arg("youtube:player-client=android,web,default")
                .arg("--remote-components")
                .arg("ejs:github");

            if let Some(browser) = &cookies_path {
                if !browser.is_empty() {
                    ytdlp_cmd.arg("--cookies-from-browser").arg(browser);
                }
            }
            ytdlp_cmd.arg(&clip.restreamer_url);

            let mut stage = crate::processing::ffmpeg::runner::PipelineStage::new(
                "yt-dlp Download Section",
                ytdlp_cmd,
            );

            if let Err(e) = stage.execute(ctx.cancel_token.clone()).await {
                log::error!(
                    "[Compilation] Gagal mendownload segmen mentah klip {}: {}",
                    i,
                    e
                );
                continue;
            }
        }

        let channel_name = clip
            .restreamer_url
            .split('@')
            .nth(1)
            .unwrap_or("Restreamer");

        use crate::processing::ffmpeg::filters::{FilterGraph, FilterNode};
        let skip_crop = ctx.config.compilation.crop_mode == "none";

        let mut drawtext_node = FilterNode::new("drawtext")
            .param("text", &format!("'{}'", channel_name))
            .param("fontcolor", "white")
            .param("fontsize", "48")
            .param("box", "1")
            .param("boxcolor", "black@0.5")
            .param("boxborderw", "10")
            .param("x", "50")
            .param("y", "50");

        if font_path.exists() {
            drawtext_node = drawtext_node.param("fontfile", &format!("'{}'", font_path_str));
        }

        let filter_str = if skip_crop {
            log::info!(
                "Mode tanpa crop: mempertahankan resolusi asli untuk klip {}",
                i
            );
            let mut graph = FilterGraph::new();
            graph.add_node(drawtext_node);
            graph.to_string()
        } else {
            let mut graph = FilterGraph::new();
            let scale_node = FilterNode::new("scale")
                .param("", "1920:1080")
                .param("force_original_aspect_ratio", "decrease");
            let pad_node = FilterNode::new("pad").param("", "1920:1080:(ow-iw)/2:(oh-ih)/2");
            graph.add_node(scale_node.outputs(&["scaled"]));
            graph.add_node(pad_node.inputs(&["scaled"]).outputs(&["padded"]));
            graph.add_node(drawtext_node.inputs(&["padded"]));
            graph.to_string()
        };

        let mut ff_cmd = tokio::process::Command::new(&ctx.deps.ffmpeg);
        ff_cmd.arg("-i").arg(raw_mp4.to_string_lossy().to_string());

        ff_cmd.arg("-vf").arg(&filter_str);
        ff_cmd.arg("-c:v").arg(encoder);
        for arg in encode_args.iter() {
            ff_cmd.arg(arg);
        }
        ff_cmd.arg("-c:a").arg("aac");
        ff_cmd.arg("-y");
        ff_cmd.arg(output_mp4.to_string_lossy().to_string());

        log::info!("Spawn FFmpeg clip untuk {:?}", clip.description);

        let mut stage =
            crate::processing::ffmpeg::runner::PipelineStage::new("Clip & Label", ff_cmd);

        match stage.execute(ctx.cancel_token.clone()).await {
            Ok(()) => {
                output_paths.push(output_mp4.to_string_lossy().to_string());
            }
            Err(e) => {
                log::error!("[Compilation] FFmpeg gagal memotong klip {}: {}", i, e);
            }
        }
    }

    Ok(output_paths)
}

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
            audio_res.video_info.title.clone(),
            search_keywords,
            Some(60),
            audio_res.video_info.upload_date.clone(),
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
        let deps = crate::utils::AppDependencies {
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
