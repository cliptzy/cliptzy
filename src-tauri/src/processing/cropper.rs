use crate::error::CliptzyError;
use crate::processing::ffmpeg::filters::{FilterGraph, FilterNode};
use rust_ffmpeg::builder::FFmpegBuilder;
use std::path::Path;

pub struct OutputConfig {
    pub width: u32,
    pub height: u32,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            width: 1080,
            height: 1920,
        }
    }
}

pub trait CropStrategy: Send + Sync {
    fn name(&self) -> &str;
    
    fn build_command(
        &self,
        input: &Path,
        output: &Path,
        output_config: &OutputConfig,
    ) -> Result<FFmpegBuilder, CliptzyError>;
}

pub struct DefaultCrop;

impl CropStrategy for DefaultCrop {
    fn name(&self) -> &str {
        "default"
    }

    fn build_command(
        &self,
        input: &Path,
        output: &Path,
        output_config: &OutputConfig,
    ) -> Result<FFmpegBuilder, CliptzyError> {
        let mut graph = FilterGraph::new();

        let scale = FilterNode::new("scale")
            .param("w", &format!("'max(iw*{}/ih,{})'", output_config.height, output_config.width))
            .param("h", &format!("'max(ih*{}/iw,{})'", output_config.width, output_config.height))
            .inputs(&["0:v"])
            .outputs(&["scaled"]);

        let crop = FilterNode::new("crop")
            .param("w", &output_config.width.to_string())
            .param("h", &output_config.height.to_string())
            .inputs(&["scaled"])
            .outputs(&["outv"]);

        graph.add_node(scale);
        graph.add_node(crop);

        let hw_accel = crate::processing::ffmpeg::hwaccel::HwAccel::detect(None);

        let mut builder = FFmpegBuilder::new()
            .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("FFmpeg builder error: {}", e) })?;
            
        builder = builder
            .input_path(input.to_path_buf())
            .filter_complex(graph.to_string())
            .raw_args(vec!["-map".to_string(), "[outv]".to_string(), "-map".to_string(), "0:a?".to_string()])
            .raw_args(hw_accel.encode_args())
            .raw_args(vec!["-c:a".to_string(), "aac".to_string()])
            .output_path(output.to_path_buf());

        Ok(builder)
    }
}

pub struct FullCrop;

impl CropStrategy for FullCrop {
    fn name(&self) -> &str {
        "full"
    }

    fn build_command(
        &self,
        input: &Path,
        output: &Path,
        output_config: &OutputConfig,
    ) -> Result<FFmpegBuilder, CliptzyError> {
        let mut graph = FilterGraph::new();

        let bg_scale = FilterNode::new("scale")
            .param("w", &format!("'max(iw*{}/ih,{})'", output_config.height, output_config.width))
            .param("h", &format!("'max(ih*{}/iw,{})'", output_config.width, output_config.height))
            .inputs(&["0:v"])
            .outputs(&["bg_scaled"]);

        let bg_crop = FilterNode::new("crop")
            .param("w", &output_config.width.to_string())
            .param("h", &output_config.height.to_string())
            .inputs(&["bg_scaled"])
            .outputs(&["bg_cropped"]);

        let bg_blur = FilterNode::new("boxblur")
            .param("", "20:5")
            .inputs(&["bg_cropped"])
            .outputs(&["bg"]);

        let fg_scale = FilterNode::new("scale")
            .param("w", &output_config.width.to_string())
            .param("h", "-2")
            .inputs(&["0:v"])
            .outputs(&["fg"]);

        let overlay = FilterNode::new("overlay")
            .param("x", "(W-w)/2")
            .param("y", "(H-h)/2")
            .inputs(&["bg", "fg"])
            .outputs(&["outv"]);

        graph.add_node(bg_scale);
        graph.add_node(bg_crop);
        graph.add_node(bg_blur);
        graph.add_node(fg_scale);
        graph.add_node(overlay);

        let hw_accel = crate::processing::ffmpeg::hwaccel::HwAccel::detect(None);

        let mut builder = FFmpegBuilder::new()
            .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("FFmpeg builder error: {}", e) })?;
            
        builder = builder
            .input_path(input.to_path_buf())
            .filter_complex(graph.to_string())
            .raw_args(vec!["-map".to_string(), "[outv]".to_string(), "-map".to_string(), "0:a?".to_string()])
            .raw_args(hw_accel.encode_args())
            .raw_args(vec!["-c:a".to_string(), "aac".to_string()])
            .output_path(output.to_path_buf());

        Ok(builder)
    }
}

pub fn create_crop_strategy(mode: &str) -> Box<dyn CropStrategy> {
    match mode {
        "default" => Box::new(DefaultCrop),
        "full" => Box::new(FullCrop),
        _ => Box::new(DefaultCrop),
    }
}
