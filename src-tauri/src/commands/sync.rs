use std::sync::Arc;
use tauri::State;

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
