#[tauri::command]
pub async fn analyze_video(
    url: String,
    cookies_path: Option<String>,
) -> Result<serde_json::Value, String> {
    let result = crate::video::youtube::analyze_youtube_video(&url, cookies_path).await?;
    Ok(serde_json::to_value(result).unwrap_or(serde_json::json!({})))
}
