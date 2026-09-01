use super::{
    apply_debug_ass, finish_crop_builder, generate_dynamic_crop_expr, CropStrategy, OutputConfig,
};
use crate::error::CliptzyError;
use crate::face::models::FaceKeyframe;
use crate::processing::ffmpeg::filters::{FilterGraph, FilterNode};
use rust_ffmpeg::builder::FFmpegBuilder;
use std::path::Path;

pub struct CenterFaceCrop;

impl CropStrategy for CenterFaceCrop {
    fn name(&self) -> &str {
        "center_face"
    }

    fn build_command(
        &self,
        input: &Path,
        output: &Path,
        output_config: &OutputConfig,
        keyframes: Option<&[FaceKeyframe]>,
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

        let x_offset = if let Some(kfs) = keyframes {
            generate_dynamic_crop_expr(kfs, "x", output_config.width, output_config.height)
        } else {
            format!(
                "max(0\\,min(iw*0.5-({}/2)\\,iw-{}))",
                output_config.width, output_config.width
            )
        };

        let y_offset = format!(
            "max(0\\,min(ih*0.5-({}/2)\\,ih-{}))",
            output_config.height, output_config.height
        );

        let crop = FilterNode::new("crop")
            .param("w", &output_config.width.to_string())
            .param("h", &output_config.height.to_string())
            .param("x", &x_offset)
            .param("y", &y_offset)
            .inputs(&["scaled"])
            .outputs(&["outv"]);

        graph.add_node(scale);
        graph.add_node(crop);

        let builder = finish_crop_builder(input, output, graph, &output_config.hw_accel)?;
        log::info!("FFmpeg Crop Command: {:?}", builder);
        Ok(builder)
    }
}
