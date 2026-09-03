use super::detector::FaceDetectorWrapper;
use super::frame_extractor::extract_frames;
use super::tracker_strategy::track_faces_in_frames;
use crate::error::CliptzyError;
use crate::face::models::NormalizedCenter;
use crate::orchestrator::pipeline::emit_progress;
use image::{self, GenericImageView};
use log::info;
use std::path::Path;
use tokio_util::sync::CancellationToken;

pub async fn get_face_keyframes(
    video_path: &Path,
    interval_sec: f32,
    tracking_mode: String,
    app_handle: Option<tauri::AppHandle>,
    cancel_token: CancellationToken,
    visual_analyzer: Option<&crate::analysis::visual::VisualEmotionAnalyzer>,
) -> Result<
    (
        Vec<super::models::FaceKeyframe>,
        Option<Vec<crate::analysis::AnalysisSegment>>,
    ),
    CliptzyError,
> {
    info!(
        "Starting face keyframe extraction for {:?} with mode: {}",
        video_path, tracking_mode
    );

    let model_path = crate::ai::onnx::ensure_model_downloaded(
        "seeta_fd_frontal_v1.0.bin",
        "https://github.com/atomashpolskiy/rustface/raw/master/model/seeta_fd_frontal_v1.0.bin",
    )
    .await
    .map_err(CliptzyError::Internal)?;

    let mut detector = FaceDetectorWrapper::new(&model_path).map_err(CliptzyError::Internal)?;

    let extracted = extract_frames(video_path, &tracking_mode, interval_sec, &cancel_token).await?;

    let (keyframes, analysis) = track_faces_in_frames(
        &extracted.paths,
        extracted.fps,
        &tracking_mode,
        interval_sec,
        &mut detector,
        app_handle.as_ref(),
        &cancel_token,
        visual_analyzer,
    )?;

    info!("Extracted {} face keyframes", keyframes.len());
    Ok((keyframes, analysis))
}

/// Detect and return normalized keyframes for **two** faces in a video.
///
/// This helper is used by the `MultiFaceCrop` mode. It extracts frames at the
/// requested interval, runs a fast face detector on **each** frame and attempts to
/// keep the two most prominent faces (by bounding‑box area). The returned data
/// is a `MultiFaceData` struct containing two vectors of `FaceKeyframe` – one
/// for each identified face – together with the tracking mode used.
///
/// The implementation deliberately mirrors the single‑face pipeline but skips
/// optical‑flow tracking; it simply records the centre of the detected faces per
/// frame. When fewer than two faces are found in a frame the previous centre is
/// reused, ensuring the output vectors have the same length as the number of
/// processed frames. If **no** faces are detected at all the function returns a
/// `CliptzyError::Config` error, which will surface to the UI as “no faces
/// detected”.
pub async fn get_two_faces_normalized_centers(
    video_path: &Path,
    interval_sec: f32,
    tracking_mode: String,
    app_handle: Option<tauri::AppHandle>,
    cancel_token: CancellationToken,
    _visual_analyzer: Option<&crate::analysis::visual::VisualEmotionAnalyzer>,
) -> Result<
    (
        crate::face::models::MultiFaceData,
        Option<Vec<crate::analysis::AnalysisSegment>>,
    ),
    CliptzyError,
> {
    // ---------------------------------------------------------------------
    // 1. Load the face detection model.
    // ---------------------------------------------------------------------
    let model_path = crate::ai::onnx::ensure_model_downloaded(
        "seeta_fd_frontal_v1.0.bin",
        "https://github.com/atomashpolskiy/rustface/raw/master/model/seeta_fd_frontal_v1.0.bin",
    )
    .await
    .map_err(CliptzyError::Internal)?;

    let mut detector = FaceDetectorWrapper::new(&model_path).map_err(CliptzyError::Internal)?;

    // ---------------------------------------------------------------------
    // 2. Extract frames for analysis.
    // ---------------------------------------------------------------------
    let extracted = extract_frames(video_path, &tracking_mode, interval_sec, &cancel_token).await?;
    let fps = extracted.fps;
    let total_frames = extracted.paths.len();

    // ---------------------------------------------------------------------
    // 3. Iterate over frames, detect faces and build keyframe vectors.
    // ---------------------------------------------------------------------
    let mut face1_keyframes: Vec<crate::face::models::FaceKeyframe> = Vec::new();
    let mut face2_keyframes: Vec<crate::face::models::FaceKeyframe> = Vec::new();
    let mut last1 = NormalizedCenter { cx: 0.5, cy: 0.5 };
    let mut last2 = NormalizedCenter { cx: 0.5, cy: 0.5 };

    for (i, path) in extracted.paths.iter().enumerate() {
        if cancel_token.is_cancelled() {
            return Err(CliptzyError::Cancelled);
        }

        // Emit progress for the UI – same cadence as the single‑face version.
        if let Some(app) = &app_handle {
            if i % 10 == 0 || i == total_frames - 1 {
                let pct = (i as f32 / total_frames as f32) * 100.0;
                emit_progress(
                    app,
                    &crate::orchestrator::pipeline::ProgressEvent {
                        stage: "tracking_multi".into(),
                        label: format!("Detecting two faces: frame {}/{}", i + 1, total_frames),
                        current: pct as u32,
                        total: 100,
                        detail: None,
                    },
                );
            }
        }

        // Load image data.
        let img = match image::open(path) {
            Ok(i) => i,
            Err(_) => {
                // If the image cannot be opened we reuse the previous centres.
                face1_keyframes.push(crate::face::models::FaceKeyframe {
                    timestamp: (i as f32 / fps) as f64,
                    cx: last1.cx,
                    cy: last1.cy,
                    mode: "glide".to_string(),
                });
                face2_keyframes.push(crate::face::models::FaceKeyframe {
                    timestamp: (i as f32 / fps) as f64,
                    cx: last2.cx,
                    cy: last2.cy,
                    mode: "glide".to_string(),
                });
                continue;
            }
        };

        let (w, h) = img.dimensions();
        let gray = img.to_luma8();
        let detections = detector.detect_faces(&gray, w, h);

        // Sort detections by area descending.
        let mut detections = detections;
        // Sort detections by bounding‑box area descending.
        detections.sort_by(|a, b| {
            let rect_a = a.bbox();
            let rect_b = b.bbox();
            let area_a = rect_a.width() * rect_a.height();
            let area_b = rect_b.width() * rect_b.height();
            area_b.cmp(&area_a)
        });
        let to_center = |d: &rustface::FaceInfo| {
            let rect = d.bbox();
            NormalizedCenter {
                cx: (rect.x() as f32 + rect.width() as f32 / 2.0) / w as f32,
                cy: (rect.y() as f32 + rect.height() as f32 / 2.0) / h as f32,
            }
        };

        // Determine centres for up to two faces (ensure ordering left‑to‑right).
        let (center1, center2) = match detections.len() {
            0 => (last1.clone(), last2.clone()),
            1 => {
                let c = to_center(&detections[0]);
                (c.clone(), last2.clone())
            }
            _ => {
                let mut c1 = to_center(&detections[0]);
                let mut c2 = to_center(&detections[1]);
                if c2.cx < c1.cx {
                    std::mem::swap(&mut c1, &mut c2);
                }
                (c1, c2)
            }
        };

        // Update last centres for missing detections in subsequent frames.
        if detections.is_empty() {
            // keep previous – already assigned.
        } else {
            last1 = center1.clone();
            last2 = center2.clone();
        }

        let ts = (i as f32 / fps) as f64;
        // First occurrence uses "cut" mode; subsequent frames use "glide".
        let mode1 = if face1_keyframes.is_empty() {
            "cut"
        } else {
            "glide"
        };
        let mode2 = if face2_keyframes.is_empty() {
            "cut"
        } else {
            "glide"
        };

        face1_keyframes.push(crate::face::models::FaceKeyframe {
            timestamp: ts,
            cx: center1.cx,
            cy: center1.cy,
            mode: mode1.to_string(),
        });
        face2_keyframes.push(crate::face::models::FaceKeyframe {
            timestamp: ts,
            cx: center2.cx,
            cy: center2.cy,
            mode: mode2.to_string(),
        });
    }

    // If after processing we have zero keyframes for either face, treat it as a
    // detection failure.
    if face1_keyframes.is_empty() || face2_keyframes.is_empty() {
        return Err(CliptzyError::Config(
            "multi_face mode requires at least two detectable faces".into(),
        ));
    }

    let multi = crate::face::models::MultiFaceData {
        face_1_keyframes: face1_keyframes,
        face_2_keyframes: face2_keyframes,
        tracking_mode,
    };

    // No visual analysis is performed here – we forward the optional analysis
    // from the single‑face pipeline (currently always `None`).
    Ok((multi, None))
}
