use crate::error::CliptzyError;
use crate::orchestrator::pipeline::{emit_progress, ProgressEvent};
use std::path::Path;
use tokio::process::Command;
use tokio_util::sync::CancellationToken;
use tokio::io::{AsyncBufReadExt, BufReader};
use std::process::Stdio;

fn extract_percentage(line: &str) -> Option<f64> {
    if !line.contains("[download]") {
        return None;
    }
    if let Some(idx) = line.find('%') {
        let before_pct = &line[..idx];
        let words: Vec<&str> = before_pct.split_whitespace().collect();
        if let Some(last) = words.last() {
            let clean: String = last.chars().filter(|c| c.is_ascii_digit() || *c == '.').collect();
            return clean.parse::<f64>().ok();
        }
    }
    None
}

fn extract_ffmpeg_time(line: &str, total_duration: f64) -> Option<f64> {
    if let Some(idx) = line.find("time=") {
        let after_time = &line[idx + 5..];
        let time_str = after_time.split_whitespace().next().unwrap_or("");
        let parts: Vec<&str> = time_str.split(':').collect();
        if parts.len() == 3 {
            let h: f64 = parts[0].parse().unwrap_or(0.0);
            let m: f64 = parts[1].parse().unwrap_or(0.0);
            let s: f64 = parts[2].parse().unwrap_or(0.0);
            let current_sec = h * 3600.0 + m * 60.0 + s;
            if total_duration > 0.0 {
                let pct = (current_sec / total_duration) * 100.0;
                return Some(pct.min(99.9));
            }
        }
    }
    None
}

pub async fn download_segment(
    url: &str,
    start: f64,
    end: f64,
    output_path: &Path,
    cookies_path: Option<String>,
    ytdlp_bin: &Path,
    progress: Option<&tauri::AppHandle>,
    cancel_token: CancellationToken,
) -> Result<(), CliptzyError> {
    let app_dir = crate::paths::app_data_dir();

    let mut cmd = Command::new(ytdlp_bin);
    cmd.arg(url)
        .arg("-f")
        .arg("bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best")
        .arg("--download-sections")
        .arg(format!("*{}-{}", start, end))
        .arg("-o")
        .arg(output_path)
        .arg("--force-keyframes-at-cuts")
        .arg("--concurrent-fragments")
        .arg("16")
        .arg("--newline")
        .arg("--ffmpeg-location")
        .arg(app_dir.join("bin"))
        .arg("--extractor-args")
        .arg("youtube:player-client=android,web,default")
        .arg("--remote-components")
        .arg("ejs:github")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if let Some(browser) = cookies_path {
        if !browser.is_empty() {
            cmd.arg("--cookies-from-browser").arg(browser);
        }
    }

    if let Some(handle) = progress {
        emit_progress(
            handle,
            &ProgressEvent {
                stage: "download".into(),
                label: format!("Downloading segment {}-{}", start, end),
                current: 0,
                total: 100,
                detail: None,
            },
        );
    }

    let mut child = cmd
        .spawn()
        .map_err(|e| CliptzyError::Download(e.to_string()))?;

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    
    let handle_clone1 = progress.cloned();
    let handle_clone2 = progress.cloned();
    
    let total_duration = end - start;

    tokio::spawn(async move {
        let mut reader = BufReader::new(stdout).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            if let Some(pct) = extract_percentage(&line) {
                if let Some(h) = &handle_clone1 {
                    let mut detail_str = line.clone();
                    if detail_str.contains("[download]") {
                        detail_str = detail_str.replace("[download]", "").trim().to_string();
                    }
                    emit_progress(
                        h,
                        &ProgressEvent {
                            stage: "download".into(),
                            label: detail_str,
                            current: pct as u32,
                            total: 100,
                            detail: None,
                        },
                    );
                }
            }
        }
    });

    tokio::spawn(async move {
        // Read stderr splitting by carriage return '\r' because ffmpeg uses it for progress updates
        let mut reader = BufReader::new(stderr);
        let mut buf = Vec::new();
        while let Ok(n) = reader.read_until(b'\r', &mut buf).await {
            if n == 0 { break; }
            let line = String::from_utf8_lossy(&buf).to_string();
            buf.clear();
            
            if let Some(pct) = extract_ffmpeg_time(&line, total_duration) {
                if let Some(h) = &handle_clone2 {
                    emit_progress(
                        h,
                        &ProgressEvent {
                            stage: "download".into(),
                            label: format!("Mendownload & Memotong Video ({:.1}%)", pct),
                            current: pct as u32,
                            total: 100,
                            detail: None,
                        },
                    );
                }
            }
        }
    });

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
