use super::{
    apply_debug_ass,
    finish_crop_builder,
    generate_dynamic_crop_expr,
    CropStrategy,
    OutputConfig,
};
use crate::error::CliptzyError;
use crate::face::models::FaceKeyframe;
use crate::processing::ffmpeg::filters::{FilterGraph, FilterNode};
use rust_ffmpeg::builder::FFmpegBuilder;
use std::path::Path;

/// Multi‑face crop mode used for podcast / interview style videos.
///
/// Layout (portrait 1080×1920):
///   * Top 60 % (1080×1152) – a static centre‑crop of the whole video.
///   * Bottom 40 % (1080×768) – split horizontally into two equal panels (540×768) that
///     track the two faces individually.
///
/// The orchestrator supplies a single slice of keyframes that contains the keyframes
/// for face 1 followed by the keyframes for face 2. The implementation splits this slice
/// into two halves and builds the required filtergraph.
pub struct MultiFaceCrop;

impl CropStrategy for MultiFaceCrop {
    fn name(&self) -> &str {
        "multi_face"
    }

    fn build_command(
        &self,
        input: &Path,
        output: &Path,
        output_config: &OutputConfig,
        keyframes: Option<&[FaceKeyframe]>,
    ) -> Result<FFmpegBuilder, CliptzyError> {
        // -------------------------------------------------------------
        // Validate and split keyframes into two equal halves.
        // -------------------------------------------------------------
        let (face1_kf, face2_kf) = match keyframes {
            Some(kfs) => {
                if kfs.len() % 2 != 0 {
                    return Err(CliptzyError::Config(
                        "multi_face mode requires an even number of keyframes".into(),
                    ));
                }
                let half = kfs.len() / 2;
                (&kfs[0..half], &kfs[half..])
            }
            None => {
                return Err(CliptzyError::Config(
                    "multi_face mode requires face keyframes".into(),
                ));
            }
        };

        // -------------------------------------------------------------
        // Dimensions.
        // -------------------------------------------------------------
        let out_w = output_config.width; // 1080
        let out_h = output_config.height; // 1920
        let top_h = ((out_h as f32) * 0.60).round() as u32; // 1152
        let bottom_h = out_h - top_h; // 768
        let bottom_w = out_w / 2; // 540 per panel

        // -------------------------------------------------------------
        // Build filter graph.
        // -------------------------------------------------------------
        let mut graph = FilterGraph::new();
        let input_v = apply_debug_ass(&mut graph, "0:v", output_config);

        // ---------- Top static centre crop ----------
        let top_scale = FilterNode::new("scale")
            .param(
                "w",
                &format!("'max(iw*{}/ih,{})'", top_h, out_w),
            )
            .param(
                "h",
                &format!("'max(ih*{}/iw,{})'", out_w, top_h),
            )
            .inputs(&[&input_v])
            .outputs(&["top_scaled"]);

        let top_crop = FilterNode::new("crop")
            .param("w", &out_w.to_string())
            .param("h", &top_h.to_string())
            .param(
                "x",
                &format!("max(0\\,min(iw*0.5-({}/2)\\,iw-{}))", out_w, out_w),
            )
            .param(
                "y",
                &format!("max(0\\,min(ih*0.5-({}/2)\\,ih-{}))", top_h, top_h),
            )
            .inputs(&["top_scaled"])
            .outputs(&["top"]);

        // ---------- Bottom left panel (face 1) ----------
        let bl_scale = FilterNode::new("scale")
            .param(
                "w",
                &format!("'max(iw*{}/ih,{})'", bottom_h, bottom_w),
            )
            .param(
                "h",
                &format!("'max(ih*{}/iw,{})'", bottom_w, bottom_h),
            )
            .inputs(&[&input_v])
            .outputs(&["bl_scaled"]);

        let bl_crop = FilterNode::new("crop")
            .param("w", &bottom_w.to_string())
            .param("h", &bottom_h.to_string())
            .param("x", &generate_dynamic_crop_expr(face1_kf, "x", bottom_w, bottom_h))
            .param("y", &generate_dynamic_crop_expr(face1_kf, "y", bottom_w, bottom_h))
            .inputs(&["bl_scaled"])
            .outputs(&["bottom_left"]);

        // ---------- Bottom right panel (face 2) ----------
        let br_scale = FilterNode::new("scale")
            .param(
                "w",
                &format!("'max(iw*{}/ih,{})'", bottom_h, bottom_w),
            )
            .param(
                "h",
                &format!("'max(ih*{}/iw,{})'", bottom_w, bottom_h),
            )
            .inputs(&[&input_v])
            .outputs(&["br_scaled"]);

        let br_crop = FilterNode::new("crop")
            .param("w", &bottom_w.to_string())
            .param("h", &bottom_h.to_string())
            .param("x", &generate_dynamic_crop_expr(face2_kf, "x", bottom_w, bottom_h))
            .param("y", &generate_dynamic_crop_expr(face2_kf, "y", bottom_w, bottom_h))
            .inputs(&["br_scaled"])
            .outputs(&["bottom_right"]);

        // ---------- Combine bottom panels horizontally ----------
        let hstack = FilterNode::new("hstack")
            .inputs(&["bottom_left", "bottom_right"])
            .outputs(&["bottom"]);

        // ---------- Stack top and bottom vertically ----------
        let vstack = FilterNode::new("vstack")
            .inputs(&["top", "bottom"])
            .outputs(&["outv"]);

        // Add nodes to graph.
        graph.add_node(top_scale);
        graph.add_node(top_crop);
        graph.add_node(bl_scale);
        graph.add_node(bl_crop);
        graph.add_node(br_scale);
        graph.add_node(br_crop);
        graph.add_node(hstack);
        graph.add_node(vstack);

        // Build final FFmpeg command.
        let builder = finish_crop_builder(input, output, graph, &output_config.hw_accel)?;
        log::info!("FFmpeg MultiFace Crop Command: {:?}", builder);
        Ok(builder)
    }
}
