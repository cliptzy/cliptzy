use super::models::{ClipPaths, ClipPayload};
use crate::processing::cropper::OutputConfig;
use std::path::Path;

pub(crate) fn apply_segment_bounds(payload: &mut ClipPayload, config: &crate::config::models::AppConfig) {
    let min_dur = config.min_duration as f64;
    let padding = config.padding as f64;

    let duration = payload.end - payload.start;
    if duration > 0.0 && duration < min_dur {
        let deficit = min_dur - duration;
        payload.start -= deficit / 2.0;
        payload.end += deficit / 2.0;
    }

    payload.start -= padding;
    payload.end += padding;

    if payload.start < 0.0 {
        let underflow = 0.0 - payload.start;
        payload.start = 0.0;
        payload.end += underflow;
    }
}

pub(crate) fn clip_paths(job_dir: &Path, idx: u32) -> ClipPaths {
    ClipPaths {
        source: job_dir.join(format!("source_{}.mp4", idx)),
        cropped: job_dir.join(format!("cropped_{}.mp4", idx)),
        final_video: job_dir.join(format!("final_{}.mp4", idx)),
        thumb: job_dir.join(format!("thumbnail_{}.jpg", idx)),
    }
}

pub(crate) async fn probe_output_dimensions(
    current_video: &Path,
    mut out_config: OutputConfig,
) -> OutputConfig {
    if let Ok(probe) = crate::video::local::probe_local_video(current_video).await {
        for stream in probe.streams {
            if stream.codec_type == Some("video".to_string()) {
                if let Some(w) = stream.width {
                    out_config.width = w as u32;
                }
                if let Some(h) = stream.height {
                    out_config.height = h as u32;
                }
                break;
            }
        }
    }
    out_config
}
