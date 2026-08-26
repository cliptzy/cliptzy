pub mod subtitle;
pub mod watermark;
pub mod vfx;
pub mod audio;

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
) -> Result<(), CliptzyError> {
    let mut graph = FilterGraph::new();
    let mut final_v = "0:v".to_string();
    let mut input_idx = 1;

    if let Some(ass_path) = &config.ass_path {
        final_v = subtitle::apply_subtitle(&mut graph, &final_v, ass_path, &config.config);
    }
    
    if config.vfx_overlay_path.is_some() {
        final_v = vfx::apply_vfx(&mut graph, &final_v, &mut input_idx);
    }

    if config.watermark_path.is_some() {
        final_v = watermark::apply_watermark(&mut graph, &final_v, &mut input_idx, &config.watermark_position);
    }

    let mut final_a = "0:a".to_string();
    if config.normalize_audio {
        final_a = audio::apply_normalization(&mut graph);
    }

    let hw_accel = crate::processing::ffmpeg::hwaccel::HwAccel::detect(None);

    let mut builder = FFmpegBuilder::new()
        .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("FFmpeg builder error: {}", e) })?;
        
    builder = builder.input_path(input_path.to_path_buf());
    
    if let Some(vfx_path) = &config.vfx_overlay_path {
        builder = builder.input_path(Path::new(vfx_path).to_path_buf());
    }

    if let Some(wm_path) = &config.watermark_path {
        let path_str = wm_path.to_string();
        let actual_path = if path_str.starts_with("assets/") || path_str.starts_with("assets\\") {
            crate::paths::app_data_dir().join(path_str)
        } else {
            std::path::PathBuf::from(path_str)
        };
        builder = builder.input_path(actual_path);
    }

    builder = builder
        .filter_complex(graph.to_string())
        .raw_args(vec!["-map".to_string(), format!("[{}]", final_v), "-map".to_string(), format!("[{}]", final_a)])
        .raw_args(hw_accel.encode_args())
        .raw_args(vec!["-c:a".to_string(), "aac".to_string()])
        .output_path(output_path.to_path_buf());

    let process = builder.spawn().await
        .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("Spawn failed: {}", e) })?;
        
    process.wait().await
        .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("Process failed: {}", e) })?;

    Ok(())
}
