use super::detector::FaceDetectorWrapper;
use super::models::FaceKeyframe;
use crate::error::CliptzyError;
use crate::orchestrator::pipeline::{emit_progress, ProgressEvent};
use image::GenericImageView;
use std::path::Path;
use tokio_util::sync::CancellationToken;

pub fn track_faces_in_frames(
    frame_paths: &[impl AsRef<Path>],
    fps: f32,
    tracking_mode: &str,
    interval_sec: f32,
    detector: &mut FaceDetectorWrapper,
    app_handle: Option<&tauri::AppHandle>,
    cancel_token: &CancellationToken,
    visual_analyzer: Option<&crate::analysis::visual::VisualEmotionAnalyzer>,
) -> Result<
    (
        Vec<FaceKeyframe>,
        Option<Vec<crate::analysis::AnalysisSegment>>,
    ),
    CliptzyError,
> {
    let mut raw_keyframes = Vec::new();
    let mut last_cx = 0.5;
    let mut last_cy = 0.5;
    let total_frames = frame_paths.len();

    let mut prev_pyramid: Option<Vec<image::GrayImage>> = None;
    let mut prev_point: Option<(f32, f32)> = None;
    let mut last_detection_frame = -100;
    let force_detect_interval = (fps * 2.0) as isize;
    let mut analysis_segments = Vec::new();

    struct CropData {
        ts: f64,
        bbox: crate::analysis::BoundingBox,
        image: image::DynamicImage,
    }
    let mut crops_to_analyze = Vec::new();
    let mut last_bbox_dims: Option<(f32, f32)> = None;

    for (i, path) in frame_paths.iter().enumerate() {
        if cancel_token.is_cancelled() {
            return Err(CliptzyError::Cancelled);
        }

        if let Some(app) = app_handle {
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
        let img = match image::open(path.as_ref()) {
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
            if let (Some(prev_pyr), Some(prev_pt), Some(curr_pyr)) =
                (&prev_pyramid, &prev_point, &curr_pyramid)
            {
                let next_res = optical_flow_lk::calc_optical_flow_ex(
                    prev_pyr,
                    curr_pyr,
                    &[*prev_pt],
                    None,
                    15,
                    30,
                    optical_flow_lk::DEFAULT_MIN_EIGEN_THRESHOLD,
                );

                if let Some(res) = next_res.first() {
                    if res.status == optical_flow_lk::TrackStatus::Tracked {
                        if res.pos.0 >= 0.0
                            && res.pos.0 < w as f32
                            && res.pos.1 >= 0.0
                            && res.pos.1 < h as f32
                            && i as isize - last_detection_frame < force_detect_interval
                        {
                            point_tracked = true;
                            prev_point = Some(res.pos);
                            last_cx = res.pos.0 / w as f32;
                            last_cy = res.pos.1 / h as f32;
                            raw_keyframes.push((ts, last_cx, last_cy));

                            if visual_analyzer.is_some() {
                                if let Some((bw, bh)) = last_bbox_dims {
                                    let nx = res.pos.0 - bw * 0.55;
                                    let ny = res.pos.1 - bh * 0.4;
                                    let cropped = img.crop_imm(
                                        nx.max(0.0) as u32,
                                        ny.max(0.0) as u32,
                                        bw as u32,
                                        bh as u32,
                                    );
                                    let resized = cropped.resize_exact(
                                        224,
                                        224,
                                        image::imageops::FilterType::Triangle,
                                    );
                                    crops_to_analyze.push(CropData {
                                        ts: ts as f64,
                                        bbox: crate::analysis::BoundingBox {
                                            x: nx / w as f32,
                                            y: ny / h as f32,
                                            w: bw / w as f32,
                                            h: bh / h as f32,
                                        },
                                        image: resized,
                                    });
                                }
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
                last_bbox_dims = Some((bbox.width() as f32, bbox.height() as f32));

                let px = bbox.x() as f32 + bbox.width() as f32 * 0.55;
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

                if visual_analyzer.is_some() {
                    let cropped = img.crop_imm(
                        bbox.x().max(0) as u32,
                        bbox.y().max(0) as u32,
                        bbox.width().max(0) as u32,
                        bbox.height().max(0) as u32,
                    );
                    let resized =
                        cropped.resize_exact(224, 224, image::imageops::FilterType::Triangle);
                    crops_to_analyze.push(CropData {
                        ts: ts as f64,
                        bbox: crate::analysis::BoundingBox {
                            x: bbox.x() as f32 / w as f32,
                            y: bbox.y() as f32 / h as f32,
                            w: bbox.width() as f32 / w as f32,
                            h: bbox.height() as f32 / h as f32,
                        },
                        image: resized,
                    });
                }
            } else {
                prev_point = None;
                raw_keyframes.push((ts, last_cx, last_cy));
            }
        }

        prev_pyramid = curr_pyramid;
    }

    let keyframes = finalize_keyframes(raw_keyframes, tracking_mode, interval_sec, fps);

    if let Some(analyzer) = visual_analyzer {
        let mut all_probs = Vec::new();
        // Batch size of 32
        for chunk in crops_to_analyze.chunks(32) {
            let images: Vec<_> = chunk.iter().map(|c| c.image.clone()).collect();
            if let Ok(probs_batch) = analyzer.run_batch_inference(&images) {
                all_probs.extend(probs_batch);
            } else {
                all_probs.extend(vec![[0.0; 7]; images.len()]);
            }
        }

        let ema_alpha = 0.3_f32; // Smoothing factor
        let mut smoothed_probs = [0.0_f32; 7];

        for (i, probs) in all_probs.into_iter().enumerate() {
            let data = &crops_to_analyze[i];

            if i == 0 {
                smoothed_probs = probs;
            } else {
                for j in 0..7 {
                    smoothed_probs[j] =
                        ema_alpha * probs[j] + (1.0 - ema_alpha) * smoothed_probs[j];
                }
            }

            let (emotion, score) =
                crate::analysis::visual::VisualEmotionAnalyzer::map_probs_to_emotion(
                    &smoothed_probs,
                );

            analysis_segments.push(crate::analysis::AnalysisSegment {
                start_time: data.ts,
                end_time: data.ts + (1.0 / fps as f64),
                emotion,
                score,
                bounding_box: Some(data.bbox.clone()),
            });
        }
    }

    let opt_analysis = if visual_analyzer.is_some() {
        Some(analysis_segments)
    } else {
        None
    };

    Ok((keyframes, opt_analysis))
}

fn finalize_keyframes(
    raw_keyframes: Vec<(f32, f32, f32)>,
    tracking_mode: &str,
    interval_sec: f32,
    fps: f32,
) -> Vec<FaceKeyframe> {
    if raw_keyframes.is_empty() {
        return Vec::new();
    }

    match tracking_mode {
        "cinematic" => {
            let alpha = 0.15;
            let mut smooth_cx = raw_keyframes[0].1;
            let mut smooth_cy = raw_keyframes[0].2;
            let mut keyframes = Vec::new();

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
            keyframes
        }
        "static" => vec![FaceKeyframe {
            timestamp: 0.0,
            cx: raw_keyframes[0].1,
            cy: raw_keyframes[0].2,
            mode: "cut".to_string(),
        }],
        _ => {
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
            classified
        }
    }
}
