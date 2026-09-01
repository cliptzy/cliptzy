use super::{apply_debug_ass, finish_crop_builder, CropStrategy, OutputConfig};
use crate::error::CliptzyError;
use crate::face::models::FaceKeyframe;
use crate::processing::ffmpeg::filters::{FilterGraph, FilterNode};
use rust_ffmpeg::builder::FFmpegBuilder;
use std::path::Path;

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
        _keyframes: Option<&[FaceKeyframe]>,
    ) -> Result<FFmpegBuilder, CliptzyError> {
        let mut graph = FilterGraph::new();
        let input_v = apply_debug_ass(&mut graph, "0:v", output_config);

        let split = FilterNode::new("split")
            .param("", "2")
            .inputs(&[&input_v])
            .outputs(&["orig_bg", "orig_fg"]);

        let bg_scale = FilterNode::new("scale")
            .param(
                "w",
                &format!(
                    "'max(iw*{}/ih,{})'",
                    output_config.height, output_config.width
                ),
            )
            .param(
                "h",
                &format!(
                    "'max(ih*{}/iw,{})'",
                    output_config.width, output_config.height
                ),
            )
            .inputs(&["orig_bg"])
            .outputs(&["bg_scaled"]);

        let bg_crop = FilterNode::new("crop")
            .param("w", &output_config.width.to_string())
            .param("h", &output_config.height.to_string())
            .inputs(&["bg_scaled"])
            .outputs(&["bg_cropped"]);

        let bg_blur = FilterNode::new("boxblur")
            .param("", "20:2")
            .inputs(&["bg_cropped"])
            .outputs(&["bg"]);

        let fg_scale = FilterNode::new("scale")
            .param("w", &output_config.width.to_string())
            .param("h", "-2")
            .inputs(&["orig_fg"])
            .outputs(&["fg"]);

        let overlay = FilterNode::new("overlay")
            .param("x", "(W-w)/2")
            .param("y", "(H-h)/2")
            .inputs(&["bg", "fg"])
            .outputs(&["outv"]);

        graph.add_node(split);
        graph.add_node(bg_scale);
        graph.add_node(bg_crop);
        graph.add_node(bg_blur);
        graph.add_node(fg_scale);
        graph.add_node(overlay);

        let builder = finish_crop_builder(input, output, graph, &output_config.hw_accel)?;
        log::info!("FFmpeg Crop Command: {:?}", builder);
        Ok(builder)
    }
}
