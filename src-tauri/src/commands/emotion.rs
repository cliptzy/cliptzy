use crate::analysis::fusion::{EmotionFusion, EmotionTimeline};
use crate::error::CliptzyError;
use serde::Serialize;
use std::path::PathBuf;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

#[derive(Serialize)]
pub struct EmotionAnalysisResult {
    pub success: bool,
    pub timeline: Option<EmotionTimeline>,
    pub message: String,
}

#[tauri::command]
pub async fn analyze_emotions(
    app: tauri::AppHandle,
    video_path: String,
    audio_path: Option<String>,
    transcript_path: Option<String>,
) -> Result<EmotionAnalysisResult, CliptzyError> {
    log::info!("Starting standalone emotion analysis for {}", video_path);

    let video_p = PathBuf::from(&video_path);
    let audio_p = audio_path.map(PathBuf::from).unwrap_or_else(|| {
        let mut p = video_p.clone();
        p.set_extension("wav");
        p
    });
    let transcript_p = transcript_path.map(PathBuf::from).unwrap_or_else(|| {
        let mut p = video_p.clone();
        p.set_extension("json");
        p
    });

    let (progress_tx, mut progress_rx) = broadcast::channel(16);
    let cancel_token = CancellationToken::new();

    // Spawn progress listener
    let app_handle = app.clone();
    tokio::spawn(async move {
        while let Ok(event) = progress_rx.recv().await {
            let _ = crate::orchestrator::pipeline::emit_progress(&app_handle, &event);
        }
    });

    let app_config = crate::config::models::AppConfig::load().unwrap_or_default();

    let fusion = EmotionFusion::new();
    match fusion
        .analyze_fusion(
            &video_p,
            &audio_p,
            &transcript_p,
            &app_config.ai,
            &cancel_token,
            &progress_tx,
        )
        .await
    {
        Ok(timeline) => Ok(EmotionAnalysisResult {
            success: true,
            timeline: Some(timeline),
            message: "Emotion analysis completed successfully".into(),
        }),
        Err(e) => {
            log::error!("Emotion analysis failed: {}", e);
            Err(e)
        }
    }
}
