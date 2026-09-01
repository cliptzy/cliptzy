use crate::orchestrator::job_cache::{hash_payload, FileFingerprint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MainAudioExtractionResult {
    pub video_info: crate::video::youtube::VideoAnalysisResult,
    pub main_audio_16k_path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EpicMoment {
    pub start: f64,
    pub end: f64,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RestreamerClip {
    pub restreamer_url: String,
    pub offset: f64,
    pub start: f64,
    pub end: f64,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RestreamerInfo {
    pub video_id: String,
    pub video_url: String,
    pub title: String,
    pub uploader: String,
    pub thumbnail: String,
    pub duration: f64,
    #[serde(default)]
    pub upload_date: Option<String>,
    #[serde(default)]
    pub view_count: Option<u64>,
}

#[derive(Serialize, Clone, Debug)]
pub struct PrepareCompilationResult {
    pub video_info: crate::video::youtube::VideoAnalysisResult,
    pub main_audio_16k_path: String,
    pub epic_moments: Vec<EpicMoment>,
    pub restreamers: Vec<RestreamerInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct VideoInfoCacheEntry {
    pub video_id: String,
    pub info: crate::video::youtube::VideoAnalysisResult,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct TranscriptCacheEntry {
    pub whisper_model: String,
    pub source_fingerprint: FileFingerprint,
    pub segments: Vec<crate::transcription::models::TranscriptionSegment>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct EpicMomentsCacheEntry {
    pub ai_provider: String,
    pub ai_model: String,
    pub transcript_hash: String,
    pub moments: Vec<EpicMoment>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct RestreamerSearchCacheEntry {
    pub query: String,
    #[serde(default)]
    pub queries_hash: String,
    pub min_duration_minutes: u32,
    #[serde(default)]
    pub main_upload_date: Option<String>,
    #[serde(default)]
    pub restreamers: Vec<RestreamerInfo>,
    /// Legacy cache field (URL-only); migrated on read.
    #[serde(default)]
    pub urls: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct MainSegmentFile {
    pub index: usize,
    pub start: f64,
    pub end: f64,
    pub description: String,
    pub wav_path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct MainSegmentsCacheEntry {
    pub main_audio_fingerprint: FileFingerprint,
    pub moments_hash: String,
    pub segments: Vec<MainSegmentFile>,
}

#[derive(Clone, Debug)]
pub(crate) struct PreparedMainSegment {
    pub wav_path: String,
    pub moment: EpicMoment,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct SyncCacheEntry {
    pub restreamer_id: String,
    pub restr_audio_fingerprint: FileFingerprint,
    pub moments_hash: String,
    pub clips: Vec<RestreamerClip>,
}

pub(crate) fn moments_hash(moments: &[EpicMoment]) -> String {
    serde_json::to_string(moments)
        .map(|s| hash_payload(&s))
        .unwrap_or_else(|_| "invalid".to_string())
}

pub(crate) fn extract_youtube_video_id(url: &str) -> String {
    url.split("v=")
        .nth(1)
        .and_then(|s| s.split('&').next())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            url.split('/')
                .last()
                .unwrap_or("unknown")
                .split('?')
                .next()
                .unwrap_or("unknown")
                .to_string()
        })
}
