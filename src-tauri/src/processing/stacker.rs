use crate::error::CliptzyError;
use std::path::{Path, PathBuf};

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
    let parent_dir = output_path.parent().unwrap_or_else(|| Path::new("."));
    let concat_file_path = parent_dir.join(format!(
        "cliptzy_concat_{}.txt",
        uuid::Uuid::new_v4()
            .to_string()
            .chars()
            .take(8)
            .collect::<String>()
    ));
    let mut file_content = String::new();

    if let Some(intro) = &config.intro_path {
        file_content.push_str(&format!(
            "file '{}'\n",
            intro
                .to_string_lossy()
                .replace('\\', "/")
                .replace('\'', "'\\''")
        ));
    }

    file_content.push_str(&format!(
        "file '{}'\n",
        main_video
            .to_string_lossy()
            .replace('\\', "/")
            .replace('\'', "'\\''")
    ));

    if let Some(outro) = &config.outro_path {
        file_content.push_str(&format!(
            "file '{}'\n",
            outro
                .to_string_lossy()
                .replace('\\', "/")
                .replace('\'', "'\\''")
        ));
    }

    std::fs::write(&concat_file_path, file_content).map_err(CliptzyError::Io)?;

    let ffmpeg_bin = crate::utils::find_executable("ffmpeg")
        .unwrap_or_else(|| std::path::PathBuf::from("ffmpeg"));
    let mut cmd = tokio::process::Command::new(&ffmpeg_bin);
    cmd.arg("-y")
        .arg("-f")
        .arg("concat")
        .arg("-safe")
        .arg("0")
        .arg("-i")
        .arg(&concat_file_path)
        .arg("-c")
        .arg("copy")
        .arg("-movflags")
        .arg("+faststart")
        .arg(output_path);

    log::info!("FFmpeg Stacker Command: {:?}", cmd);

    let output = cmd.output().await.map_err(|e| CliptzyError::FFmpeg {
        code: -1,
        message: format!("Spawn failed: {}", e),
    })?;

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        let _ = std::fs::remove_file(&concat_file_path);
        return Err(CliptzyError::FFmpeg {
            code: output.status.code().unwrap_or(-1),
            message: format!("Stacker failed with {}: {}", output.status, err_msg),
        });
    }

    let _ = std::fs::remove_file(&concat_file_path);
    Ok(())
}
