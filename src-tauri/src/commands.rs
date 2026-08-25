// commands.rs - Tauri command handlers
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::State;

#[tauri::command]
pub fn get_system_metrics() -> crate::monitor::ProcessMetrics {
    crate::monitor::get_system_metrics()
}

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
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Gagal membaca file cookies: {}", e))?;

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
pub async fn copy_asset_file(source_path: String, filename: String) -> Result<String, String> {
    let app_dir = crate::paths::app_data_dir();
    let assets_dir = app_dir.join("assets");
    std::fs::create_dir_all(&assets_dir).map_err(|e| e.to_string())?;

    let dest_path = assets_dir.join(filename);
    std::fs::copy(&source_path, &dest_path).map_err(|e| e.to_string())?;

    // Return relative path for config, e.g. "assets/filename.png"
    Ok(format!(
        "assets/{}",
        dest_path.file_name().unwrap().to_string_lossy()
    ))
}

#[tauri::command]
pub async fn save_config_file(config_json: String) -> Result<(), String> {
    let app_dir = crate::paths::app_data_dir();
    let config_path = app_dir.join("config.json");
    std::fs::write(&config_path, config_json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn analyze_video(
    url: String,
    cookies_path: Option<String>,
) -> Result<serde_json::Value, String> {
    let result = crate::video::youtube::analyze_youtube_video(&url, cookies_path).await?;
    Ok(serde_json::to_value(result).unwrap_or(serde_json::json!({})))
}

#[tauri::command]
pub async fn login_with_google(
    client: State<'_, Arc<crate::supabase::SupabaseClient>>,
) -> Result<bool, String> {
    client.login_with_google().await
}

#[tauri::command]
pub async fn logout(client: State<'_, Arc<crate::supabase::SupabaseClient>>) -> Result<(), String> {
    client.logout().await
}

#[tauri::command]
pub fn get_user_id(client: State<'_, Arc<crate::supabase::SupabaseClient>>) -> Option<String> {
    client.get_user_id()
}

#[tauri::command]
pub async fn sync_config_up(
    client: State<'_, Arc<crate::supabase::SupabaseClient>>,
    config_dict: serde_json::Value,
) -> Result<bool, String> {
    client.sync_config_up(config_dict).await
}

#[tauri::command]
pub async fn sync_config_down(
    client: State<'_, Arc<crate::supabase::SupabaseClient>>,
) -> Result<Option<serde_json::Value>, String> {
    client.sync_config_down().await
}

#[tauri::command]
pub async fn upload_file(
    client: State<'_, Arc<crate::supabase::SupabaseClient>>,
    local_path: String,
    remote_filename: String,
) -> Result<bool, String> {
    client
        .upload_file(&std::path::PathBuf::from(local_path), &remote_filename)
        .await
}

#[tauri::command]
pub async fn download_file(
    client: State<'_, Arc<crate::supabase::SupabaseClient>>,
    remote_filename: String,
    local_path: String,
) -> Result<bool, String> {
    client
        .download_file(&remote_filename, &std::path::PathBuf::from(local_path))
        .await
}

#[tauri::command]
pub fn get_user_info(
    client: State<'_, Arc<crate::supabase::SupabaseClient>>,
) -> Option<serde_json::Value> {
    if let Some(id) = client.get_user_id() {
        Some(serde_json::json!({
            "id": id,
            "email": client.get_user_email(),
            "display_name": client.get_user_display_name(),
            "avatar_url": client.get_user_avatar_url(),
        }))
    } else {
        None
    }
}
