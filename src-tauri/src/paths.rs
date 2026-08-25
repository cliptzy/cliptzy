// paths.rs - Centralized Path Resolution
use std::path::PathBuf;

/// Mengembalikan path ke direktori data aplikasi (root).
/// - DEV: `<project_root>/`
/// - PROD: `<AppData>/com.dickymuliafiqri.cliptzy/`
pub fn app_data_dir() -> PathBuf {
    if cfg!(debug_assertions) {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .unwrap_or_else(|_| ".".to_string());
        // CARGO_MANIFEST_DIR is src-tauri, so we go up one level to project root
        PathBuf::from(manifest_dir).parent().unwrap_or(std::path::Path::new(".")).to_path_buf()
    } else {
        let app_data = dirs::data_local_dir()
            .expect("Cannot determine AppData directory");
        app_data.join("com.dickymuliafiqri.cliptzy")
    }
}
