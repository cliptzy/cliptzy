#[tauri::command]
pub fn get_system_metrics() -> crate::monitor::ProcessMetrics {
    crate::monitor::get_system_metrics()
}

#[tauri::command]
pub async fn get_available_hwaccels() -> Result<Vec<String>, String> {
    let mut accels = vec!["cpu".to_string()];
    
    #[cfg(target_os = "macos")]
    accels.push("mac".to_string());
    
    // Attempt to invoke ffmpeg to detect real accels
    if let Ok(output) = std::process::Command::new("ffmpeg").arg("-hwaccels").output() {
        let text = String::from_utf8_lossy(&output.stdout).to_lowercase();
        if text.contains("videotoolbox") && !accels.contains(&"mac".to_string()) {
            accels.push("mac".to_string());
        }
        if text.contains("cuda") || text.contains("nvenc") || text.contains("cuvid") {
            accels.push("nvidia".to_string());
        }
        if text.contains("amf") || text.contains("d3d11va") {
            accels.push("amd".to_string());
        }
        if text.contains("qsv") {
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
        },
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn clean_output_folder() -> Result<(), String> {
    let output_dir = crate::paths::app_data_dir().join("output");
    
    if output_dir.exists() {
        if let Err(e) = std::fs::remove_dir_all(&output_dir) {
            return Err(format!("Gagal membersihkan folder output: {}", e));
        }
    }
    
    if let Err(e) = std::fs::create_dir_all(&output_dir) {
        return Err(format!("Gagal membuat ulang folder output: {}", e));
    }
    
    Ok(())
}
