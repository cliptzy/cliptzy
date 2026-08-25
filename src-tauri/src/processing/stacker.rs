use crate::error::CliptzyError;
use rust_ffmpeg::builder::FFmpegBuilder;
use std::path::{Path, PathBuf};
use std::fs;

pub struct StackerConfig {
    pub intro_path: Option<PathBuf>,
    pub outro_path: Option<PathBuf>,
    pub watermark_path: Option<PathBuf>,
}

pub async fn stack_video(
    main_video: &Path,
    output_path: &Path,
    config: &StackerConfig,
) -> Result<(), CliptzyError> {
    let temp_dir = std::env::temp_dir();
    let concat_file_path = temp_dir.join(format!("cliptzy_concat_{}.txt", uuid::Uuid::new_v4()));
    
    let mut file_content = String::new();
    
    if let Some(intro) = &config.intro_path {
        file_content.push_str(&format!("file '{}'\n", intro.to_string_lossy().replace("'", "'\\''")));
    }
    
    file_content.push_str(&format!("file '{}'\n", main_video.to_string_lossy().replace("'", "'\\''")));
    
    if let Some(outro) = &config.outro_path {
        file_content.push_str(&format!("file '{}'\n", outro.to_string_lossy().replace("'", "'\\''")));
    }
    
    fs::write(&concat_file_path, file_content).map_err(CliptzyError::Io)?;
    
    let builder = FFmpegBuilder::new()
        .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("Builder error: {}", e) })?
        .raw_args(vec!["-f".to_string(), "concat".to_string(), "-safe".to_string(), "0".to_string()])
        .input_path(concat_file_path.clone())
        .raw_args(vec!["-c".to_string(), "copy".to_string()])
        .output_path(output_path.to_path_buf());
        
    let process = builder.spawn().await
        .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("Spawn failed: {}", e) })?;
        
    process.wait().await
        .map_err(|e| CliptzyError::FFmpeg { code: -1, message: format!("Process failed: {}", e) })?;
    
    let _ = fs::remove_file(concat_file_path);
    
    Ok(())
}
