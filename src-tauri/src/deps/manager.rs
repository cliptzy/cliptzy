use crate::error::CliptzyError;
use crate::utils::find_executable;
use reqwest::Client;
use serde::Serialize;
use std::io::{Cursor, Write};
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};

#[derive(Clone, Debug)]
pub struct AppDependencies {
    pub ytdlp: PathBuf,
    pub ffmpeg: PathBuf,
}

impl AppDependencies {
    pub fn check() -> Result<Self, String> {
        let ytdlp = find_executable("yt-dlp")
            .ok_or_else(|| "Binary yt-dlp tidak ditemukan di PATH atau folder bin".to_string())?;
        let ffmpeg = find_executable("ffmpeg")
            .ok_or_else(|| "Binary ffmpeg tidak ditemukan di PATH atau folder bin".to_string())?;

        Ok(Self { ytdlp, ffmpeg })
    }
}

#[derive(Serialize, Clone)]
pub struct DependencyProgress {
    pub step: String,
    pub progress: f32, // 0.0 to 100.0
}

#[derive(Serialize)]
pub struct DependencyStatus {
    pub ffmpeg_installed: bool,
    pub ffmpeg_version: String,
    pub deno_installed: bool,
    pub deno_version: String,
    pub ytdlp_installed: bool,
    pub ytdlp_version: String,
}

pub fn setup_env() {
    let app_dir = crate::paths::app_data_dir();
    let bin_dir = app_dir.join("bin");

    if let Some(path) = std::env::var_os("PATH") {
        let mut paths = std::env::split_paths(&path).collect::<Vec<_>>();
        paths.retain(|p| p != &bin_dir);
        paths.insert(0, bin_dir.clone());
        if let Ok(new_path) = std::env::join_paths(paths) {
            std::env::set_var("PATH", new_path);
        }
    } else {
        std::env::set_var("PATH", bin_dir);
    }
}

#[tauri::command]
pub async fn check_dependencies() -> Result<DependencyStatus, CliptzyError> {
    log::debug!("Memeriksa dependensi sistem (FFmpeg, Deno, yt-dlp)");
    let app_dir = crate::paths::app_data_dir();
    let bin_dir = app_dir.join("bin");

    let ffmpeg_bin = find_executable("ffmpeg").or_else(|| {
        #[cfg(target_os = "windows")]
        let p = bin_dir.join("ffmpeg.exe");
        #[cfg(not(target_os = "windows"))]
        let p = bin_dir.join("ffmpeg");
        if p.exists() { Some(p) } else { None }
    });

    let deno_bin = find_executable("deno").or_else(|| {
        #[cfg(target_os = "windows")]
        let p = bin_dir.join("deno.exe");
        #[cfg(not(target_os = "windows"))]
        let p = bin_dir.join("deno");
        if p.exists() { Some(p) } else { None }
    });

    let ytdlp_bin = find_executable("yt-dlp").or_else(|| {
        #[cfg(target_os = "windows")]
        let p = bin_dir.join("yt-dlp.exe");
        #[cfg(not(target_os = "windows"))]
        let p = bin_dir.join("yt-dlp");
        if p.exists() { Some(p) } else { None }
    });

    let mut ffmpeg_installed = false;
    let mut ffmpeg_version = "Tidak terpasang".to_string();

    let mut deno_installed = false;
    let mut deno_version = "Tidak terpasang".to_string();

    let mut ytdlp_installed = false;
    let mut ytdlp_version = "Tidak terpasang".to_string();

    if let Some(ref path) = ffmpeg_bin {
        log::debug!("Ditemukan binary FFmpeg di {:?}", path);
        if let Ok(output) = tokio::process::Command::new(path)
            .arg("-version")
            .output()
            .await
        {
            if output.status.success() {
                ffmpeg_installed = true;
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Some(first_line) = stdout.lines().next() {
                    ffmpeg_version = first_line
                        .split("Copyright")
                        .next()
                        .unwrap_or("")
                        .replace("ffmpeg version", "")
                        .trim()
                        .to_string();
                } else {
                    ffmpeg_version = "Terpasang (Versi tidak diketahui)".to_string();
                }
                log::info!("FFmpeg terdeteksi: {}", ffmpeg_version);
            }
        }
    } else {
        log::warn!("Binary FFmpeg tidak ditemukan.");
    }

    if let Some(ref path) = deno_bin {
        log::debug!("Ditemukan binary Deno di {:?}", path);
        if let Ok(output) = tokio::process::Command::new(path)
            .arg("--version")
            .output()
            .await
        {
            if output.status.success() {
                deno_installed = true;
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Some(first_line) = stdout.lines().next() {
                    deno_version = first_line.replace("deno", "").trim().to_string();
                } else {
                    deno_version = "Terpasang".to_string();
                }
                log::info!("Deno terdeteksi: {}", deno_version);
            }
        }
    } else {
        log::warn!("Binary Deno tidak ditemukan.");
    }

    if let Some(ref path) = ytdlp_bin {
        log::debug!("Ditemukan binary yt-dlp di {:?}", path);
        if let Ok(output) = tokio::process::Command::new(path)
            .arg("--version")
            .output()
            .await
        {
            if output.status.success() {
                ytdlp_installed = true;
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Some(first_line) = stdout.lines().next() {
                    ytdlp_version = first_line.trim().to_string();
                } else {
                    ytdlp_version = "Terpasang".to_string();
                }
                log::info!("yt-dlp terdeteksi: {}", ytdlp_version);
            }
        }
    } else {
        log::warn!("Binary yt-dlp tidak ditemukan.");
    }

    Ok(DependencyStatus {
        ffmpeg_installed,
        ffmpeg_version,
        deno_installed,
        deno_version,
        ytdlp_installed,
        ytdlp_version,
    })
}

#[tauri::command]
pub async fn install_dependencies(app: AppHandle) -> Result<(), CliptzyError> {
    log::info!("Memulai proses instalasi dependensi...");
    let app_dir = crate::paths::app_data_dir();
    let bin_dir = app_dir.join("bin");
    std::fs::create_dir_all(&bin_dir)?;
    log::debug!("Direktori bin diatur ke {:?}", bin_dir);

    let emit_progress = |step: &str, progress: f32| {
        log::info!("Progres Instalasi: {} ({}%)", step, progress);
        let _ = app.emit(
            "deps-progress",
            DependencyProgress {
                step: step.to_string(),
                progress,
            },
        );
    };

    emit_progress("Memulai instalasi...", 0.0);

    let client = Client::new();

    #[cfg(target_os = "windows")]
    let deno_url =
        "https://github.com/denoland/deno/releases/latest/download/deno-x86_64-pc-windows-msvc.zip";
    #[cfg(target_os = "macos")]
    let deno_url =
        "https://github.com/denoland/deno/releases/latest/download/deno-x86_64-apple-darwin.zip";
    #[cfg(target_os = "linux")]
    let deno_url = "https://github.com/denoland/deno/releases/latest/download/deno-x86_64-unknown-linux-gnu.zip";

    emit_progress("Mengunduh Deno...", 2.0);
    let mut res = client
        .get(deno_url)
        .send()
        .await
        .map_err(|e| CliptzyError::Download(e.to_string()))?;
    let mut buffer = Vec::new();
    while let Some(chunk) = res
        .chunk()
        .await
        .map_err(|e| CliptzyError::Download(e.to_string()))?
    {
        buffer.write_all(&chunk)?;
    }

    emit_progress("Mengekstrak Deno...", 5.0);
    let reader = Cursor::new(buffer);
    let mut archive = zip::ZipArchive::new(reader).map_err(|e| {
        CliptzyError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        ))
    })?;
    for i in 0..archive.len() {
        let mut file = match archive.by_index(i) {
            Ok(f) => f,
            Err(e) => {
                log::warn!("Gagal membaca entri arsip {}: {}", i, e);
                continue;
            }
        };
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        let file_name = outpath.file_name().unwrap_or_default().to_string_lossy();
        if file_name.ends_with("deno.exe") || file_name.ends_with("deno") {
            let dest = bin_dir.join(file_name.as_ref());
            let mut outfile = std::fs::File::create(&dest)?;
            std::io::copy(&mut file, &mut outfile)?;

            #[cfg(not(target_os = "windows"))]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&dest)?.permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(&dest, perms)?;
            }
        }
    }

    #[cfg(target_os = "windows")]
    let ffmpeg_url = "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip";
    #[cfg(target_os = "macos")]
    let ffmpeg_url = "https://evermeet.cx/ffmpeg/getrelease/zip"; // Note: this is just ffmpeg, might need ffprobe too
    #[cfg(target_os = "linux")]
    let ffmpeg_url = "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-linux64-gpl.tar.xz";

    emit_progress("Mengunduh FFmpeg...", 10.0);

    let mut res = client
        .get(ffmpeg_url)
        .send()
        .await
        .map_err(|e| CliptzyError::Download(e.to_string()))?;

    let total_size = res.content_length().unwrap_or(100_000_000) as f32;
    let mut downloaded: f32 = 0.0;

    let mut buffer = Vec::new();
    while let Some(chunk) = res
        .chunk()
        .await
        .map_err(|e| CliptzyError::Download(e.to_string()))?
    {
        buffer.write_all(&chunk)?;
        downloaded += chunk.len() as f32;
        let p = 10.0 + (downloaded / total_size) * 60.0;
        emit_progress("Mengunduh FFmpeg...", p.min(70.0));
    }

    emit_progress("Mengekstrak FFmpeg...", 75.0);

    if ffmpeg_url.ends_with(".zip") {
        let reader = Cursor::new(buffer);
        let mut archive = zip::ZipArchive::new(reader).map_err(|e| {
            CliptzyError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;

        for i in 0..archive.len() {
            let mut file = match archive.by_index(i) {
                Ok(f) => f,
                Err(e) => {
                    log::warn!("Gagal membaca entri arsip {}: {}", i, e);
                    continue;
                }
            };
            let outpath = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            let file_name = outpath.file_name().unwrap_or_default().to_string_lossy();
            if file_name.ends_with("ffmpeg.exe")
                || file_name.ends_with("ffprobe.exe")
                || file_name.ends_with("ffmpeg")
                || file_name.ends_with("ffprobe")
            {
                let dest = bin_dir.join(file_name.as_ref());
                let mut outfile = std::fs::File::create(&dest)?;
                std::io::copy(&mut file, &mut outfile)?;

                #[cfg(not(target_os = "windows"))]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = std::fs::metadata(&dest)?.permissions();
                    perms.set_mode(0o755);
                    std::fs::set_permissions(&dest, perms)?;
                }
            }
        }
    } else {
        // Simple fallback for linux
        emit_progress(
            "Ekstraksi Linux belum diimplementasi (Gunakan apt install ffmpeg).",
            80.0,
        );
    }

    emit_progress("Mengunduh & memasang yt-dlp...", 85.0);
    crate::utils::kill_processes(&["yt-dlp"]);
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let installer = yt_dlp::client::deps::LibraryInstaller::new(bin_dir.clone());
    let ytdlp_path = installer
        .install_youtube(None)
        .await
        .map_err(|e| CliptzyError::Download(format!("Gagal memasang yt-dlp: {}", e)))?;
    log::info!("yt-dlp berhasil dipasang di {:?}", ytdlp_path);

    emit_progress("Selesai!", 100.0);

    Ok(())
}

#[tauri::command]
pub async fn install_ytdlp(app: AppHandle) -> Result<(), CliptzyError> {
    log::info!("Memulai proses instalasi yt-dlp...");
    let app_dir = crate::paths::app_data_dir();
    let bin_dir = app_dir.join("bin");
    std::fs::create_dir_all(&bin_dir)?;

    let emit_progress = |step: &str, progress: f32| {
        log::info!("Progres Instalasi yt-dlp: {} ({}%)", step, progress);
        let _ = app.emit(
            "deps-progress",
            DependencyProgress {
                step: step.to_string(),
                progress,
            },
        );
    };

    emit_progress("Menyiapkan instalasi yt-dlp...", 10.0);

    crate::utils::kill_processes(&["yt-dlp"]);
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    emit_progress("Mengunduh yt-dlp binary...", 40.0);

    let installer = yt_dlp::client::deps::LibraryInstaller::new(bin_dir.clone());
    let path = installer
        .install_youtube(None)
        .await
        .map_err(|e| CliptzyError::Download(format!("Gagal memasang yt-dlp: {}", e)))?;

    log::info!("yt-dlp berhasil dipasang di {:?}", path);
    emit_progress("yt-dlp berhasil dipasang!", 100.0);

    Ok(())
}

