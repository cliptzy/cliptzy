use crate::error::CliptzyError;
use std::process::Stdio;
use tokio_util::sync::CancellationToken;

pub struct PipelineStage {
    pub name: String,
    pub cmd: tokio::process::Command,
}

impl PipelineStage {
    pub fn new(name: &str, mut cmd: tokio::process::Command) -> Self {
        cmd.stdin(Stdio::null());
        cmd.stderr(Stdio::piped());
        Self {
            name: name.to_string(),
            cmd,
        }
    }

    pub async fn execute(&mut self, cancel_token: CancellationToken) -> Result<(), CliptzyError> {
        let mut child = self.cmd.spawn().map_err(|e| CliptzyError::FFmpeg {
            code: -1,
            message: format!("Gagal spawn proses {}: {}", self.name, e),
        })?;

        let stderr = child.stderr.take();
        let stderr_handle = if let Some(stderr) = stderr {
            tokio::spawn(async move {
                use tokio::io::AsyncReadExt;
                let mut buffer = Vec::new();
                let mut reader = tokio::io::BufReader::new(stderr);
                let _ = reader.read_to_end(&mut buffer).await;
                let mut s = String::from_utf8_lossy(&buffer).into_owned();
                if s.len() > 4000 {
                    s = s[s.len() - 4000..].to_string();
                }
                s
            })
        } else {
            tokio::spawn(async { String::new() })
        };

        tokio::select! {
            status = child.wait() => {
                let exit_status = status.map_err(|e| CliptzyError::FFmpeg {
                    code: -1,
                    message: format!("Gagal wait proses {}: {}", self.name, e),
                })?;

                let stderr_output = stderr_handle.await.unwrap_or_default();

                if !exit_status.success() {
                    return Err(CliptzyError::FFmpeg {
                        code: exit_status.code().unwrap_or(-1),
                        message: format!("Proses {} gagal: {}", self.name, stderr_output),
                    });
                }
                Ok(())
            }
            _ = cancel_token.cancelled() => {
                log::warn!("Membatalkan proses {}...", self.name);
                let _ = child.kill().await;
                // Attempt OS-level aggressive kill
                #[cfg(target_os = "windows")]
                {
                    let _ = std::process::Command::new("taskkill")
                        .args(&["/F", "/T", "/PID", &child.id().unwrap_or(0).to_string()])
                        .status();
                }
                #[cfg(not(target_os = "windows"))]
                {
                    let _ = std::process::Command::new("kill")
                        .args(&["-9", &child.id().unwrap_or(0).to_string()])
                        .status();
                }
                Err(CliptzyError::Config(format!("Proses {} dibatalkan oleh pengguna", self.name)))
            }
        }
    }
}
