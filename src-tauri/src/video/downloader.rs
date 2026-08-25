use crate::error::CliptzyError;
use std::path::{Path, PathBuf};

// Nanti implementasi wrapper yt-dlp yang lebih canggih (range download, progress)
pub async fn download_segment(
    url: &str,
    start: f64,
    end: f64,
    output_path: &Path,
    cookies_path: Option<String>,
) -> Result<(), CliptzyError> {
    // Placeholder untuk Phase 1
    // Nanti akan memanggil downloader dengan `--download-sections *start-end`
    // via Command std::process::Command atau library yt-dlp
    
    // Saat ini sekadar delegate ke downloader biasa untuk testing
    crate::video::youtube::download_youtube_video(url, output_path, cookies_path).await.map_err(CliptzyError::Download)
}
