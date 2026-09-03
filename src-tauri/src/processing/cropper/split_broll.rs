use super::{apply_debug_ass, CropStrategy, OutputConfig};
use crate::error::CliptzyError;
use crate::processing::broll_manager::BrollManager;
use crate::processing::ffmpeg::filters::{FilterGraph, FilterNode};
use rust_ffmpeg::builder::FFmpegBuilder;
use std::path::{Path, PathBuf};

/// Crop strategy that splits the output video vertically:
/// - Top half (50%): Main video center-cropped to 1080x960
/// - Bottom half (50%): Random B-roll video scaled/cropped to 1080x960
///
/// The B-roll will loop if shorter than main video, or trim if longer.
/// Total output resolution: 1080×1920 (9:16 portrait).
pub struct SplitBrollCrop;

impl CropStrategy for SplitBrollCrop {
    fn name(&self) -> &str {
        "split_broll"
    }

    fn build_command(
        &self,
        input: &Path,
        output: &Path,
        output_config: &OutputConfig,
        _keyframes: Option<&[crate::face::models::FaceKeyframe]>,
    ) -> Result<FFmpegBuilder, CliptzyError> {
        // Get B-roll directory and pick random asset
        let app_dir = crate::paths::app_data_dir();
        let broll_dir = app_dir.join(&output_config.broll_dir);
        let broll_manager = BrollManager::new(&broll_dir);
        let broll_path = broll_manager.pick_random()?;

        log::info!("SplitBrollCrop: Selected B-roll: {}", broll_path.display());

        // Build filter graph with 2 inputs
        let mut graph = FilterGraph::new();

        // Apply debug ASS if configured
        let main_v = apply_debug_ass(&mut graph, "0:v", output_config);
        let broll_v = "1:v".to_string();

        // Calculate dimensions: split output vertically into 2 equal parts
        let half_height = output_config.height / 2; // 960px for 1920 total

        // TOP HALF: Main video center-cropped to 1080x960
        let main_crop = FilterNode::new("crop")
            .param("w", &output_config.width.to_string())
            .param("h", &half_height.to_string())
            .param("x", &format!("(iw-{})/2", output_config.width))
            .param("y", &format!("(ih-{})/2", half_height))
            .inputs(&[&main_v])
            .outputs(&["main_cropped"]);
        graph.add_node(main_crop);

        // BOTTOM HALF: B-roll scaled and center-cropped to 1080x960
        // Scale to cover the target dimensions while maintaining aspect ratio
        let broll_scale = FilterNode::new("scale")
            .param(
                "w",
                &format!(
                    "if(gt(dar\\,{}/{})\\,{}\\,-2)",
                    output_config.width, half_height, output_config.width
                ),
            )
            .param(
                "h",
                &format!(
                    "if(gt(dar\\,{}/{})\\,-2\\,{})",
                    output_config.width, half_height, half_height
                ),
            )
            .inputs(&[&broll_v])
            .outputs(&["broll_scaled"]);
        graph.add_node(broll_scale);

        let broll_crop = FilterNode::new("crop")
            .param("w", &output_config.width.to_string())
            .param("h", &half_height.to_string())
            .param("x", &format!("(iw-{})/2", output_config.width))
            .param("y", &format!("(ih-{})/2", half_height))
            .inputs(&["broll_scaled"])
            .outputs(&["broll_cropped"]);
        graph.add_node(broll_crop);

        // Vertical stack: main on top, broll on bottom
        let vstack = FilterNode::new("vstack")
            .param("inputs", "2")
            .inputs(&["main_cropped", "broll_cropped"])
            .outputs(&["outv"]);
        graph.add_node(vstack);

        // Build FFmpeg command with 2 inputs
        let mut builder = FFmpegBuilder::new().map_err(|e| CliptzyError::FFmpeg {
            code: -1,
            message: format!("FFmpeg builder error: {}", e),
        })?;

        // Input 0: Main video
        // Input 1: B-roll video (with looping if needed)
        builder = builder
            .input_path(input.to_path_buf())
            .raw_args(self.build_broll_input_args(&broll_path)?)
            .filter_complex(graph.to_string())
            .raw_args(vec![
                "-map".to_string(),
                "[outv]".to_string(),
                "-map".to_string(),
                "0:a?".to_string(), // Audio from main video only
            ])
            .raw_args(output_config.hw_accel.encode_args())
            .raw_args(vec![
                "-c:a".to_string(),
                "aac".to_string(),
                "-shortest".to_string(), // Stop when shortest stream ends (main video)
            ])
            .output_path(output.to_path_buf());

        log::info!(
            "FFmpeg SplitBrollCrop filter_complex: {}",
            graph.to_string()
        );
        Ok(builder)
    }
}

impl SplitBrollCrop {
    /// Build input args for B-roll with looping support
    /// Uses -stream_loop to repeat B-roll if shorter than main video
    fn build_broll_input_args(&self, broll_path: &PathBuf) -> Result<Vec<String>, CliptzyError> {
        // We use -stream_loop -1 for infinite loop, then rely on -shortest to stop
        // This ensures B-roll keeps playing throughout the entire main video
        Ok(vec![
            "-stream_loop".to_string(),
            "-1".to_string(), // Infinite loop
            "-i".to_string(),
            broll_path.to_string_lossy().to_string(),
        ])
    }
}
