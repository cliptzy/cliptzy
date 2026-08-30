use crate::error::CliptzyError;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CompilationSequence {
    pub main_moment_path: String,
    pub reaction_paths: Vec<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct CompileResult {
    pub success: bool,
    pub output_path: String,
}

pub struct CompileVideoUseCase {
    pub job_dir: PathBuf,
    pub hwaccel: crate::processing::ffmpeg::hwaccel::HwAccel,
    pub deps: crate::utils::AppDependencies,
}

/// Format a filesystem path for FFmpeg concat demuxer list files.
/// Windows drive letters must NOT use filter-style `\\:` escaping.
fn format_concat_entry(path: &Path) -> String {
    let normalized = path.to_string_lossy().replace('\\', "/");
    format!("file '{}'", normalized.replace('\'', "'\\''"))
}

impl CompileVideoUseCase {
    pub fn new(job_dir: PathBuf, hwaccel: crate::processing::ffmpeg::hwaccel::HwAccel, deps: crate::utils::AppDependencies) -> Self {
        Self { job_dir, hwaccel, deps }
    }

    pub async fn execute(
        &mut self,
        sequences: Vec<CompilationSequence>,
        output_filename: &str,
        cancel_token: tokio_util::sync::CancellationToken,
    ) -> Result<CompileResult, CliptzyError> {
        let concat_txt_path = self.job_dir.join("concat.txt");
        let mut concat_content = String::new();

        for seq in sequences {
            let main_path = Path::new(&seq.main_moment_path);
            if main_path.exists() {
                concat_content.push_str(&format!("{}\n", format_concat_entry(main_path)));
            } else {
                log::warn!("Momen asli tidak ditemukan: {}", seq.main_moment_path);
            }

            for rx_path in seq.reaction_paths {
                let p = Path::new(&rx_path);
                if p.exists() {
                    concat_content.push_str(&format!("{}\n", format_concat_entry(p)));
                } else {
                    log::warn!("Reaksi tidak ditemukan: {}", rx_path);
                }
            }
        }

        if concat_content.is_empty() {
            return Err(CliptzyError::Config("Tidak ada urutan klip valid untuk dikompilasi".into()));
        }

        std::fs::write(&concat_txt_path, &concat_content)
            .map_err(|e| CliptzyError::Config(format!("Gagal menulis concat.txt: {}", e)))?;

        log::debug!("concat.txt contents:\n{}", concat_content);

        let output_mp4 = self.job_dir.join(output_filename);
        let concat_input = concat_txt_path.to_string_lossy().to_string();

        let mut cmd = tokio::process::Command::new(&self.deps.ffmpeg);
        cmd.arg("-f").arg("concat")
            .arg("-safe").arg("0")
            .arg("-i").arg(&concat_input)
            .arg("-c").arg("copy")
            .arg("-y")
            .arg(output_mp4.to_string_lossy().to_string());

        let mut stage = crate::processing::ffmpeg::runner::PipelineStage::new("Concat Compile", cmd);
        
        if stage.execute(cancel_token.clone()).await.is_err() {
            log::warn!("Stream copy gagal, mencoba fallback re-encode...");
            
            let encoder = self.hwaccel.encoder();
            let encode_args = self.hwaccel.encode_args();

            let mut fallback_cmd = tokio::process::Command::new(&self.deps.ffmpeg);
            fallback_cmd
                .arg("-f").arg("concat")
                .arg("-safe").arg("0")
                .arg("-i").arg(&concat_input)
                .arg("-c:v").arg(encoder);
            for arg in &encode_args {
                fallback_cmd.arg(arg);
            }
            fallback_cmd
                .arg("-c:a").arg("aac")
                .arg("-y")
                .arg(output_mp4.to_string_lossy().to_string());
            
            let mut fallback_stage = crate::processing::ffmpeg::runner::PipelineStage::new("Concat Re-encode", fallback_cmd);
            fallback_stage.execute(cancel_token).await?;
        }

        Ok(CompileResult {
            success: true,
            output_path: output_mp4.to_string_lossy().to_string(),
        })
    }
}
