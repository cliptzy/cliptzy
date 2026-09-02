#[tauri::command]
pub fn get_system_metrics() -> crate::monitor::ProcessMetrics {
    crate::monitor::get_system_metrics()
}

#[tauri::command]
pub fn check_system_specs() -> crate::monitor::SystemSpecsCheck {
    crate::monitor::check_system_specs()
}

#[tauri::command]
pub fn exit_app(code: i32) {
    std::process::exit(code);
}

#[tauri::command]
pub async fn get_available_hwaccels() -> Result<Vec<String>, String> {
    let mut accels = vec!["cpu".to_string()];

    #[cfg(target_os = "macos")]
    accels.push("mac".to_string());

    let gpus = crate::system::get_system_gpus();
    let has_nvidia = gpus.iter().any(|g| g.contains("nvidia") || g.contains("geforce") || g.contains("quadro") || g.contains("rtx") || g.contains("gtx"));
    let has_amd = gpus.iter().any(|g| g.contains("amd") || g.contains("radeon") || g.contains("rx "));
    let has_intel = gpus.iter().any(|g| g.contains("intel") || g.contains("uhd") || g.contains("iris") || g.contains("arc"));

    // Attempt to invoke ffmpeg to detect real accels
    let ffmpeg_bin = crate::utils::find_executable("ffmpeg").unwrap_or_else(|| std::path::PathBuf::from("ffmpeg"));
    if let Ok(output) = std::process::Command::new(&ffmpeg_bin)
        .arg("-hwaccels")
        .output()
    {
        let text = String::from_utf8_lossy(&output.stdout).to_lowercase();
        if text.contains("videotoolbox") && !accels.contains(&"mac".to_string()) {
            accels.push("mac".to_string());
        }
        if (text.contains("cuda") || text.contains("nvenc") || text.contains("cuvid")) && has_nvidia {
            accels.push("nvidia".to_string());
        }
        if (text.contains("amf") || text.contains("d3d11va")) && has_amd {
            accels.push("amd".to_string());
        }
        if text.contains("qsv") && has_intel {
            accels.push("intel".to_string());
        }
    }

    Ok(accels)
}

fn calculate_size(dir: &std::path::Path) -> std::io::Result<u64> {
    let mut size = 0;
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                size += calculate_size(&path)?;
            } else {
                size += entry.metadata()?.len();
            }
        }
    }
    Ok(size)
}

#[tauri::command]
pub async fn get_output_folder_size() -> Result<f64, String> {
    let output_dir = crate::paths::app_data_dir().join("output");

    if !output_dir.exists() {
        return Ok(0.0);
    }

    match calculate_size(&output_dir) {
        Ok(size) => {
            let gb = size as f64 / (1024.0 * 1024.0 * 1024.0);
            Ok(gb)
        }
        Err(e) => {
            log::error!("Gagal menghitung ukuran folder output: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn clean_output_folder() -> Result<(), String> {
    let output_dir = crate::paths::app_data_dir().join("output");

    if output_dir.exists() {
        if let Err(e) = std::fs::remove_dir_all(&output_dir) {
            let msg = format!("Gagal membersihkan folder output: {}", e);
            log::error!("{}", msg);
            return Err(msg);
        }
    }

    if let Err(e) = std::fs::create_dir_all(&output_dir) {
        let msg = format!("Gagal membuat ulang folder output: {}", e);
        log::error!("{}", msg);
        return Err(msg);
    }

    Ok(())
}

#[tauri::command]
pub async fn cancel_processing(state: tauri::State<'_, crate::AppState>) -> Result<(), String> {
    let mut token_guard = state.cancel_token.lock().await;
    if let Some(token) = token_guard.take() {
        token.cancel();
        
        // Force kill any ffmpeg/yt-dlp to interrupt rust_ffmpeg processes
        crate::utils::kill_processes(&["ffmpeg", "yt-dlp"]);

        log::info!("Proses dihentikan oleh user (CancellationToken trigger & aggressive kill)");
    }
    Ok(())
}

#[tauri::command]
pub fn get_installed_browsers() -> Vec<String> {
    crate::system::get_installed_browsers_list()
}
