pub mod ai;
pub mod analysis;
pub mod auth;
pub mod channels;
pub mod commands;
pub mod config;
pub mod constants;
pub mod deps;
pub mod error;
pub mod face;
pub mod monitor;
pub mod orchestrator;
pub mod paths;
pub mod processing;
pub mod supabase;
pub mod system;
pub mod transcription;
pub mod tts;
pub mod uploaders;
pub mod utils;
pub mod video;

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

pub struct AppState {
    pub cancel_token: Mutex<Option<CancellationToken>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Set up PATH environment variables for dependencies
    deps::manager::setup_env();

    let app_dir = paths::app_data_dir();
    let logs_dir = app_dir.join("logs");
    std::fs::create_dir_all(&logs_dir).ok();

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Folder {
                        path: logs_dir,
                        file_name: Some("cliptzy.log".to_string()),
                    }),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
                ])
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(Arc::new(
            supabase::SupabaseClient::new().unwrap_or_else(|e| {
                log::warn!(
                    "Supabase tidak tersedia, menjalankan mode offline: {}",
                    e
                );
                supabase::SupabaseClient::offline()
            }),
        ))
        .manage(AppState {
            cancel_token: Mutex::new(None),
        })
        .setup(|_app| {
            log::info!("Aplikasi Cliptzy dimulai. Log diaktifkan.");
            ctrlc::set_handler(move || {
                log::info!("Ctrl+C received, shutting down...");
                std::process::exit(0);
            })
            .ok();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::system::get_system_metrics,
            commands::system::check_system_specs,
            commands::system::exit_app,
            commands::system::get_available_hwaccels,
            commands::system::get_installed_browsers,
            commands::system::get_output_folder_size,
            commands::system::clean_output_folder,
            commands::system::cancel_processing,
            commands::cookies::copy_cookies_file,
            commands::cookies::validate_cookies_file,
            commands::cookies::test_youtube_cookies,
            commands::config::copy_asset_file,
            commands::config::save_config_file,
            commands::config::load_config_file,
            commands::config::read_image_base64,
            commands::video::analyze_video,
            commands::video::clip_video,
            commands::video::prepare_compilation,
            commands::video::execute_compilation,
            commands::video::analyze_segment_audio,
            commands::video::list_broll_assets,
            commands::video::import_broll_file,
            commands::video::delete_broll_file,
            commands::ai::fetch_openai_models,
            commands::auth::login_with_google,
            commands::auth::logout,
            commands::auth::get_user_id,
            commands::auth::is_supabase_available,
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
