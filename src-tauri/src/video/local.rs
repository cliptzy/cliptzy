use crate::error::CliptzyError;
use rust_ffmpeg::builder::FFmpegBuilder;
use std::path::Path;

#[derive(serde::Deserialize, Debug)]
pub struct SimpleProbeStream {
    pub codec_type: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(serde::Deserialize, Debug)]
pub struct SimpleProbeFormat {
    pub duration: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct SimpleProbeResult {
    pub format: Option<SimpleProbeFormat>,
    #[serde(default)]
    pub streams: Vec<SimpleProbeStream>,
}

pub async fn probe_local_video(
    path: &Path,
) -> Result<SimpleProbeResult, CliptzyError> {
    let mut cmd = tokio::process::Command::new("ffprobe");
    cmd.arg("-v")
        .arg("quiet")
        .arg("-print_format")
        .arg("json")
        .arg("-show_format")
        .arg("-show_streams")
        .arg(path)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    let output = cmd.output().await.map_err(|e| CliptzyError::FFmpeg {
        code: -1,
        message: format!("ffprobe launch error: {}", e),
    })?;

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        return Err(CliptzyError::FFmpeg {
            code: -1,
            message: format!("ffprobe failed: {}", err_msg),
        });
    }

    // Kita gunakan custom struct alih-alih rust_ffprobe yang rentan terhadap parse error (karena strict type check pada atribut yang tidak kita butuhkan)
    let mut probe: serde_json::Value =
        serde_json::from_slice(&output.stdout).map_err(|e| CliptzyError::FFmpeg {
            code: -1,
            message: format!("ffprobe parse error: {}", e),
        })?;

    let format_duration = probe.get("format").and_then(|f| f.get("duration")).and_then(|d| d.as_str()).map(|s| s.to_string());
    
    let mut parsed_streams = Vec::new();
    if let Some(streams) = probe.get_mut("streams").and_then(|s| s.as_array_mut()) {
        for s in streams {
            parsed_streams.push(SimpleProbeStream {
                codec_type: s.get("codec_type").and_then(|c| c.as_str()).map(|s| s.to_string()),
                width: s.get("width").and_then(|w| w.as_u64()).map(|w| w as u32),
                height: s.get("height").and_then(|h| h.as_u64()).map(|h| h as u32),
            });
        }
    }

    Ok(SimpleProbeResult {
        format: Some(SimpleProbeFormat { duration: format_duration }),
        streams: parsed_streams,
    })
}

pub async fn cut_local_segment(
    input_path: &Path,
    start: f64,
    end: f64,
    output_path: &Path,
) -> Result<(), CliptzyError> {
    let mut builder = FFmpegBuilder::new().map_err(|e| CliptzyError::FFmpeg {
        code: -1,
        message: format!("FFmpeg builder error: {}", e),
    })?;

    builder = builder
        .input_path(input_path.to_path_buf())
        .raw_args(vec!["-ss".to_string(), start.to_string()])
        .raw_args(vec!["-to".to_string(), end.to_string()])
        .raw_args(vec!["-c".to_string(), "copy".to_string()])
        .output_path(output_path.to_path_buf());

    log::info!("FFmpeg Local Cut Command: {:?}", builder);

    let process = builder.spawn().await.map_err(|e| CliptzyError::FFmpeg {
        code: -1,
        message: format!("Spawn failed: {}", e),
    })?;

    process.wait().await.map_err(|e| CliptzyError::FFmpeg {
        code: -1,
        message: format!("Process failed: {}", e),
    })?;

    Ok(())
}
