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

use crate::face::models::FaceKeyframe;

pub trait CropStrategy: Send + Sync {
    fn name(&self) -> &str;

    fn build_command(
        &self,
        input: &Path,
        output: &Path,
        output_config: &OutputConfig,
        keyframes: Option<&[FaceKeyframe]>,
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
        _keyframes: Option<&[FaceKeyframe]>,
    ) -> Result<FFmpegBuilder, CliptzyError> {
        let mut graph = FilterGraph::new();

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

        let mut builder = FFmpegBuilder::new().map_err(|e| CliptzyError::FFmpeg {
            code: -1,
            message: format!("FFmpeg builder error: {}", e),
        })?;

        builder = builder
            .input_path(input.to_path_buf())
            .filter_complex(graph.to_string())
            .raw_args(vec![
                "-map".to_string(),
                "[outv]".to_string(),
                "-map".to_string(),
                "0:a?".to_string(),
            ])
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
        _keyframes: Option<&[FaceKeyframe]>,
    ) -> Result<FFmpegBuilder, CliptzyError> {
        let mut graph = FilterGraph::new();

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
            .inputs(&["0:v"])
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

        let mut builder = FFmpegBuilder::new().map_err(|e| CliptzyError::FFmpeg {
            code: -1,
            message: format!("FFmpeg builder error: {}", e),
        })?;

        builder = builder
            .input_path(input.to_path_buf())
            .filter_complex(graph.to_string())
            .raw_args(vec![
                "-map".to_string(),
                "[outv]".to_string(),
                "-map".to_string(),
                "0:a?".to_string(),
            ])
            .raw_args(hw_accel.encode_args())
            .raw_args(vec!["-c:a".to_string(), "aac".to_string()])
            .output_path(output.to_path_buf());

        Ok(builder)
    }
}

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

        let bottom_h = output_config.height / 4;
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

        let split = FilterNode::new("split")
            .param("", "3")
            .inputs(&["0:v"])
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

        let hw_accel = crate::processing::ffmpeg::hwaccel::HwAccel::detect(None);

        let mut builder = FFmpegBuilder::new().map_err(|e| CliptzyError::FFmpeg {
            code: -1,
            message: format!("FFmpeg builder error: {}", e),
        })?;

        builder = builder
            .input_path(input.to_path_buf())
            .filter_complex(graph.to_string())
            .raw_args(vec![
                "-map".to_string(),
                "[outv]".to_string(),
                "-map".to_string(),
                "0:a?".to_string(),
            ])
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
        "full_face" => Box::new(FullFaceCrop),
        _ => Box::new(DefaultCrop),
    }
}

fn generate_dynamic_crop_expr(
    keyframes: &[crate::face::models::FaceKeyframe],
    axis: &str,
    crop_w: u32,
    crop_h: u32,
) -> String {
    if keyframes.is_empty() {
        if axis == "x" {
            return format!("max(0\\,min(iw*0.5-({}/2)\\,iw-{}))", crop_w, crop_w);
        } else {
            return format!("max(0\\,min(ih*0.5-({}/2)\\,ih-{}))", crop_h, crop_h);
        }
    }

    let mut kfs = keyframes.to_vec();
    if kfs.len() > 85 {
        for tol_idx in 1..100 {
            let tol = (tol_idx as f32) * 0.005;
            let mut simplified = vec![kfs[0].clone()];
            for i in 1..kfs.len() - 1 {
                let prev = simplified.last().unwrap();
                let curr = &kfs[i];
                if (curr.cx - prev.cx).abs() < tol && (curr.cy - prev.cy).abs() < tol {
                    continue;
                }
                simplified.push(curr.clone());
            }
            simplified.push(kfs.last().unwrap().clone());
            if simplified.len() <= 85 {
                kfs = simplified;
                break;
            }
        }
    }

    let offset = |pos: f32, is_x: bool| -> String {
        if is_x {
            format!("max(0\\,min(iw*{:.3}-({}/2)\\,iw-{}))", pos, crop_w, crop_w)
        } else {
            format!("max(0\\,min(ih*{:.3}-({}/2)\\,ih-{}))", pos, crop_h, crop_h)
        }
    };

    let offset_dynamic = |pos_expr: &str, is_x: bool| -> String {
        if is_x {
            format!(
                "max(0\\,min(iw*({})-({}/2)\\,iw-{}))",
                pos_expr, crop_w, crop_w
            )
        } else {
            format!(
                "max(0\\,min(ih*({})-({}/2)\\,ih-{}))",
                pos_expr, crop_h, crop_h
            )
        }
    };

    let mut terms = Vec::new();
    let is_x = axis == "x";

    for i in 0..kfs.len() - 1 {
        let curr = &kfs[i];
        let next = &kfs[i + 1];

        let curr_pos = if is_x { curr.cx } else { curr.cy };
        let next_pos = if is_x { next.cx } else { next.cy };

        let time_cond = format!(
            "(gte(t\\,{:.3})*lt(t\\,{:.3}))",
            curr.timestamp, next.timestamp
        );

        if next.mode == "cut" || (next_pos - curr_pos).abs() < 0.01 {
            terms.push(format!("({})*{}", offset(curr_pos, is_x), time_cond));
        } else {
            let dur = next.timestamp - curr.timestamp;
            let progress = format!("((t-{:.3})/{:.3})", curr.timestamp, dur);
            let pos_expr = format!(
                "({:.3}+({:.3}-{:.3})*{})",
                curr_pos, next_pos, curr_pos, progress
            );
            terms.push(format!(
                "({})*{}",
                offset_dynamic(&pos_expr, is_x),
                time_cond
            ));
        }
    }

    let last = kfs.last().unwrap();
    let last_pos = if is_x { last.cx } else { last.cy };
    terms.push(format!(
        "({})*gte(t\\,{:.3})",
        offset(last_pos, is_x),
        last.timestamp
    ));

    terms.join("+")
}
