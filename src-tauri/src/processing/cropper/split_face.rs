use super::{
    apply_debug_ass, finish_crop_builder, generate_dynamic_crop_expr, CropStrategy,
    OutputConfig,
};
use crate::error::CliptzyError;
use crate::face::models::FaceKeyframe;
use crate::processing::ffmpeg::filters::{FilterGraph, FilterNode};
use rust_ffmpeg::builder::FFmpegBuilder;
use std::path::Path;

/// Crop strategy that splits the output video vertically:
/// - Top half shows the main video centered‑cropped (9:16).
/// - Bottom half shows a dynamic face‑tracking crop following the streamer.
/// The total output resolution is 1080×1920 (portrait), each half 1080×960.
pub struct SplitFaceCrop;

impl CropStrategy for SplitFaceCrop {
    fn name(&self) -> &str {
        "split_face"
    }

    fn build_command(
        &self,
        input: &Path,
        output: &Path,
        output_config: &OutputConfig,
        keyframes: Option<&[FaceKeyframe]>,
    ) -> Result<FFmpegBuilder, CliptzyError> {
        // Ensure we have face keyframes for the bottom half; otherwise return an error.
        let kfs = match keyframes {
            Some(k) => k,
            None => {
                return Err(CliptzyError::Config(
                    "split_face mode requires face tracking data".into(),
                ));
            }
        };

        let mut graph = FilterGraph::new();
        let input_v = apply_debug_ass(&mut graph, "0:v", output_config);

        // Scale the source video so it fits the output width while preserving aspect ratio.
        let scale = FilterNode::new("scale")
            .param(
                "w",
                &format!("'max(iw*{}/ih,{})'", output_config.height, output_config.width),
            )
            .param(
                "h",
                &format!("'max(ih*{}/iw,{})'", output_config.width, output_config.height),
            )
            .inputs(&[&input_v])
            .outputs(&["scaled"]);

        // ----- Top half (static center crop) -----
        let top_crop = FilterNode::new("crop")
            .param("w", &output_config.width.to_string())
            .param("h", &(output_config.height / 2).to_string())
            .param("x", "0")
            .param("y", "0")
            .inputs(&["scaled"])
            .outputs(&["top"]);

        // ----- Bottom half (dynamic face crop) -----
        let bottom_w = output_config.width;
        let bottom_h = output_config.height / 2;

        let x_offset = generate_dynamic_crop_expr(kfs, "x", bottom_w, bottom_h);
        let y_offset = generate_dynamic_crop_expr(kfs, "y", bottom_w, bottom_h);

        let bottom_crop = FilterNode::new("crop")
            .param("w", &bottom_w.to_string())
            .param("h", &bottom_h.to_string())
            .param("x", &x_offset)
            .param("y", &y_offset)
            .inputs(&["scaled"])
            .outputs(&["bottom"]);

        // Stack the two halves vertically.
        let vstack = FilterNode::new("vstack")
            .inputs(&["top", "bottom"])
            .outputs(&["stacked"]);

        // Assemble graph.
        graph.add_node(scale);
        graph.add_node(top_crop);
        graph.add_node(bottom_crop);
        graph.add_node(vstack);

        // Finish building the FFmpeg command.
        let builder = finish_crop_builder(input, output, graph, &output_config.hw_accel)?;
        log::info!("FFmpeg SplitFaceCrop Command: {:?}", builder);
        Ok(builder)
    }
}
