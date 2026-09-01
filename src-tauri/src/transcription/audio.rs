use crate::error::CliptzyError;
use rust_ffmpeg::builder::FFmpegBuilder;
use std::path::Path;

/// Decode mono f32 samples from a 16-bit PCM WAV file.
pub fn decode_wav(path: &str) -> Result<(Vec<f32>, u32), String> {
    let mut reader = hound::WavReader::open(path).map_err(|e| e.to_string())?;
    let sample_rate = reader.spec().sample_rate;

    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap_or(0) as f32 / 32768.0)
        .collect();

    Ok((samples, sample_rate))
}

/// Write mono f32 samples to a 16-bit PCM WAV file.
pub fn write_wav_segment(path: &Path, samples: &[f32], sample_rate: u32) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(path, spec).map_err(|e| e.to_string())?;
    for &sample in samples {
        let amplitude = (sample * 32767.0).clamp(-32768.0, 32767.0) as i16;
        writer.write_sample(amplitude).map_err(|e| e.to_string())?;
    }
    writer.finalize().map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn extract_audio_segment(
    input_url: &str,
    start: f64,
    end: f64,
    output_path: &Path,
    cookies_path: Option<&str>,
    ytdlp_bin: &Path,
) -> Result<(), CliptzyError> {
    let app_dir = crate::paths::app_data_dir();

    // 1. Resolve source file (Local File or Download to Cache)
    let source_file = if input_url.starts_with("http") {
        let cache_dir = app_dir.join("cache");
        std::fs::create_dir_all(&cache_dir).ok();

        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        input_url.hash(&mut hasher);
        let hash = hasher.finish();

        let cached_audio_path = cache_dir.join(format!("full_audio_{}.wav", hash));

        if !cached_audio_path.exists() {
            log::info!(
                "Mengunduh audio penuh untuk di-cache: {:?}",
                cached_audio_path
            );
            let mut cmd = tokio::process::Command::new(ytdlp_bin);

            cmd.arg("-f")
                .arg("bestaudio")
                .arg("--extract-audio")
                .arg("--audio-format")
                .arg("wav")
                .arg("--postprocessor-args")
                .arg("ffmpeg:-ar 16000 -ac 1 -c:a pcm_s16le")
                .arg("--extractor-args")
                .arg("youtube:player-client=android,web,default")
                .arg("--remote-components")
                .arg("ejs:github")
                .arg("-o")
                .arg(cached_audio_path.to_string_lossy().to_string());

            if let Some(browser) = cookies_path {
                if !browser.is_empty() {
                    cmd.arg("--cookies-from-browser").arg(browser);
                }
            }

            let mut child = cmd
                .arg(input_url)
                .spawn()
                .map_err(|e| CliptzyError::FFmpeg {
                    code: -1,
                    message: format!("Gagal menjalankan yt-dlp: {}", e),
                })?;

            let status = child.wait().await.map_err(|e| CliptzyError::FFmpeg {
                code: -1,
                message: format!("Proses gagal: {}", e),
            })?;

            if !status.success() {
                let _ = std::fs::remove_file(&cached_audio_path);
                return Err(CliptzyError::FFmpeg {
                    code: status.code().unwrap_or(-1),
                    message: "yt-dlp download failed".into(),
                });
            }
        } else {
            log::info!("Audio cache ditemukan: {:?}", cached_audio_path);
        }
        cached_audio_path.to_string_lossy().to_string()
    } else {
        input_url.to_string()
    };

    // 2. Extract Segment from Local File
    log::info!("Memotong audio lokal dari detik {} sampai {}", start, end);
    let mut builder = FFmpegBuilder::new().map_err(|e| CliptzyError::FFmpeg {
        code: -1,
        message: format!("FFmpeg builder error: {}", e),
    })?;

    builder = builder
        // For local files, FFmpegBuilder will put input first, then output options.
        // It's still very fast for local files even with output seek.
        .input_path(std::path::PathBuf::from(&source_file))
        .raw_args(vec!["-ss".to_string(), start.to_string()])
        .raw_args(vec!["-to".to_string(), end.to_string()])
        .raw_args(vec!["-vn".to_string()])
        .raw_args(vec!["-y".to_string()])
        .raw_args(vec!["-ar".to_string(), "16000".to_string()])
        .raw_args(vec!["-ac".to_string(), "1".to_string()])
        .raw_args(vec!["-c:a".to_string(), "pcm_s16le".to_string()])
        .output_path(output_path.to_path_buf());

    log::info!("FFmpeg Audio Command: {:?}", builder);

    let process = builder.spawn().await.map_err(|e| CliptzyError::FFmpeg {
        code: -1,
        message: format!("Spawn failed: {}", e),
    })?;

    process.wait().await.map_err(|e| CliptzyError::FFmpeg {
        code: -1,
        message: format!("Process failed: {}", e),
    })?;

    Ok(())
}
