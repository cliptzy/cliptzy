pub mod monitor;
pub mod paths;
pub mod commands;
pub mod supabase;
pub mod video;
pub mod error;
pub mod config;
pub mod processing;
pub mod transcription;
pub mod face;
pub mod ai;
pub mod analysis;
pub mod tts;
pub mod uploaders;
pub mod orchestrator;
pub mod channels;
pub mod deps;
pub mod constants;

use std::sync::Arc;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Set up PATH environment variables for dependencies
    deps::manager::setup_env();

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
            commands::system::get_system_metrics,
            commands::system::get_available_hwaccels,
            commands::system::get_output_folder_size,
            commands::system::clean_output_folder,
            commands::cookies::copy_cookies_file,
            commands::cookies::validate_cookies_file,
            commands::config::copy_asset_file,
            commands::config::save_config_file,
            commands::video::analyze_video,
            commands::video::clip_video,
            commands::video::analyze_segment_audio,
            commands::auth::login_with_google,
            commands::auth::logout,
            commands::auth::get_user_id,
            commands::sync::sync_config_up,
            commands::sync::sync_config_down,
            commands::sync::upload_file,
            commands::sync::download_file,
            commands::auth::get_user_info,
            orchestrator::scan::scan_video,
            deps::manager::check_dependencies,
            deps::manager::install_dependencies,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
