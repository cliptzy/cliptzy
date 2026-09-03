use crate::error::CliptzyError;
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use tokio::process::Command;
use tokio_util::sync::CancellationToken;

pub struct ExtractedFrames {
    pub paths: Vec<PathBuf>,
    pub fps: f32,
    /// Owns the temporary directory so frame files persist for the lifetime
    /// of this struct. Made `pub` so the face tracker cache can keep the
    /// directory alive after extraction.
    pub temp_dir: TempDir,
}

impl ExtractedFrames {
    /// Returns true if the underlying temporary directory still exists on disk.
    /// This is used by the frame cache to detect stale entries.
    pub fn is_valid(&self) -> bool {
        self.temp_dir.path().exists()
    }
}

pub async fn extract_frames(
    video_path: &Path,
    tracking_mode: &str,
    interval_sec: f32,
    cancel_token: &CancellationToken,
) -> Result<ExtractedFrames, CliptzyError> {
    let tmp_dir =
        TempDir::new().map_err(|e| CliptzyError::Internal(format!("Tempdir error: {}", e)))?;

    let fps = if tracking_mode == "cinematic" {
        15.0
    } else {
        1.0 / interval_sec
    };

    let frame_pattern = tmp_dir.path().join("frame_%04d.jpg");
    let ffmpeg_bin =
        crate::utils::find_executable("ffmpeg").unwrap_or_else(|| PathBuf::from("ffmpeg"));

    let video_path_str = video_path
        .to_str()
        .ok_or_else(|| CliptzyError::Internal(format!("Invalid video path: {:?}", video_path)))?;
    let frame_pattern_str = frame_pattern
        .to_str()
        .ok_or_else(|| CliptzyError::Internal("Invalid temp frame path".into()))?;

    let scale_opt = if tracking_mode == "cinematic" {
        "scale=-1:240"
    } else {
        "scale=-1:360"
    };

    let mut args = vec![
        "-y".to_string(),
        "-hide_banner".to_string(),
        "-loglevel".to_string(),
        "error".to_string(),
        "-i".to_string(),
        video_path_str.to_string(),
    ];

    if tracking_mode == "static" {
        args.push("-vframes".to_string());
        args.push("1".to_string());
    }

    args.push("-vf".to_string());
    args.push(format!("fps={},{}", fps, scale_opt));
    args.push(frame_pattern_str.to_string());

    let mut child =
        Command::new(&ffmpeg_bin)
            .args(&args)
            .spawn()
            .map_err(|e| CliptzyError::FFmpeg {
                code: -1,
                message: format!("FFmpeg extract spawn failed: {}", e),
            })?;

    let status = tokio::select! {
        _ = cancel_token.cancelled() => {
            let _ = child.kill().await;
            return Err(CliptzyError::Cancelled);
        }
        res = child.wait() => {
            res.map_err(|e| CliptzyError::FFmpeg {
                code: -1,
                message: format!("FFmpeg wait failed: {}", e),
            })?
        }
    };

    if !status.success() {
        return Err(CliptzyError::FFmpeg {
            code: status.code().unwrap_or(-1),
            message: "Failed to extract frames".into(),
        });
    }

    let mut paths: Vec<_> = std::fs::read_dir(tmp_dir.path())
        .map_err(CliptzyError::Io)?
        .filter_map(|res| res.ok())
        .map(|dir_entry| dir_entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "jpg"))
        .collect();
    paths.sort();

    Ok(ExtractedFrames {
        paths,
        fps,
        temp_dir: tmp_dir,
    })
}
