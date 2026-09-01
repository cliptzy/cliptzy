use crate::error::CliptzyError;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn login_with_google(
    client: State<'_, Arc<crate::supabase::SupabaseClient>>,
) -> Result<bool, CliptzyError> {
    client.login_with_google().await
}

#[tauri::command]
pub async fn logout(
    client: State<'_, Arc<crate::supabase::SupabaseClient>>,
) -> Result<(), CliptzyError> {
    client.logout().await
}

#[tauri::command]
pub fn get_user_id(client: State<'_, Arc<crate::supabase::SupabaseClient>>) -> Option<String> {
    client.get_user_id()
}

#[tauri::command]
pub fn get_user_info(
    client: State<'_, Arc<crate::supabase::SupabaseClient>>,
) -> Option<serde_json::Value> {
    log::debug!("[Tauri Command] get_user_info invoked");
    if let Some(id) = client.get_user_id() {
        log::debug!("[Tauri Command] User ID found: {}", id);
        Some(serde_json::json!({
            "id": id,
            "email": client.get_user_email(),
            "display_name": client.get_user_display_name(),
            "avatar_url": client.get_user_avatar_url(),
        }))
    } else {
        log::debug!("[Tauri Command] No active session found");
        None
    }
}

#[tauri::command]
pub fn is_supabase_available(
    client: State<'_, Arc<crate::supabase::SupabaseClient>>,
) -> bool {
    client.is_available()
}
