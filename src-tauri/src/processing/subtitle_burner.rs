use crate::error::CliptzyError;
use crate::processing::ffmpeg::filters::{FilterGraph, FilterNode};
use rust_ffmpeg::builder::FFmpegBuilder;
use std::path::Path;

pub struct SubtitleBurnerConfig {
    pub ass_path: String,
    pub vfx_overlay_path: Option<String>,
    pub normalize_audio: bool,
}

pub async fn burn_subtitle(
    input_path: &Path,
    output_path: &Path,
    config: &SubtitleBurnerConfig,
) -> Result<(), CliptzyError> {
    let mut graph = FilterGraph::new();

    // Fix for Windows paths in FFmpeg filter graph:
    // 1. Replace backslashes with forward slashes
    // 2. Escape the drive letter colon
    // 3. Wrap the whole path in single quotes to handle spaces
    let safe_path = config.ass_path.replace("\\", "/");
    let escaped_ass = safe_path.replace(":", "\\:");
    let final_ass = format!("'{}'", escaped_ass);
    
    let sub_node = FilterNode::new("subtitles")
        .param("", &final_ass)
        .inputs(&["0:v"])
        .outputs(&["v_subbed"]);
    graph.add_node(sub_node);

    let mut final_v = "v_subbed";

    if config.vfx_overlay_path.is_some() {
        let chromakey = FilterNode::new("chromakey")
            .param("color", "0x00FF00")
            .param("similarity", "0.2")
            .inputs(&["1:v"])
            .outputs(&["vfx_keyed"]);
        
        let overlay = FilterNode::new("overlay")
            .param("x", "(W-w)/2")
            .param("y", "(H-h)/2")
            .param("eof_action", "pass")
            .inputs(&[final_v, "vfx_keyed"])
            .outputs(&["v_vfx"]);

        graph.add_node(chromakey);
        graph.add_node(overlay);
        final_v = "v_vfx";
    }

    let mut final_a = "0:a";
    if config.normalize_audio {
        let loudnorm = FilterNode::new("loudnorm")
            .param("I", "-16")
            .param("LRA", "11")
            .param("TP", "-1.5")
            .inputs(&["0:a"])
            .outputs(&["a_norm"]);
        
        graph.add_node(loudnorm);
        final_a = "a_norm";
    }

    let hw_accel = crate::processing::ffmpeg::hwaccel::HwAccel::detect(None);

    let mut builder = FFmpegBuilder::new()
        .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("FFmpeg builder error: {}", e) })?;
        
    builder = builder.input_path(input_path.to_path_buf());
    
    if let Some(vfx_path) = &config.vfx_overlay_path {
        builder = builder.input_path(Path::new(vfx_path).to_path_buf());
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
