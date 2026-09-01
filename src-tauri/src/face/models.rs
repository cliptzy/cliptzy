use serde::{Deserialize, Serialize};

/// Keyframe for a face with normalized centre coordinates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceKeyframe {
    pub timestamp: f64,
    pub cx: f32,
    pub cy: f32,
    /// "cut" for the first keyframe, "glide" for subsequent frames.
    pub mode: String,
}

/// Simple normalised centre used by the tracker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedCenter {
    pub cx: f32,
    pub cy: f32,
}

/// Keyframes for a single‑face detection pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleFaceData {
    pub face_keyframes: Vec<FaceKeyframe>,
    pub tracking_mode: String,
}

/// Keyframes for a two‑face (podcast) pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiFaceData {
    pub face_1_keyframes: Vec<FaceKeyframe>,
    pub face_2_keyframes: Vec<FaceKeyframe>,
    pub tracking_mode: String,
}

/// Wrapper for either single‑ or multi‑face data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FaceData {
    Single(SingleFaceData),
    Multi(MultiFaceData),
}
