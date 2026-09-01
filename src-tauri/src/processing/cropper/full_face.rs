use super::{
    apply_debug_ass, finish_crop_builder, generate_dynamic_crop_expr, CropStrategy, OutputConfig,
};
use crate::error::CliptzyError;
use crate::face::models::FaceKeyframe;
use crate::processing::ffmpeg::filters::{FilterGraph, FilterNode};
use rust_ffmpeg::builder::FFmpegBuilder;
use std::path::Path;

pub struct FullFaceCrop;

impl CropStrategy for FullFaceCrop {
    fn name(&self) -> &str {
        "full_face"
    }

    fn build_command(
        &self,
        input: &Path,
        output: &Path,
        output_config: &OutputConfig,
        keyframes: Option<&[FaceKeyframe]>,
    ) -> Result<FFmpegBuilder, CliptzyError> {
        let mut graph = FilterGraph::new();
        let bottom_h = output_config.height / 3;

        let (x_offset_bottom, y_offset_bottom) = if let Some(kfs) = keyframes {
            (
                generate_dynamic_crop_expr(kfs, "x", output_config.width, bottom_h),
                generate_dynamic_crop_expr(kfs, "y", output_config.width, bottom_h),
            )
        } else {
            (
                format!(
                    "max(0\\,min(iw*0.5-({}/2)\\,iw-{}))",
                    output_config.width, output_config.width
                ),
                format!("max(0\\,min(ih*0.5-({}/2)\\,ih-{}))", bottom_h, bottom_h),
            )
        };

        let input_v = apply_debug_ass(&mut graph, "0:v", output_config);

        let split = FilterNode::new("split")
            .param("", "3")
            .inputs(&[&input_v])
            .outputs(&["orig1", "orig2", "orig_bg"]);

        let bg_scale = FilterNode::new("scale")
            .param("w", &output_config.width.to_string())
            .param("h", &output_config.height.to_string())
            .param("force_original_aspect_ratio", "increase")
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
            .inputs(&["orig1"])
            .outputs(&["top_vid"]);

        let face_scale = FilterNode::new("scale")
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
            .inputs(&["orig2"])
            .outputs(&["scaled"]);

        let face_crop = FilterNode::new("crop")
            .param("w", &output_config.width.to_string())
            .param("h", &bottom_h.to_string())
            .param("x", &x_offset_bottom)
            .param("y", &y_offset_bottom)
            .inputs(&["scaled"])
            .outputs(&["bottom_vid"]);

        let vstack = FilterNode::new("vstack")
            .inputs(&["top_vid", "bottom_vid"])
            .outputs(&["stacked"]);

        let overlay = FilterNode::new("overlay")
            .param("x", "(W-w)/2")
            .param("y", "(H-h)/2")
            .inputs(&["bg", "stacked"])
            .outputs(&["outv"]);

        graph.add_node(split);
        graph.add_node(bg_scale);
        graph.add_node(bg_crop);
        graph.add_node(bg_blur);
        graph.add_node(fg_scale);
        graph.add_node(face_scale);
        graph.add_node(face_crop);
        graph.add_node(vstack);
        graph.add_node(overlay);

        let builder = finish_crop_builder(input, output, graph, &output_config.hw_accel)?;
        log::info!("FFmpeg Crop Command: {:?}", builder);
        Ok(builder)
    }
}
