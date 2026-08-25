use std::path::{Path, PathBuf};
use yt_dlp::Downloader;

#[derive(serde::Serialize)]
pub struct SegmentInfo {
    pub start: f64,
    pub end: f64,
    pub score: f64,
}

#[derive(serde::Serialize)]
pub struct VideoAnalysisResult {
    pub video_id: String,
    pub title: String,
    pub thumbnail: String,
    pub duration: f64,
    pub segments: Vec<SegmentInfo>,
}

pub async fn get_downloader(
    bin_dir: PathBuf,
    cookies_path: Option<String>,
) -> Result<Downloader, String> {
    let mut builder = Downloader::with_new_binaries(bin_dir.clone(), bin_dir.clone())
        .await
        .map_err(|e| format!("Gagal inisialisasi downloader: {}", e))?;

    if let Some(cookie_file) = cookies_path {
        if !cookie_file.is_empty() {
            let app_dir = crate::paths::app_data_dir();
            let cookie_path = app_dir.join(&cookie_file);
            if cookie_path.exists() {
                builder = builder.with_cookies(cookie_path);
            } else if Path::new(&cookie_file).exists() {
                builder = builder.with_cookies(PathBuf::from(cookie_file));
            }
        }
    }

    builder.build().await.map_err(|e| format!("Gagal build downloader: {}", e))
}

pub async fn analyze_youtube_video(
    url: &str,
    cookies_path: Option<String>,
) -> Result<VideoAnalysisResult, String> {
    let app_dir = crate::paths::app_data_dir();
    let bin_dir = app_dir.join("bin");

    let downloader = get_downloader(bin_dir, cookies_path).await?;

    let video = downloader
        .fetch_video_infos(url)
        .await
        .map_err(|e| format!("Gagal fetch info video: {}", e))?;

    let heatmap = video
        .get_heatmap()
        .ok_or("Heatmap tidak tersedia untuk video ini")?;

    let engaged = heatmap.get_highly_engaged_segments(0.5);

    let segments: Vec<SegmentInfo> = engaged
        .into_iter()
        .map(|h| SegmentInfo {
            start: h.start_time,
            end: h.end_time,
            score: h.value,
        })
        .collect();

    Ok(VideoAnalysisResult {
        video_id: video.id,
        title: video.title,
        thumbnail: video.thumbnail.unwrap_or_default(),
        duration: video.duration.unwrap_or(0) as f64,
        segments,
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
