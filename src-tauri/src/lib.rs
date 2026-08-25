pub mod monitor;
pub mod paths;
pub mod commands;
pub mod supabase;
pub mod video;

use std::sync::Arc;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Inisialisasi tracing untuk logging
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(Arc::new(supabase::SupabaseClient::new().expect("Failed to initialize Supabase")))
        .setup(|_app| {
            ctrlc::set_handler(move || {
                tracing::info!("Ctrl+C received, shutting down...");
                std::process::exit(0);
            }).ok();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_system_metrics,
            commands::copy_cookies_file,
            commands::copy_asset_file,
            commands::save_config_file,
            commands::analyze_video,
            commands::login_with_google,
            commands::logout,
            commands::get_user_id,
            commands::sync_config_up,
            commands::sync_config_down,
            commands::upload_file,
            commands::download_file,
            commands::get_user_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
