// commands.rs - Tauri command handlers
use std::sync::Arc;
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
