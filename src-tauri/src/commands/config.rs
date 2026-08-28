use crate::config::models::AppConfig;
use crate::error::CliptzyError;

#[tauri::command]
pub async fn copy_asset_file(
    source_path: String,
    filename: String,
) -> Result<String, CliptzyError> {
    let app_dir = crate::paths::app_data_dir();
    let assets_dir = app_dir.join("assets");
    std::fs::create_dir_all(&assets_dir)?;

    let dest_path = assets_dir.join(filename);
    std::fs::copy(&source_path, &dest_path)?;

    Ok(format!(
        "assets/{}",
        dest_path.file_name().unwrap().to_string_lossy()
    ))
}

#[tauri::command]
pub async fn save_config_file(config_json: String) -> Result<(), CliptzyError> {
    tracing::debug!("Menerima permintaan save_config_file dari Frontend");
    let config: AppConfig = serde_json::from_str(&config_json).map_err(|e| {
        tracing::error!("Payload config dari UI tidak valid: {}", e);
        CliptzyError::Config(e.to_string())
    })?;
    config.save()?;
    Ok(())
}

#[tauri::command]
pub async fn load_config_file() -> Result<String, CliptzyError> {
    tracing::debug!("Menerima permintaan load_config_file dari Frontend");
    let config = AppConfig::load()?;
    let json = serde_json::to_string(&config)?;
    Ok(json)
}

#[tauri::command]
pub async fn read_image_base64(path: String) -> Result<String, CliptzyError> {
    use std::path::PathBuf;
    let actual_path = if path.starts_with("assets/") || path.starts_with("assets\\") {
        crate::paths::app_data_dir().join(&path)
    } else {
        PathBuf::from(&path)
    };

    let bytes =
        std::fs::read(&actual_path).map_err(|e| CliptzyError::FileNotFound(e.to_string()))?;

    use base64::{engine::general_purpose, Engine as _};
    let b64 = general_purpose::STANDARD.encode(&bytes);

    let ext = actual_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png");
    let mime = match ext.to_lowercase().as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        _ => "image/png",
    };

    Ok(format!("data:{};base64,{}", mime, b64))
}
