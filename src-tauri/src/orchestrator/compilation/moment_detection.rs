use super::helpers::emit_stage;
use super::models::{EpicMoment, EpicMomentsCacheEntry, TranscriptCacheEntry};
use crate::config::models::AIConfig;
use crate::error::CliptzyError;
use crate::orchestrator::job_cache::{
    self, ai_model_name, cache_file, fingerprint, hash_payload, is_fingerprint_valid,
    read_json_cache, write_json_cache,
};
use crate::orchestrator::pipeline::PipelineContext;

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
            4. The duration of each segment CANNOT exceed 30 SECONDS except when the full context of the team fight or momentum shift requires it. Do not cut the action abruptly; ensure the buildup and aftermath are included.\n\
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
    let chunk_size = if is_local_ai { 150 } else { 1500 };
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
            String::new()
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
