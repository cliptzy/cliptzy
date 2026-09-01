pub fn get_system_gpus() -> Vec<String> {
    let mut gpus = Vec::new();

    #[cfg(target_os = "windows")]
    {
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
