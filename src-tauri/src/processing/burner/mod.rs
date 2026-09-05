pub mod audio;
pub mod builtin;
pub mod subtitle;
pub mod vfx;
pub mod watermark;

use crate::error::CliptzyError;
use crate::processing::ffmpeg::filters::FilterGraph;
use rust_ffmpeg::builder::FFmpegBuilder;
use std::path::Path;

pub struct VideoBurnerConfig {
    pub ass_path: Option<String>,
    pub scheduled_effects: Vec<crate::processing::effects::ScheduledEffect>,
    pub scheduled_builtin_effects: Vec<builtin::ScheduledBuiltinEffect>,
    pub normalize_audio: bool,
    pub config: Option<crate::transcription::models::SubtitleConfig>,
    pub watermark_path: Option<String>,
    pub watermark_position: String,
    pub hw_accel: crate::processing::ffmpeg::hwaccel::HwAccel,
    pub debug_ass_path: Option<String>,
}

pub async fn burn_video_effects(
    input_path: &Path,
    output_path: &Path,
    config: &VideoBurnerConfig,
    progress_info: Option<(&tauri::AppHandle, f64)>,
) -> Result<(), CliptzyError> {
    let mut graph = FilterGraph::new();
    let mut final_v = "0:v".to_string();
    let mut input_idx = 1;

    if let Some(ass_path) = &config.ass_path {
        final_v = subtitle::apply_subtitle(&mut graph, &final_v, ass_path, &config.config);
    }

    // 1. Resolve and validate scheduled effects to ensure assets exist on disk
    let valid_effects: Vec<(&crate::processing::effects::ScheduledEffect, std::path::PathBuf)> =
        config
            .scheduled_effects
            .iter()
            .filter_map(|effect| {
                let vfx_path = effect.effect.resolve_path();
                if vfx_path.exists() {
                    Some((effect, vfx_path))
                } else {
                    log::warn!(
                        "Video effect asset not found at {:?}, skipping effect '{}'",
                        vfx_path,
                        effect.effect.name
                    );
                    None
                }
            })
            .collect();

    let mut delayed_audio_pads = Vec::new();

    for (effect, _) in &valid_effects {
        let (new_v, delayed_a) = vfx::apply_vfx(
            &mut graph,
            &final_v,
            &mut input_idx,
            effect.start_time,
            effect.end_time,
        );
        final_v = new_v;
        delayed_audio_pads.push(delayed_a);
    }

    let mut final_a = if delayed_audio_pads.is_empty() {
        "0:a".to_string()
    } else {
        use crate::processing::ffmpeg::filters::FilterNode;
        let mut amix_inputs = vec!["0:a".to_string()];
        amix_inputs.extend(delayed_audio_pads);
        let amix_input_refs: Vec<&str> = amix_inputs.iter().map(|s| s.as_str()).collect();

        let amix = FilterNode::new("amix")
            .param("inputs", &amix_inputs.len().to_string())
            .param("duration", "first")
            .param("dropout_transition", "0")
            .param("normalize", "0")
            .inputs(&amix_input_refs)
            .outputs(&["a_vfx_mixed"]);

        graph.add_node(amix);
        "a_vfx_mixed".to_string()
    };

    // 2. Resolve and validate watermark path
    let resolved_watermark: Option<std::path::PathBuf> = config
        .watermark_path
        .as_ref()
        .and_then(|wm_path| {
            let path_str = wm_path.trim();
            if path_str.is_empty() {
                None
            } else {
                let actual_path = if path_str.starts_with("assets/") || path_str.starts_with("assets\\")
                {
                    crate::paths::app_data_dir().join(path_str)
                } else {
                    std::path::PathBuf::from(path_str)
                };
                if actual_path.exists() {
                    Some(actual_path)
                } else {
                    log::warn!(
                        "Watermark asset not found at {:?}, skipping watermark overlay",
                        actual_path
                    );
                    None
                }
            }
        });

    // Built-in visual effects (camera shake, white flash, B&W vignette, deep-fried, punch zoom)
    final_v = builtin::apply_builtin_effects(
        &mut graph,
        &final_v,
        &config.scheduled_builtin_effects,
    );

    if resolved_watermark.is_some() {
        final_v = watermark::apply_watermark(
            &mut graph,
            &final_v,
            &mut input_idx,
            &config.watermark_position,
        );
    }

    // MSI Afterburner OSD HUD overlay (rendered on top of all visual elements)
    if let Some(debug_ass) = &config.debug_ass_path {
        final_v = subtitle::apply_subtitle(&mut graph, &final_v, debug_ass, &None);
    }

    if config.normalize_audio {
        final_a = audio::apply_normalization(&mut graph, &final_a);
    }

    let hw_accel = &config.hw_accel;

    let mut builder = FFmpegBuilder::new().map_err(|e| CliptzyError::FFmpeg {
        code: -1,
        message: format!("FFmpeg builder error: {}", e),
    })?;

    builder = builder.input_path(input_path.to_path_buf());

    for (_, vfx_path) in &valid_effects {
        builder = builder.input_path(vfx_path.clone());
    }

    if let Some(wm_path) = resolved_watermark {
        builder = builder.input_path(wm_path);
    }

    builder = builder
        .filter_complex(graph.to_string())
        .raw_args(vec![
            "-map".to_string(),
            format!("[{}]", final_v),
            "-map".to_string(),
            format!("[{}]", final_a),
        ])
        .raw_args(hw_accel.encode_args())
        .raw_args(vec![
            "-pix_fmt".to_string(),
            "yuv420p".to_string(),
            "-movflags".to_string(),
            "+faststart".to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            "-b:a".to_string(),
            "192k".to_string(),
        ])
        .overwrite()
        .output_path(output_path.to_path_buf());

    log::info!("FFmpeg Burn Command: {:?}", builder);

    if let Some((app_handle, total_duration)) = progress_info {
        let handle_clone = app_handle.clone();
        builder = builder.on_progress(move |prog| {
            if let Some(time) = prog.time {
                let current_sec = time.as_secs_f64();
                if total_duration > 0.0 {
                    let mut pct = (current_sec / total_duration) * 100.0;
                    if pct > 99.9 {
                        pct = 99.9;
                    }
                    crate::orchestrator::pipeline::emit_progress(
                        &handle_clone,
                        &crate::orchestrator::pipeline::ProgressEvent {
                            stage: "subtitle".into(),
                            label: format!(
                                "Menambahkan efek visual/teks ke video... ({:.1}%)",
                                pct
                            ),
                            current: pct as u32,
                            total: 100,
                            detail: None,
                        },
                    );
                }
            }
        });
    }

    let process = builder.spawn().await.map_err(|e| CliptzyError::FFmpeg {
        code: -1,
        message: format!("Spawn failed: {}", e),
    })?;

    process.wait().await.map_err(|e| CliptzyError::FFmpeg {
        code: -1,
        message: format!("Process failed: {}", e),
    })?;

    Ok(())
}
