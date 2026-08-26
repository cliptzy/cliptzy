use crate::error::CliptzyError;
use std::path::Path;
use rust_ffprobe::probe;
use rust_ffmpeg::builder::FFmpegBuilder;

pub async fn probe_local_video(path: &Path) -> Result<rust_ffprobe::types::ProbeResult, CliptzyError> {
    let mut cmd = tokio::process::Command::new("ffprobe");
    cmd.arg("-v").arg("quiet")
       .arg("-print_format").arg("json")
       .arg("-show_format")
       .arg("-show_streams")
       .arg(path)
       .stdin(std::process::Stdio::null())
       .stdout(std::process::Stdio::piped())
       .stderr(std::process::Stdio::piped());

    let output = cmd.output().await
        .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("ffprobe launch error: {}", e) })?;

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        return Err(CliptzyError::FFmpeg { code: -1, message: format!("ffprobe failed: {}", err_msg) });
    }

    let probe: rust_ffprobe::types::ProbeResult = serde_json::from_slice(&output.stdout)
        .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("ffprobe parse error: {}", e) })?;

    Ok(probe)
}

pub async fn cut_local_segment(
    input_path: &Path,
    start: f64,
    end: f64,
    output_path: &Path,
) -> Result<(), CliptzyError> {
    let mut builder = FFmpegBuilder::new()
        .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("FFmpeg builder error: {}", e) })?;
        
    builder = builder
        .input_path(input_path.to_path_buf())
        .raw_args(vec!["-ss".to_string(), start.to_string()])
        .raw_args(vec!["-to".to_string(), end.to_string()])
        .raw_args(vec!["-c".to_string(), "copy".to_string()])
        .output_path(output_path.to_path_buf());
        
    let process = builder.spawn().await
        .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("Spawn failed: {}", e) })?;
        
    process.wait().await
        .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("Process failed: {}", e) })?;
        
    Ok(())
}
