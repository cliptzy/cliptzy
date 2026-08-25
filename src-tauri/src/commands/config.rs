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
