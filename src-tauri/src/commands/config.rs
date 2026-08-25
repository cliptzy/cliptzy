use crate::error::CliptzyError;
use crate::config::models::AppConfig;

#[tauri::command]
pub async fn copy_asset_file(source_path: String, filename: String) -> Result<String, CliptzyError> {
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
    let config: AppConfig = serde_json::from_str(&config_json)?;
    config.save()?;
    Ok(())
}

#[tauri::command]
pub async fn load_config_file() -> Result<String, CliptzyError> {
    let config = AppConfig::load()?;
    let json = serde_json::to_string(&config)?;
    Ok(json)
}
