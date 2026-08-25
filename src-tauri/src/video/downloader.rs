use crate::error::CliptzyError;
use std::path::Path;
use tokio::process::Command;
use crate::orchestrator::pipeline::{ProgressEvent, emit_progress};
use tokio_util::sync::CancellationToken;

pub async fn download_segment(
    url: &str,
    start: f64,
    end: f64,
    output_path: &Path,
    cookies_path: Option<String>,
    progress: Option<&tauri::AppHandle>,
    cancel_token: CancellationToken,
) -> Result<(), CliptzyError> {
    let app_dir = crate::paths::app_data_dir();
    
    #[cfg(target_os = "windows")]
    let ytdlp_bin = app_dir.join("bin").join("yt-dlp.exe");
    #[cfg(not(target_os = "windows"))]
    let ytdlp_bin = app_dir.join("bin").join("yt-dlp");

    if !ytdlp_bin.exists() {
        return Err(CliptzyError::Download("yt-dlp binary not found".into()));
    }

    let mut cmd = Command::new(&ytdlp_bin);
    cmd.arg(url)
       .arg("-f").arg("bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best")
       .arg("--download-sections").arg(format!("*{}-{}", start, end))
       .arg("-o").arg(output_path)
       .arg("--force-keyframes-at-cuts")
       .arg("--concurrent-fragments").arg("16");

    if let Some(cookie) = cookies_path {
        if !cookie.is_empty() {
            let cookie_path = app_dir.join(&cookie);
            if cookie_path.exists() {
                cmd.arg("--cookies").arg(cookie_path);
            } else if Path::new(&cookie).exists() {
                cmd.arg("--cookies").arg(cookie);
            }
        }
    }

    if let Some(handle) = progress {
        emit_progress(handle, &ProgressEvent {
            stage: "download".into(),
            label: format!("Downloading segment {}-{}", start, end),
            current: 0,
            total: 100,
            detail: None,
        });
    }

    let mut child = cmd.spawn().map_err(|e| CliptzyError::Download(e.to_string()))?;

    tokio::select! {
        _ = cancel_token.cancelled() => {
            let _ = child.kill().await;
            Err(CliptzyError::Cancelled)
        }
        status = child.wait() => {
            let status = status.map_err(|e| CliptzyError::Download(e.to_string()))?;
            if status.success() {
                if let Some(handle) = progress {
                    emit_progress(handle, &ProgressEvent {
                        stage: "download".into(),
                        label: "Download complete".into(),
                        current: 100,
                        total: 100,
                        detail: None,
                    });
                }
                Ok(())
            } else {
                Err(CliptzyError::Download(format!("yt-dlp exited with status: {}", status)))
            }
        }
    }
}
