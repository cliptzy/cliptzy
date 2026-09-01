use super::{CropStrategy, OutputConfig};
use crate::error::CliptzyError;
use crate::face::models::FaceKeyframe;
use rust_ffmpeg::builder::FFmpegBuilder;
use std::path::Path;

pub struct PassthroughCrop;

impl CropStrategy for PassthroughCrop {
    fn name(&self) -> &str {
        "none"
    }

    fn build_command(
        &self,
        input: &Path,
        output: &Path,
        _output_config: &OutputConfig,
        _keyframes: Option<&[FaceKeyframe]>,
    ) -> Result<FFmpegBuilder, CliptzyError> {
        let mut builder = FFmpegBuilder::new().map_err(|e| CliptzyError::FFmpeg {
            code: -1,
            message: format!("FFmpeg builder error: {}", e),
        })?;

        builder = builder
            .input_path(input.to_path_buf())
            .raw_args(vec![
                "-c:v".to_string(),
                "copy".to_string(),
                "-c:a".to_string(),
                "copy".to_string(),
            ])
            .output_path(output.to_path_buf());

        log::info!("FFmpeg passthrough tanpa crop: {:?}", builder);
        Ok(builder)
    }
}
