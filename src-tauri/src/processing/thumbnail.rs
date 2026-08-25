use crate::error::CliptzyError;
use rust_ffmpeg::builder::FFmpegBuilder;
use std::path::Path;

pub async fn generate_thumbnail(
    video_path: &Path,
    output_path: &Path,
    time_offset: f64,
) -> Result<(), CliptzyError> {
    let builder = FFmpegBuilder::new()
        .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("Builder error: {}", e) })?
        .raw_args(vec!["-ss".to_string(), time_offset.to_string()])
        .input_path(video_path.to_path_buf())
        .raw_args(vec!["-vframes".to_string(), "1".to_string(), "-q:v".to_string(), "2".to_string()])
        .output_path(output_path.to_path_buf());

    let process = builder.spawn().await
        .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("Spawn failed: {}", e) })?;
        
    process.wait().await
        .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("Process failed: {}", e) })?;

    Ok(())
}

pub async fn generate_compilation_thumbnail(
    _clip_paths: &[&Path],
    _output_path: &Path,
) -> Result<(), CliptzyError> {
    // 2x2 grid collage 
    Ok(())
}
