use super::detector::FaceDetectorWrapper;
use super::models::FaceKeyframe;
use crate::error::CliptzyError;
use crate::orchestrator::pipeline::{emit_progress, ProgressEvent};
use image::GenericImageView;
use std::path::Path;
use tokio::process::Command;
use tempfile::tempdir;
use log::info;

pub async fn get_face_keyframes(
    video_path: &Path,
    interval_sec: f32,
    tracking_mode: String,
    app_handle: Option<tauri::AppHandle>,
    cancel_token: tokio_util::sync::CancellationToken,
) -> Result<Vec<FaceKeyframe>, CliptzyError> {
    info!("Starting face keyframe extraction for {:?} with mode: {}", video_path, tracking_mode);

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
    
    let fps = if tracking_mode == "cinematic" {
        15.0
    } else {
        1.0 / interval_sec
    };
    
    let fps_str = format!("{}", fps);

    let frame_pattern = tmp_dir.path().join("frame_%04d.jpg");

    let ffmpeg_bin = crate::utils::find_executable("ffmpeg").unwrap_or_else(|| std::path::PathBuf::from("ffmpeg"));
    
    let mut args = vec![
        "-y".to_string(),
        "-hide_banner".to_string(),
        "-loglevel".to_string(),
        "error".to_string(),
        "-i".to_string(),
        video_path.to_str().unwrap().to_string(),
    ];
    
    if tracking_mode == "static" {
        args.push("-vframes".to_string());
        args.push("1".to_string());
    }
    
    let scale_opt = if tracking_mode == "cinematic" {
        "scale=-1:240"
    } else {
        "scale=-1:360"
    };
    
    args.push("-vf".to_string());
    args.push(format!("fps={},{}", fps_str, scale_opt));
    args.push(frame_pattern.to_str().unwrap().to_string());

    let mut child = Command::new(&ffmpeg_bin)
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

    let mut prev_pyramid: Option<Vec<image::GrayImage>> = None;
    let mut prev_point: Option<(f32, f32)> = None;
    let mut last_detection_frame = -100;
    let force_detect_interval = (fps * 2.0) as isize;

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
                        label: format!("Melacak titik wajah: frame {}/{}", i + 1, total_frames),
                        current: pct as u32,
                        total: 100,
                        detail: None,
                    },
                );
            }
        }

        let ts = i as f32 / fps;
        let img = match image::open(path) {
            Ok(i) => i,
            Err(_) => {
                raw_keyframes.push((ts, last_cx, last_cy));
                continue;
            }
        };

        let (w, h) = img.dimensions();
        let gray = img.to_luma8();
        let mut point_tracked = false;
        
        let curr_pyramid = if tracking_mode == "cinematic" {
            Some(optical_flow_lk::build_pyramid(&gray, 3))
        } else {
            None
        };

        if tracking_mode == "cinematic" {
            if let (Some(prev_pyr), Some(prev_pt), Some(curr_pyr)) = (&prev_pyramid, &prev_point, &curr_pyramid) {
                let next_res = optical_flow_lk::calc_optical_flow_ex(
                    prev_pyr,
                    curr_pyr,
                    &[*prev_pt],
                    None,
                    15, // window_size
                    30, // max_iterations
                    optical_flow_lk::DEFAULT_MIN_EIGEN_THRESHOLD,
                );

                if let Some(res) = next_res.first() {
                    if res.status == optical_flow_lk::TrackStatus::Tracked {
                        if res.pos.0 >= 0.0 && res.pos.0 < w as f32 && res.pos.1 >= 0.0 && res.pos.1 < h as f32 {
                            if i as isize - last_detection_frame < force_detect_interval {
                                point_tracked = true;
                                prev_point = Some(res.pos);
                                last_cx = res.pos.0 / w as f32;
                                last_cy = res.pos.1 / h as f32;
                                raw_keyframes.push((ts, last_cx, last_cy));
                            }
                        }
                    }
                }
            }
        }

        if !point_tracked {
            let faces = detector.detect_faces(gray.as_raw(), w, h);
            if let Some(largest) = faces.iter().max_by(|a, b| {
                let area_a = a.bbox().width() * a.bbox().height();
                let area_b = b.bbox().width() * b.bbox().height();
                area_a.cmp(&area_b)
            }) {
                let bbox = largest.bbox();
                
                let px = bbox.x() as f32 + bbox.width() as f32 / 2.0;
                let py = if tracking_mode == "cinematic" {
                    bbox.y() as f32 + bbox.height() as f32 * 0.4
                } else {
                    bbox.y() as f32 + bbox.height() as f32 / 2.0
                };
                
                let px = px.clamp(0.0, w as f32 - 1.0);
                let py = py.clamp(0.0, h as f32 - 1.0);

                prev_point = Some((px, py));
                last_detection_frame = i as isize;

                last_cx = px / w as f32;
                last_cy = py / h as f32;
                raw_keyframes.push((ts, last_cx, last_cy));
            } else {
                prev_point = None; 
                raw_keyframes.push((ts, last_cx, last_cy));
            }
        }

        prev_pyramid = curr_pyramid;
    }

    let mut keyframes = Vec::new();
    if !raw_keyframes.is_empty() {
        if tracking_mode == "cinematic" {
            let alpha = 0.15;
            let mut smooth_cx = raw_keyframes[0].1;
            let mut smooth_cy = raw_keyframes[0].2;

            for (ts, cx, cy) in raw_keyframes {
                smooth_cx = alpha * cx + (1.0 - alpha) * smooth_cx;
                smooth_cy = alpha * cy + (1.0 - alpha) * smooth_cy;

                if ts % interval_sec < (1.0 / fps) {
                    keyframes.push(FaceKeyframe {
                        timestamp: ts as f64,
                        cx: smooth_cx,
                        cy: smooth_cy,
                        mode: "glide".to_string(),
                    });
                }
            }
        } else if tracking_mode == "static" {
            // For static mode, there's only 1 frame extracted anyway
            keyframes.push(FaceKeyframe {
                timestamp: 0.0,
                cx: raw_keyframes[0].1,
                cy: raw_keyframes[0].2,
                mode: "cut".to_string(),
            });
        } else {
            // "fast" mode (old behavior)
            // It just passes the raw keyframes, maybe apply jitter filter?
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

                let (final_cx, final_cy, mode) = if dist < 0.03 {
                    (stable_cx, stable_cy, "glide")
                } else if dist > 0.15 {
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
            keyframes = classified;
        }
    }

    info!("Extracted {} face keyframes", keyframes.len());
    Ok(keyframes)
}
