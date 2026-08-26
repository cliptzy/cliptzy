use crate::error::CliptzyError;
use crate::video::youtube::{analyze_youtube_video, SegmentInfo};
use crate::video::local::probe_local_video;
use serde::Serialize;
use std::path::Path;

#[derive(Serialize)]
pub struct ScanResult {
    pub video_id: String,
    pub title: String,
    pub thumbnail: String,
    pub duration: f64,
    pub segments: Vec<SegmentInfo>,
    pub stream_url: Option<String>,
}

#[tauri::command]
pub async fn scan_video(url: String, cookies_path: Option<String>) -> Result<ScanResult, CliptzyError> {
    if url.starts_with("http") || url.starts_with("www") {
        // YouTube video
        let analysis = analyze_youtube_video(&url, cookies_path).await
            .map_err(|e| CliptzyError::Download(e))?;
            
        Ok(ScanResult {
            video_id: analysis.video_id,
            title: analysis.title,
            thumbnail: analysis.thumbnail,
            duration: analysis.duration,
            segments: analysis.segments,
            stream_url: analysis.stream_url,
        })
    } else {
        // Local video
        let path = Path::new(&url);
        if !path.exists() {
            return Err(CliptzyError::FileNotFound(url));
        }
        
        let probe = probe_local_video(path).await?;
        let duration = probe.format.and_then(|f| f.duration).unwrap_or("0".to_string()).parse::<f64>().unwrap_or(0.0);
        
        let title = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        let config = crate::config::models::AppConfig::load().unwrap_or_default();
        let min_duration = config.min_duration as f64;
        
        // Generate sequential segments based on min_duration
        let mut segments = Vec::new();
        let mut start: f64 = 0.0;
        let segment_length: f64 = if min_duration > 0.0 { min_duration } else { 60.0 };
        
        while start < duration {
            let end = f64::min(start + segment_length, duration);
            if end - start > 10.0 {
                segments.push(SegmentInfo {
                    start,
                    end,
                    score: 1.0,
                });
            }
            start += segment_length;
        }
        
        Ok(ScanResult {
            video_id: "local".to_string(),
            title,
            thumbnail: "".to_string(),
            duration,
            segments,
            stream_url: Some(url),
        })
    }
}
