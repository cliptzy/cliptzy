mod center_face;
mod default;
mod full;
mod full_face;
mod passthrough;

use crate::error::CliptzyError;
use crate::face::models::FaceKeyframe;
use crate::processing::ffmpeg::filters::{FilterGraph, FilterNode};
use crate::processing::ffmpeg::hwaccel::HwAccel;
use rust_ffmpeg::builder::FFmpegBuilder;
use std::path::Path;

pub use center_face::CenterFaceCrop;
pub use default::DefaultCrop;
pub use full::FullCrop;
pub use full_face::FullFaceCrop;
pub use passthrough::PassthroughCrop;

pub struct OutputConfig {
    pub width: u32,
    pub height: u32,
    pub hw_accel: HwAccel,
    pub debug_ass_path: Option<String>,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            width: 1080,
            height: 1920,
            hw_accel: HwAccel::Cpu,
            debug_ass_path: None,
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
        keyframes: Option<&[FaceKeyframe]>,
    ) -> Result<FFmpegBuilder, CliptzyError>;
}

pub fn create_crop_strategy(mode: &str) -> Box<dyn CropStrategy> {
    match mode {
        "none" => Box::new(PassthroughCrop),
        "default" => Box::new(DefaultCrop),
        "full" => Box::new(FullCrop),
        "full_face" => Box::new(FullFaceCrop),
        "center_face" => Box::new(CenterFaceCrop),
        _ => Box::new(DefaultCrop),
    }
}

pub(crate) fn apply_debug_ass(
    graph: &mut FilterGraph,
    input: &str,
    output_config: &OutputConfig,
) -> String {
    if let Some(ass) = &output_config.debug_ass_path {
        let safe_path = ass.replace('\\', "/");
        let escaped_ass = safe_path.replace(':', "\\:");
        let final_ass = format!("'{}'", escaped_ass);

        let sub_node = FilterNode::new("subtitles")
            .param("", &final_ass)
            .inputs(&[input])
            .outputs(&["debug_v"]);
        graph.add_node(sub_node);
        "debug_v".to_string()
    } else {
        input.to_string()
    }
}

pub(crate) fn finish_crop_builder(
    input: &Path,
    output: &Path,
    graph: FilterGraph,
    hw_accel: &HwAccel,
) -> Result<FFmpegBuilder, CliptzyError> {
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

pub(crate) fn generate_dynamic_crop_expr(
    keyframes: &[FaceKeyframe],
    axis: &str,
    crop_w: u32,
    crop_h: u32,
) -> String {
    if keyframes.is_empty() {
        if axis == "x" {
            return format!("max(0\\,min(iw*0.5-({}/2)\\,iw-{}))", crop_w, crop_w);
        }
        return format!("max(0\\,min(ih*0.5-({}/2)\\,ih-{}))", crop_h, crop_h);
    }

    let mut kfs = keyframes.to_vec();
    if kfs.len() > 85 {
        for tol_idx in 1..100 {
            let tol = (tol_idx as f32) * 0.005;
            let Some(first) = kfs.first() else {
                break;
            };
            let mut simplified = vec![first.clone()];
            for i in 1..kfs.len().saturating_sub(1) {
                let Some(prev) = simplified.last() else {
                    break;
                };
                let curr = &kfs[i];
                if (curr.cx - prev.cx).abs() < tol && (curr.cy - prev.cy).abs() < tol {
                    continue;
                }
                simplified.push(curr.clone());
            }
            if let Some(last) = kfs.last() {
                simplified.push(last.clone());
            }
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

    for i in 0..kfs.len().saturating_sub(1) {
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

    if let Some(last) = kfs.last() {
        let last_pos = if is_x { last.cx } else { last.cy };
        terms.push(format!(
            "({})*gte(t\\,{:.3})",
            offset(last_pos, is_x),
            last.timestamp
        ));
    }

    terms.join("+")
}
