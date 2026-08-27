use super::detector::FaceDetectorWrapper;
use super::models::FaceKeyframe;
use crate::error::CliptzyError;
use crate::orchestrator::pipeline::{emit_progress, ProgressEvent};
use image::GenericImageView;
use std::path::Path;
use tokio::process::Command;
use tempfile::tempdir;
use tracing::info;

const EXTREME_THRESHOLD: f32 = 0.15;
const JITTER_THRESHOLD: f32 = 0.03;

pub async fn get_face_keyframes(
    video_path: &Path,
    interval_sec: f32,
    app_handle: Option<tauri::AppHandle>,
    cancel_token: tokio_util::sync::CancellationToken,
) -> Result<Vec<FaceKeyframe>, CliptzyError> {
    info!("Starting face keyframe extraction for {:?}", video_path);

    let model_dir = Path::new("models");
    std::fs::create_dir_all(model_dir).ok();
    let model_path = model_dir.join("seeta_fd_frontal_v1.0.bin");

    if !model_path.exists() {
        info!("Model not found. Downloading to {:?}...", model_path);
        let url =
            "https://github.com/atomashpolskiy/rustface/raw/master/model/seeta_fd_frontal_v1.0.bin";
        let response = reqwest::get(url)
            .await
            .map_err(|e| CliptzyError::Internal(format!("Download failed: {}", e)))?;
        let bytes = response
            .bytes()
            .await
            .map_err(|e| CliptzyError::Internal(format!("Bytes failed: {}", e)))?;
        std::fs::write(&model_path, bytes)
            .map_err(|e| CliptzyError::Internal(format!("Write failed: {}", e)))?;
        info!("Model downloaded successfully.");
    }

    let mut detector =
        FaceDetectorWrapper::new(&model_path).map_err(|e| CliptzyError::Internal(e))?;

    let tmp_dir = tempdir().map_err(|e| CliptzyError::Internal(format!("Tempdir error: {}", e)))?;
    let fps_str = format!("1/{}", interval_sec);

    let frame_pattern = tmp_dir.path().join("frame_%04d.jpg");

    let mut child = Command::new("ffmpeg")
        .args(&[
            "-y",
            "-hide_banner",
            "-loglevel",
            "error",
            "-i",
            video_path.to_str().unwrap(),
            "-vf",
            &format!("fps={},scale=-1:360", fps_str),
            frame_pattern.to_str().unwrap(),
        ])
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
        .unwrap()
        .filter_map(|res| res.ok())
        .map(|dir_entry| dir_entry.path())
        .filter(|path| path.extension().map_or(false, |ext| ext == "jpg"))
        .collect();

    paths.sort();

    let mut raw_keyframes = Vec::new();
    let mut last_cx = 0.5;
    let mut last_cy = 0.5;
    let total_frames = paths.len();

    for (i, path) in paths.iter().enumerate() {
        if cancel_token.is_cancelled() {
            return Err(CliptzyError::Cancelled);
        }

        if let Some(app) = &app_handle {
            if i % 10 == 0 || i == total_frames - 1 {
                let pct = (i as f32 / total_frames as f32) * 100.0;
                emit_progress(
                    app,
                    &ProgressEvent {
                        stage: "tracking".into(),
                        label: format!("Mendeteksi wajah: frame {}/{}", i + 1, total_frames),
                        current: pct as u32,
                        total: 100,
                        detail: None,
                    },
                );
            }
        }

        let ts = i as f32 * interval_sec;
        let img = match image::open(path) {
            Ok(i) => i,
            Err(_) => {
                raw_keyframes.push((ts, last_cx, last_cy));
                continue;
            }
        };

        let (w, h) = img.dimensions();
        // convert to grayscale for rustface
        let gray = img.to_luma8();

        let faces = detector.detect_faces(gray.as_raw(), w, h);

        if let Some(largest) = faces.iter().max_by(|a, b| {
            let area_a = a.bbox().width() * a.bbox().height();
            let area_b = b.bbox().width() * b.bbox().height();
            area_a.cmp(&area_b)
        }) {
            let bbox = largest.bbox();
            let cx = (bbox.x() as f32 + bbox.width() as f32 / 2.0) / w as f32;
            let cy = (bbox.y() as f32 + bbox.height() as f32 / 2.0) / h as f32;

            let cx = cx.clamp(0.0, 1.0);
            let cy = cy.clamp(0.0, 1.0);

            last_cx = cx;
            last_cy = cy;
            raw_keyframes.push((ts, cx, cy));
        } else {
            raw_keyframes.push((ts, last_cx, last_cy));
        }
    }

    let mut keyframes = Vec::new();
    if !raw_keyframes.is_empty() {
        let mut classified = Vec::new();
        let (mut stable_cx, mut stable_cy) = (raw_keyframes[0].1, raw_keyframes[0].2);
        classified.push(FaceKeyframe {
            timestamp: raw_keyframes[0].0 as f64,
            cx: stable_cx,
            cy: stable_cy,
            mode: "cut".to_string(),
        });

        for i in 1..raw_keyframes.len() {
            let (ts, cx, cy) = raw_keyframes[i];
            let dist = ((cx - stable_cx).powi(2) + (cy - stable_cy).powi(2)).sqrt();

            let (final_cx, final_cy, mode) = if dist < JITTER_THRESHOLD {
                (stable_cx, stable_cy, "glide")
            } else if dist > EXTREME_THRESHOLD {
                stable_cx = cx;
                stable_cy = cy;
                (cx, cy, "cut")
            } else {
                stable_cx = cx;
                stable_cy = cy;
                (cx, cy, "glide")
            };

            classified.push(FaceKeyframe {
                timestamp: ts as f64,
                cx: final_cx,
                cy: final_cy,
                mode: mode.to_string(),
            });
        }

        if classified.len() > 2 {
            keyframes.push(classified[0].clone());
            for i in 1..classified.len() - 1 {
                let prev = &keyframes.last().unwrap();
                let curr = &classified[i];
                let next = &classified[i + 1];

                if (prev.cx - curr.cx).abs() < 0.0001
                    && (prev.cy - curr.cy).abs() < 0.0001
                    && (curr.cx - next.cx).abs() < 0.0001
                    && (curr.cy - next.cy).abs() < 0.0001
                {
                    continue;
                }
                keyframes.push(curr.clone());
            }
            keyframes.push(classified.last().unwrap().clone());
        } else {
            keyframes = classified;
        }
    }

    info!("Extracted {} face keyframes", keyframes.len());
    Ok(keyframes)
}
