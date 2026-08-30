use std::path::{Path, PathBuf};
use yt_dlp::Downloader;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct SegmentInfo {
    pub start: f64,
    pub end: f64,
    pub score: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct VideoAnalysisResult {
    pub video_id: String,
    pub title: String,
    pub thumbnail: String,
    pub duration: f64,
    pub segments: Vec<SegmentInfo>,
    pub stream_url: Option<String>,
}

pub async fn get_downloader(
    bin_dir: PathBuf,
    cookies_path: Option<String>,
) -> Result<Downloader, String> {
    let mut builder = Downloader::with_new_binaries(bin_dir.clone(), bin_dir.clone())
        .await
        .map_err(|e| format!("Gagal inisialisasi downloader: {}", e))?;

    let mut extra_args = vec![
        "--extractor-args".to_string(),
        "youtube:player-client=android,web,default".to_string(),
        "--remote-components".to_string(),
        "ejs:github".to_string(),
    ];

    if let Some(browser) = cookies_path {
        if !browser.is_empty() {
            log::info!("Menggunakan cookies dari browser: {}", browser);
            extra_args.push("--cookies-from-browser".to_string());
            extra_args.push(browser);
        }
    }

    builder = builder.with_args(extra_args);



    builder
        .build()
        .await
        .map_err(|e| format!("Gagal build downloader: {}", e))
}

pub async fn analyze_youtube_video(
    url: &str,
    cookies_path: Option<String>,
    ytdlp_bin: &Path,
) -> Result<VideoAnalysisResult, String> {
    let mut cmd = tokio::process::Command::new(ytdlp_bin);
    
    let mut args_to_log = vec!["--dump-single-json".to_string(), "--no-warnings".to_string()];
    cmd.arg("--dump-single-json").arg("--no-warnings");

    args_to_log.push("--extractor-args".to_string());
    args_to_log.push("youtube:player-client=android,web,default".to_string());
    args_to_log.push("--remote-components".to_string());
    args_to_log.push("ejs:github".to_string());
    
    cmd.arg("--extractor-args").arg("youtube:player-client=android,web,default")
       .arg("--remote-components").arg("ejs:github");

    if let Some(browser) = cookies_path {
        if !browser.is_empty() {
            args_to_log.push("--cookies-from-browser".to_string());
            args_to_log.push(browser.clone());
            cmd.arg("--cookies-from-browser").arg(browser);
        }
    }
    
    args_to_log.push(url.to_string());
    cmd.arg(url);

    log::info!("Menjalankan perintah fetch video: yt-dlp {}", args_to_log.join(" "));

    let output = cmd.output().await.map_err(|e| format!("Gagal mengeksekusi yt-dlp: {}", e))?;

    if !output.status.success() {
        let err_str = String::from_utf8_lossy(&output.stderr);
        log::error!("yt-dlp error: {}", err_str);
        return Err(format!("Gagal fetch info video: {}", err_str));
    }

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    
    // Kadang yt-dlp mengeluarkan pesan peringatan sebelum JSON object
    let json_start = stdout_str.find('{').unwrap_or(0);
    let json_end = stdout_str.rfind('}').map(|i| i + 1).unwrap_or(stdout_str.len());
    let clean_json = if json_start < json_end {
        &stdout_str[json_start..json_end]
    } else {
        &stdout_str
    };
    
    let video: yt_dlp::model::video::Video = match serde_json::from_str(clean_json) {
        Ok(v) => v,
        Err(e) => {
            log::error!("Gagal memparsing JSON dari yt-dlp: {}", e);
            let peek = &clean_json[..std::cmp::min(clean_json.len(), 500)];
            log::debug!("Output yt-dlp: {}...", peek);
            return Err(format!("Gagal memparsing JSON dari yt-dlp: {}", e));
        }
    };

    let engaged = video.get_heatmap()
        .map(|h| h.get_highly_engaged_segments(0.5))
        .unwrap_or_default();

    let config = crate::config::models::AppConfig::load().unwrap_or_default();
    let padding = config.padding as f64;
    let min_duration = config.min_duration as f64;
    let video_duration = video.duration.unwrap_or(0) as f64;

    let mut segments: Vec<SegmentInfo> = Vec::new();
    for h in engaged {
        let mut start = (h.start_time - padding).max(0.0);
        let mut end = (h.end_time + padding).min(video_duration);

        if end - start < min_duration {
            end = (start + min_duration).min(video_duration);
            if end - start < min_duration {
                start = (end - min_duration).max(0.0);
            }
        }

        segments.push(SegmentInfo {
            start,
            end,
            score: h.value,
        });
    }

    log::info!("Mencari format media yang cocok untuk video ID: {}", video.id);

    let valid_formats: Vec<_> = video
        .formats
        .iter()
        .filter(|f| {
            f.format_type().is_audio_and_video()
                || f.format_type().is_audio()
                || f.format_type().is_video()
        })
        .collect();

    let stream_url = valid_formats
        .iter()
        .find(|f| f.format_type().is_audio_and_video() && f.download_info.ext.as_str() == "mp4")
        .and_then(|f| f.download_info.url.clone())
        .or_else(|| {
            video
                .best_audio_video_format()
                .ok()
                .and_then(|f| f.download_info.url.clone())
        })
        .or_else(|| {
            valid_formats
                .iter()
                .find(|f| f.format_type().is_audio())
                .and_then(|f| f.download_info.url.clone())
        })
        .or_else(|| {
            valid_formats
                .first()
                .and_then(|f| f.download_info.url.clone())
        });

    if let Some(ref url) = stream_url {
        log::info!(
            "Berhasil mendapatkan stream URL: {}...",
            &url[..std::cmp::min(url.len(), 50)]
        );
    } else {
        log::warn!("Tidak menemukan stream URL yang valid dari yt-dlp!");
    }

    Ok(VideoAnalysisResult {
        video_id: video.id.clone(),
        title: video.title.clone(),
        thumbnail: video.thumbnail.unwrap_or_default(),
        duration: video_duration,
        segments,
        stream_url,
    })
}

pub async fn download_youtube_video(
    url: &str,
    output_path: &Path,
    cookies_path: Option<String>,
) -> Result<(), String> {
    let app_dir = crate::paths::app_data_dir();
    let bin_dir = app_dir.join("bin");

    let downloader = get_downloader(bin_dir, cookies_path).await?;

    // Fetch video first
    let video = downloader
        .fetch_video_infos(url)
        .await
        .map_err(|e| format!("Gagal fetch info video: {}", e))?;

    // Execute download
    downloader
        .download(&video, output_path)
        .execute()
        .await
        .map_err(|e| format!("Gagal mendownload video: {}", e))?;

    Ok(())
}


