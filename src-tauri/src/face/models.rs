use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceKeyframe {
    pub timestamp: f64,
    pub cx: f32,
    pub cy: f32,
    pub mode: String, // "cut" or "glide"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedCenter {
    pub cx: f32,
    pub cy: f32,
}
