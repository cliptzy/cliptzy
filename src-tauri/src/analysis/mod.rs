pub mod audio;
pub mod fusion;
pub mod text;
pub mod visual;
pub mod voice;

use serde::{Deserialize, Serialize};

/// Standar klasifikasi emosi lintas-analyzer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EmotionLabel {
    Neutral,
    Happy,
    Angry,
    Shock,
    Fear,
    Sad,
    Confused,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

/// Output universal dari segala macam Analyzer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSegment {
    pub start_time: f64,
    pub end_time: f64,
    pub emotion: EmotionLabel,
    pub score: f32, // Skala confidence 0.0 - 1.0

    /// Diisi jika analyzer berbasis visual (Bounding Box Wajah), None jika teks/audio
    pub bounding_box: Option<BoundingBox>,
}

/// Trait universal untuk semua instrumen Analyzer (Visual, Text, Audio)
#[async_trait::async_trait]
pub trait EmotionAnalyzer: Send + Sync {
    fn name(&self) -> &str;

    /// Analisa media dan mereturn array segmen yang mengandung emosi dominan
    async fn analyze(
        &self,
        input_path: &std::path::Path,
        cancel: &tokio_util::sync::CancellationToken,
        progress: &tokio::sync::broadcast::Sender<crate::orchestrator::pipeline::ProgressEvent>,
    ) -> Result<Vec<AnalysisSegment>, crate::error::CliptzyError>;
}
