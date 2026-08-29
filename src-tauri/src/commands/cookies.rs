use std::time::{SystemTime, UNIX_EPOCH};

#[tauri::command]
pub async fn copy_cookies_file(source_path: String) -> Result<String, String> {
    let app_dir = crate::paths::app_data_dir();
    let cred_dir = app_dir.join("cred");
    std::fs::create_dir_all(&cred_dir).map_err(|e| e.to_string())?;

    let dest_path = cred_dir.join("cookies.txt");
    std::fs::copy(&source_path, &dest_path).map_err(|e| e.to_string())?;

    Ok(dest_path.to_string_lossy().to_string())
}

/// Validates a cookies.txt file by checking:
/// 1. File exists at the given path
/// 2. File is in Netscape cookies.txt format (tab-separated, 7 fields)
/// 3. At least one YouTube-related cookie is not expired
#[tauri::command]
pub async fn validate_cookies_file(cookies_path: String) -> Result<serde_json::Value, String> {
    let path = std::path::Path::new(&cookies_path);

    // Check 1: File exists
    if !path.exists() {
        return Ok(serde_json::json!({
            "valid": false,
            "reason": "file_not_found",
            "message": "File cookies tidak ditemukan di path yang diberikan"
        }));
    }

    // Check 2 & 3: Read and parse cookies
    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Gagal membaca file cookies: {}", e))?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    let youtube_domains = [".youtube.com", ".google.com", "youtube.com", "google.com"];

    let mut total_cookies = 0u32;
    let mut youtube_cookies = 0u32;
    let mut valid_youtube_cookies = 0u32;
    let mut has_valid_format = false;

    for line in content.lines() {
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let fields: Vec<&str> = line.split('\t').collect();

        // Netscape cookie format: domain, flag, path, secure, expiration, name, value
        if fields.len() < 7 {
            continue;
        }

        has_valid_format = true;
        total_cookies += 1;

        let domain = fields[0].to_lowercase();
        let is_youtube = youtube_domains.iter().any(|d| domain.contains(d));

        if !is_youtube {
            continue;
        }

        youtube_cookies += 1;

        // Check expiration (field index 4)
        // "0" means session cookie (valid until browser closes, treat as valid)
        let expiry_str = fields[4].trim();
        if expiry_str == "0" {
            valid_youtube_cookies += 1;
            continue;
        }

        if let Ok(expiry) = expiry_str.parse::<u64>() {
            if expiry > now {
                valid_youtube_cookies += 1;
            }
        }
    }

    if !has_valid_format {
        return Ok(serde_json::json!({
            "valid": false,
            "reason": "invalid_format",
            "message": "File bukan format cookies.txt yang valid (Netscape format)"
        }));
    }

    if youtube_cookies == 0 {
        return Ok(serde_json::json!({
            "valid": false,
            "reason": "no_youtube_cookies",
            "message": "Tidak ditemukan cookies YouTube di dalam file",
            "total_cookies": total_cookies
        }));
    }

    if valid_youtube_cookies == 0 {
        return Ok(serde_json::json!({
            "valid": false,
            "reason": "all_expired",
            "message": "Semua cookies YouTube sudah kedaluwarsa, silakan perbarui file cookies",
            "total_cookies": total_cookies,
            "youtube_cookies": youtube_cookies
        }));
    }

    Ok(serde_json::json!({
        "valid": true,
        "reason": "ok",
        "message": format!("{} cookies YouTube valid", valid_youtube_cookies),
        "total_cookies": total_cookies,
        "youtube_cookies": youtube_cookies,
        "valid_youtube_cookies": valid_youtube_cookies
    }))
}

#[tauri::command]
pub async fn test_youtube_cookies(browser_name: String) -> Result<serde_json::Value, String> {
    let app_dir = crate::paths::app_data_dir();

    #[cfg(target_os = "windows")]
    let ytdlp_bin = app_dir.join("bin").join("yt-dlp.exe");
    #[cfg(not(target_os = "windows"))]
    let ytdlp_bin = app_dir.join("bin").join("yt-dlp");

    if !ytdlp_bin.exists() {
        return Err("Binary yt-dlp tidak ditemukan".into());
    }

    let mut cmd = tokio::process::Command::new(&ytdlp_bin);
    if !browser_name.is_empty() {
        cmd.arg("--cookies-from-browser").arg(&browser_name);
    }
    cmd.arg("--extractor-args")
        .arg("youtube:player-client=android,web,default")
        .arg("--remote-components")
        .arg("ejs:github")
        .arg("--dump-json")
        .arg("--no-warnings")
        .arg("https://www.youtube.com/watch?v=gBSX9DPhRqg"); // a typical small test video

    let output = cmd
        .output()
        .await
        .map_err(|e| format!("Gagal menjalankan yt-dlp: {}", e))?;

    if output.status.success() {
        Ok(serde_json::json!({
            "valid": true,
            "message": "Cookies valid! Berhasil fetching YouTube dengan yt-dlp secara penuh."
        }))
    } else {
        let err_str = String::from_utf8_lossy(&output.stderr);
        log::error!("Cookies test failed: {}", err_str);

        let mut reason = "Tidak dapat fetch video.";
        if err_str.contains("Sign in to confirm you")
            || err_str.contains("Cookies are no longer valid")
            || err_str.contains("Missing required Visitor Data")
        {
            reason = "Cookies kedaluwarsa atau ditolak oleh YouTube. Silakan export cookies baru dari browser.";
        }

        Ok(serde_json::json!({
            "valid": false,
            "message": format!("Test yt-dlp gagal: {}", reason),
            "stderr": err_str.to_string()
        }))
    }
}
