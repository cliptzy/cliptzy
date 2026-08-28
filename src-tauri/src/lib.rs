pub mod ai;
pub mod analysis;
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

    // Inisialisasi tracing untuk logging
    let app_dir = paths::app_data_dir();
    let logs_dir = app_dir.join("logs");
    std::fs::create_dir_all(&logs_dir).ok();

    let file_appender = tracing_appender::rolling::daily(logs_dir, "cliptzy.log");
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);
    Box::leak(Box::new(guard));

    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    let stdout_log = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_ansi(true);
        
    let file_log = tracing_subscriber::fmt::layer()
        .with_writer(file_writer)
        .with_ansi(false);

    tracing_subscriber::registry()
        .with(stdout_log)
        .with(file_log)
        .with(tracing_subscriber::EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    tracing::info!("Aplikasi Cliptzy dimulai. Log diaktifkan.");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(Arc::new(
            supabase::SupabaseClient::new().expect("Failed to initialize Supabase"),
        ))
        .manage(AppState {
            cancel_token: Mutex::new(None),
        })
        .setup(|_app| {
            ctrlc::set_handler(move || {
                tracing::info!("Ctrl+C received, shutting down...");
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
