use std::path::PathBuf;

pub fn find_executable(name: &str) -> Option<PathBuf> {
    if let Ok(path) = which::which(name) {
        return Some(path);
    }

    let app_dir = crate::paths::app_data_dir();
    let local_bin = if cfg!(target_os = "windows") {
        if name.ends_with(".exe") {
            app_dir.join("bin").join(name)
        } else {
            app_dir.join("bin").join(format!("{}.exe", name))
        }
    } else {
        if name.ends_with(".app") {
            app_dir.join("/Applications").join(name)
        } else {
            app_dir.join("bin").join(name)
        }
    };

    if local_bin.exists() {
        return Some(local_bin);
    }

    None
}

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

pub fn get_system_gpus() -> Vec<String> {
    let mut gpus = Vec::new();

    #[cfg(target_os = "windows")]
    {
        // Try WMI first via powershell (safest and most reliable on Windows)
        if let Ok(output) = std::process::Command::new("powershell")
            .args(&["-Command", "Get-CimInstance -ClassName Win32_VideoController | Select-Object -ExpandProperty Name"])
            .output()
        {
            let text = String::from_utf8_lossy(&output.stdout).to_lowercase();
            for line in text.lines() {
                let gpu_name = line.trim();
                if !gpu_name.is_empty() {
                    gpus.push(gpu_name.to_string());
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = std::process::Command::new("system_profiler")
            .arg("SPDisplaysDataType")
            .output()
        {
            let text = String::from_utf8_lossy(&output.stdout).to_lowercase();
            // Just push the whole text for macos, we just need to check if 'apple' or 'intel' is in it
            gpus.push(text.to_string());
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Ok(output) = std::process::Command::new("lspci").output() {
            let text = String::from_utf8_lossy(&output.stdout).to_lowercase();
            for line in text.lines() {
                if line.contains("vga compatible controller") || line.contains("3d controller") {
                    gpus.push(line.to_string());
                }
            }
        }
    }

    gpus
}

pub fn get_installed_browsers_list() -> Vec<String> {
    let mut browsers = Vec::new();
    let targets = vec![
        (
            "chrome",
            vec!["chrome", "google-chrome", "chrome.exe", "Google Chrome.app"],
        ),
        (
            "edge",
            vec![
                "msedge",
                "msedge.exe",
                "microsoft-edge",
                "Microsoft Edge.app",
            ],
        ),
        ("firefox", vec!["firefox", "firefox.exe", "Firefox.app"]),
        (
            "brave",
            vec!["brave", "brave-browser", "brave.exe", "Brave.app"],
        ),
        ("opera", vec!["opera", "opera.exe", "Opera.app"]),
        ("vivaldi", vec!["vivaldi", "vivaldi.exe", "Vivaldi.app"]),
        ("safari", vec!["Safari.app"]),
    ];

    for (name, aliases) in targets {
        for alias in aliases {
            if find_executable(alias).is_some() {
                browsers.push(name.to_string());
                break;
            }
        }
    }

    // For Windows, fall back to checking AppData paths if which fails, because browsers
    // are often not in PATH.
    #[cfg(target_os = "windows")]
    {
        let local_app_data = std::env::var("LOCALAPPDATA").unwrap_or_default();
        let roaming_app_data = std::env::var("APPDATA").unwrap_or_default();

        let extra_checks = vec![
            (
                "chrome",
                format!("{}\\Google\\Chrome\\User Data", local_app_data),
            ),
            (
                "edge",
                format!("{}\\Microsoft\\Edge\\User Data", local_app_data),
            ),
            (
                "firefox",
                format!("{}\\Mozilla\\Firefox\\Profiles", roaming_app_data),
            ),
            (
                "brave",
                format!(
                    "{}\\BraveSoftware\\Brave-Browser\\User Data",
                    local_app_data
                ),
            ),
            (
                "opera",
                format!("{}\\Opera Software\\Opera Stable", roaming_app_data),
            ),
            ("vivaldi", format!("{}\\Vivaldi\\User Data", local_app_data)),
        ];

        for (name, path_str) in extra_checks {
            if !browsers.contains(&name.to_string()) && std::path::Path::new(&path_str).exists() {
                browsers.push(name.to_string());
            }
        }
    }

    browsers
}

pub fn kill_processes(names: &[&str]) {
    for name in names {
        #[cfg(not(target_os = "windows"))]
        {
            let _ = tokio::process::Command::new("killall").arg(name).spawn();
        }

        #[cfg(target_os = "windows")]
        {
            let exe_name = format!("{}.exe", name);
            let _ = tokio::process::Command::new("taskkill")
                .args(&["/IM", &exe_name, "/F"])
                .spawn();
        }
    }
}

pub async fn ensure_model_downloaded(
    file_name: &str,
    url: &str,
) -> Result<PathBuf, String> {
    let model_dir = crate::paths::app_data_dir().join("models");
    std::fs::create_dir_all(&model_dir).map_err(|e| format!("Failed to create models dir: {}", e))?;
    
    let model_path = model_dir.join(file_name);
    
    if !model_path.exists() {
        log::info!("Model not found. Downloading {} to {:?}", file_name, model_path);
        let response = reqwest::get(url)
            .await
            .map_err(|e| format!("Download failed for {}: {}", file_name, e))?;
            
        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read bytes for {}: {}", file_name, e))?;
            
        std::fs::write(&model_path, bytes)
            .map_err(|e| format!("Write failed for {}: {}", file_name, e))?;
            
        log::info!("Model {} downloaded successfully.", file_name);
    }
    
    Ok(model_path)
}
