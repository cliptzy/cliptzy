use std::sync::Arc;
use tauri::State;

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
