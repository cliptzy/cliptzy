pub mod audio;
pub mod subtitle;
pub mod vfx;
pub mod watermark;

use crate::error::CliptzyError;
use crate::processing::ffmpeg::filters::FilterGraph;
use rust_ffmpeg::builder::FFmpegBuilder;
use std::path::Path;

pub struct VideoBurnerConfig {
    pub ass_path: Option<String>,
    pub vfx_overlay_path: Option<String>,
    pub normalize_audio: bool,
    pub config: Option<crate::transcription::models::SubtitleConfig>,
    pub watermark_path: Option<String>,
    pub watermark_position: String,
}

pub async fn burn_video_effects(
    input_path: &Path,
    output_path: &Path,
    config: &VideoBurnerConfig,
    progress_info: Option<(&tauri::AppHandle, f64)>,
) -> Result<(), CliptzyError> {
    let mut graph = FilterGraph::new();
    let mut final_v = "0:v".to_string();
    let mut input_idx = 1;

    if let Some(ass_path) = &config.ass_path {
        final_v = subtitle::apply_subtitle(&mut graph, &final_v, ass_path, &config.config);
    }

    if let Some(vfx) = &config.vfx_overlay_path {
        if !vfx.trim().is_empty() {
            final_v = vfx::apply_vfx(&mut graph, &final_v, &mut input_idx);
        }
    }

    if let Some(wm) = &config.watermark_path {
        if !wm.trim().is_empty() {
            final_v = watermark::apply_watermark(
                &mut graph,
                &final_v,
                &mut input_idx,
                &config.watermark_position,
            );
        }
    }

    let mut final_a = "0:a".to_string();
    if config.normalize_audio {
        final_a = audio::apply_normalization(&mut graph);
    }

    let hw_accel = crate::processing::ffmpeg::hwaccel::HwAccel::detect(None);

    let mut builder = FFmpegBuilder::new().map_err(|e| CliptzyError::FFmpeg {
        code: -1,
        message: format!("FFmpeg builder error: {}", e),
    })?;

    builder = builder.input_path(input_path.to_path_buf());

    if let Some(vfx_path) = &config.vfx_overlay_path {
        if !vfx_path.trim().is_empty() {
            builder = builder.input_path(Path::new(vfx_path).to_path_buf());
        }
    }

    if let Some(wm_path) = &config.watermark_path {
        let path_str = wm_path.trim();
        if !path_str.is_empty() {
            let actual_path = if path_str.starts_with("assets/") || path_str.starts_with("assets\\")
            {
                crate::paths::app_data_dir().join(path_str)
            } else {
                std::path::PathBuf::from(path_str)
            };
            builder = builder.input_path(actual_path);
        }
    }

    builder = builder
        .filter_complex(graph.to_string())
        .raw_args(vec![
            "-map".to_string(),
            format!("[{}]", final_v),
            "-map".to_string(),
            format!("[{}]", final_a),
        ])
        .raw_args(hw_accel.encode_args())
        .raw_args(vec!["-c:a".to_string(), "aac".to_string()])
        .output_path(output_path.to_path_buf());

    if let Some((app_handle, total_duration)) = progress_info {
        let handle_clone = app_handle.clone();
        builder = builder.on_progress(move |prog| {
            if let Some(time) = prog.time {
                let current_sec = time.as_secs_f64();
                if total_duration > 0.0 {
                    let mut pct = (current_sec / total_duration) * 100.0;
                    if pct > 99.9 { pct = 99.9; }
                    crate::orchestrator::pipeline::emit_progress(
                        &handle_clone,
                        &crate::orchestrator::pipeline::ProgressEvent {
                            stage: "subtitle".into(),
                            label: format!("Menambahkan efek visual/teks ke video... ({:.1}%)", pct),
                            current: pct as u32,
                            total: 100,
                            detail: None,
                        }
                    );
                }
            }
        });
    }

    let process = builder.spawn().await.map_err(|e| CliptzyError::FFmpeg {
        code: -1,
        message: format!("Spawn failed: {}", e),
    })?;

    process.wait().await.map_err(|e| CliptzyError::FFmpeg {
        code: -1,
        message: format!("Process failed: {}", e),
    })?;

    Ok(())
}
