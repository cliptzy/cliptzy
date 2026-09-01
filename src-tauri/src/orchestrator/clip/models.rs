use crate::analysis::AnalysisSegment;
use crate::orchestrator::job_cache::FileFingerprint;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct SegmentTranscriptCacheEntry {
    pub whisper_model: String,
    pub source_fingerprint: FileFingerprint,
    pub segments: Vec<crate::transcription::models::TranscriptionSegment>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct EmotionCacheEntry {
    pub source_fingerprint: FileFingerprint,
    pub segments: Vec<AnalysisSegment>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ClipPayload {
    pub url: String,
    pub video_id: String,
    pub start: f64,
    pub end: f64,
    pub crop_mode: String,
    pub use_subtitle: bool,
    pub cookies_path: Option<String>,
    pub segment_index: u32,
}

#[derive(Serialize, Clone, Debug)]
pub struct ClipResult {
    pub success: bool,
    pub output_path: String,
    pub thumbnail_path: String,
}

pub(crate) struct ClipPaths {
    pub source: PathBuf,
    pub cropped: PathBuf,
    pub final_video: PathBuf,
    pub thumb: PathBuf,
}
