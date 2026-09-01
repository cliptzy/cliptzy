use super::{apply_debug_ass, finish_crop_builder, CropStrategy, OutputConfig};
use crate::error::CliptzyError;
use crate::face::models::FaceKeyframe;
use crate::processing::ffmpeg::filters::{FilterGraph, FilterNode};
use rust_ffmpeg::builder::FFmpegBuilder;
use std::path::Path;

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
        _keyframes: Option<&[FaceKeyframe]>,
    ) -> Result<FFmpegBuilder, CliptzyError> {
        let mut graph = FilterGraph::new();
        let input_v = apply_debug_ass(&mut graph, "0:v", output_config);

        let scale = FilterNode::new("scale")
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
            .inputs(&[&input_v])
            .outputs(&["scaled"]);

        let crop = FilterNode::new("crop")
            .param("w", &output_config.width.to_string())
            .param("h", &output_config.height.to_string())
            .inputs(&["scaled"])
            .outputs(&["outv"]);

        graph.add_node(scale);
        graph.add_node(crop);

        finish_crop_builder(input, output, graph, &output_config.hw_accel)
    }
}
