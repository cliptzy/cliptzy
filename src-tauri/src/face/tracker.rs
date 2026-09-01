use super::detector::FaceDetectorWrapper;
use super::frame_extractor::extract_frames;
use super::tracker_strategy::track_faces_in_frames;
use crate::error::CliptzyError;
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
) -> Result<(Vec<super::models::FaceKeyframe>, Option<Vec<crate::analysis::AnalysisSegment>>), CliptzyError>
{
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

    let mut detector =
        FaceDetectorWrapper::new(&model_path).map_err(CliptzyError::Internal)?;

    let extracted =
        extract_frames(video_path, &tracking_mode, interval_sec, &cancel_token).await?;

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
