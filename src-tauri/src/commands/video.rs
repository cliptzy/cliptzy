use crate::config::models::AppConfig;
use crate::deps::AppDependencies;
use crate::error::CliptzyError;
use crate::orchestrator::clip::{ClipPayload, ClipResult, ClipVideoUseCase};
use crate::orchestrator::compilation::{
    EpicMoment, ExecuteCompilationUseCase, PrepareCompilationResult, PrepareCompilationUseCase,
};
use crate::orchestrator::pipeline::PipelineContext;
use crate::orchestrator::segment_audio::{
    AnalyzeSegmentAudioUseCase, SegmentAudioAnalysisResult,
};
use crate::processing::broll_manager::BrollManager;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[tauri::command]
pub async fn analyze_video(
    app: tauri::AppHandle,
    url: String,
    cookies_path: Option<String>,
) -> Result<serde_json::Value, String> {
    let deps = AppDependencies::check()?;
    let result =
        crate::video::youtube::analyze_youtube_video(&url, cookies_path, &deps.ytdlp).await?;

    let _ = upsert_job_history(
        &app,
        &result.video_id,
        Some(&result.title),
        Some(&result.video_url),
        Some(&result.thumbnail),
        Some("clipper"),
        "Draft",
    ).await;

    Ok(serde_json::to_value(result).unwrap_or(serde_json::json!({})))
}

#[tauri::command]
pub async fn clip_video(
    app: tauri::AppHandle,
    state: tauri::State<'_, crate::AppState>,
    payload: ClipPayload,
) -> Result<ClipResult, CliptzyError> {
    let video_id = payload.video_id.clone();
    log::info!(
        "Menerima permintaan clip_video untuk Video ID: {}",
        video_id
    );

    let _ = upsert_job_history(
        &app,
        &video_id,
        None,
        None,
        None,
        None,
        "Processing",
    ).await;

    let cancel_token = tokio_util::sync::CancellationToken::new();
    *state.cancel_token.lock().await = Some(cancel_token.clone());
    let (progress_tx, _) = tokio::sync::broadcast::channel(100);

    let ctx = build_pipeline_context(
        app.clone(),
        cancel_token,
        progress_tx,
        video_id.clone(),
    )
    .await?;

    let mut use_case = ClipVideoUseCase::new(ctx);
    let result = use_case.execute(payload).await;

    let _ = upsert_job_history(
        &app,
        &video_id,
        None,
        None,
        None,
        None,
        if result.is_ok() { "Completed" } else { "Failed" },
    ).await;

    result
}

#[tauri::command]
pub async fn analyze_segment_audio(
    url: String,
    start: f64,
    end: f64,
    _stream_url: Option<String>,
) -> Result<SegmentAudioAnalysisResult, CliptzyError> {
    AnalyzeSegmentAudioUseCase::execute(&url, start, end).await
}

#[tauri::command]
pub async fn list_broll_assets() -> Result<Vec<String>, CliptzyError> {
    let config = AppConfig::load()?;
    let app_dir = crate::paths::app_data_dir();
    let broll_dir = app_dir.join(&config.broll_dir);
    
    let manager = BrollManager::new(&broll_dir);
    let files = manager.list_broll_files()?;
    
    // Return just the filenames for simplicity
    let mut filenames = Vec::new();
    for file in files {
        if let Some(name) = file.file_name() {
            filenames.push(name.to_string_lossy().to_string());
        }
    }
    
    Ok(filenames)
}

#[tauri::command]
pub async fn import_broll_file(source_path: String) -> Result<String, CliptzyError> {
    let config = AppConfig::load()?;
    let app_dir = crate::paths::app_data_dir();
    let broll_dir = app_dir.join(&config.broll_dir);
    
    // Ensure broll directory exists
    fs::create_dir_all(&broll_dir)?;
    
    let source = Path::new(&source_path);
    if !source.exists() {
        return Err(CliptzyError::Config(format!(
            "Source file does not exist: {}",
            source_path
        )));
    }
    
    // Generate destination filename
    let file_name = source.file_name()
        .ok_or_else(|| CliptzyError::Config("Invalid source file path".to_string()))?;
    let dest_path = broll_dir.join(file_name);
    
    // Copy the file
    fs::copy(source, &dest_path)?;
    
    Ok(dest_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn delete_broll_file(filename: String) -> Result<(), CliptzyError> {
    let config = AppConfig::load()?;
    let app_dir = crate::paths::app_data_dir();
    let broll_dir = app_dir.join(&config.broll_dir);
    
    let file_path = broll_dir.join(&filename);
    if !file_path.exists() {
        return Err(CliptzyError::Config(format!(
            "B-roll file does not exist: {}",
            filename
        )));
    }
    
    fs::remove_file(file_path)?;
    
    Ok(())
}

#[tauri::command]
pub async fn prepare_compilation(
    app: tauri::AppHandle,
    state: tauri::State<'_, crate::AppState>,
    video_url: String,
    video_id: String,
    search_keywords: Option<String>,
) -> Result<PrepareCompilationResult, CliptzyError> {
    log::info!(
        "Menerima permintaan prepare_compilation untuk Video ID: {}",
        video_id
    );

    let cancel_token = tokio_util::sync::CancellationToken::new();
    *state.cancel_token.lock().await = Some(cancel_token.clone());
    let (progress_tx, _) = tokio::sync::broadcast::channel(100);

    let ctx = build_pipeline_context(app.clone(), cancel_token, progress_tx, video_id.clone())
        .await?;

    let mut use_case = PrepareCompilationUseCase::new(ctx);
    let result = use_case
        .execute(video_url, search_keywords)
        .await
        .map_err(|e| {
            log::error!(
                "prepare_compilation gagal untuk Video ID {}: {}",
                video_id,
                e
            );
            e
        });
        
    if let Ok(res) = &result {
        let _ = upsert_job_history(
            &app,
            &video_id,
            Some(&res.video_info.title),
            Some(&res.video_info.video_url),
            Some(&res.video_info.thumbnail),
            Some("compilation"),
            "Draft",
        ).await;
    }
    
    result
}

#[tauri::command]
pub async fn execute_compilation(
    app: tauri::AppHandle,
    state: tauri::State<'_, crate::AppState>,
    video_id: String,
    main_audio_path: String,
    restreamer_urls: Vec<String>,
    moments: Vec<EpicMoment>,
    output_filename: String,
) -> Result<String, CliptzyError> {
    log::info!(
        "Menerima permintaan execute_compilation untuk Video ID: {}",
        video_id
    );

    let _ = upsert_job_history(
        &app,
        &video_id,
        None,
        None,
        None,
        None,
        "Processing",
    ).await;

    let cancel_token = tokio_util::sync::CancellationToken::new();
    *state.cancel_token.lock().await = Some(cancel_token.clone());
    let (progress_tx, _) = tokio::sync::broadcast::channel(100);

    let ctx = build_pipeline_context(app.clone(), cancel_token, progress_tx, video_id.clone())
        .await?;

    let mut use_case = ExecuteCompilationUseCase::new(ctx);
    let result = use_case
        .execute(
            main_audio_path,
            restreamer_urls,
            moments,
            output_filename,
        )
        .await
        .map_err(|e| {
            log::error!(
                "execute_compilation gagal untuk Video ID {}: {}",
                video_id,
                e
            );
            e
        });

    let _ = upsert_job_history(
        &app,
        &video_id,
        None,
        None,
        None,
        None,
        if result.is_ok() { "Completed" } else { "Failed" },
    ).await;

    result
}

async fn build_pipeline_context(
    app_handle: tauri::AppHandle,
    cancel_token: tokio_util::sync::CancellationToken,
    progress_tx: crate::orchestrator::pipeline::ProgressTx,
    video_id: String,
) -> Result<PipelineContext, CliptzyError> {
    let config = AppConfig::load().unwrap_or_default();
    let job_dir = crate::paths::app_data_dir().join("jobs").join(&video_id);
    let deps = AppDependencies::check().map_err(CliptzyError::Download)?;

    Ok(PipelineContext {
        job_dir,
        video_id,
        config,
        cancel_token,
        progress_tx,
        app_handle,
        metadata: HashMap::new(),
        deps,
    })
}

pub async fn upsert_job_history(
    app: &tauri::AppHandle, 
    video_id: &str, 
    title: Option<&str>,
    url: Option<&str>,
    thumbnail: Option<&str>,
    mode: Option<&str>, // "clipper" | "compilation"
    status: &str // "Draft" | "Processing" | "Completed" | "Failed"
) -> Result<(), crate::error::CliptzyError> {
    use serde_json::json;
    use tauri_plugin_store::StoreExt;
    use std::time::{SystemTime, UNIX_EPOCH};

    let store = app.store("history.json")
        .map_err(|e| crate::error::CliptzyError::Internal(e.to_string()))?;
    
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let mut job_data = store.get(video_id).unwrap_or(json!({}));

    if let Some(t) = title { job_data["title"] = json!(t); }
    if let Some(u) = url { job_data["url"] = json!(u); }
    if let Some(th) = thumbnail { job_data["thumbnail"] = json!(th); }
    if let Some(m) = mode { job_data["mode"] = json!(m); }
    
    job_data["video_id"] = json!(video_id);
    job_data["status"] = json!(status);
    job_data["updated_at"] = json!(now);

    store.set(video_id, job_data);
    store.save().map_err(|e| crate::error::CliptzyError::Internal(e.to_string()))?;

    Ok(())
}

