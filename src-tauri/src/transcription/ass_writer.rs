use crate::error::CliptzyError;
use crate::transcription::models::{SubtitleConfig, TranscriptionSegment};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

/// Builds a render-ready subtitle config from app settings and video dimensions.
pub fn build_render_config(
    app: &crate::config::models::SubtitleConfig,
    video_height: u32,
) -> SubtitleConfig {
    let mut sub_config = SubtitleConfig::default();
    if !app.font.is_empty() {
        sub_config.font = app.font.clone();
    }
    if app.font_size > 0 {
        sub_config.font_size = app.font_size;
    }
    if !app.color.is_empty() {
        sub_config.primary_color = app.color.clone();
    }
    if !app.bg_color.is_empty() {
        sub_config.back_color = app.bg_color.clone();
    }
    if app.border_style > 0 {
        sub_config.border_style = app.border_style;
    }
    if !app.animation.is_empty() {
        sub_config.animation = app.animation.clone();
    }
    if app.max_words > 0 {
        sub_config.max_words_per_line = app.max_words as usize;
    }
    sub_config.alignment = match app.location.as_str() {
        "top" => 8,
        "center" => 5,
        "bottom" => 2,
        _ => 2,
    };
    sub_config.margin_v = (video_height as f32 * 0.12) as u32;
    apply_brutalist_box_style(&mut sub_config);
    sub_config
}

/// Applies the "Brutalist Box" preset when border_style == 3.
pub fn apply_brutalist_box_style(config: &mut SubtitleConfig) {
    if config.border_style == 3 {
        config.font = "Courier New".to_string();
        config.primary_color = "&H00FFFFFF".to_string();
        config.outline_color = "&H002626DC".to_string();
        config.back_color = "&H00000000".to_string();
        config.outline = 4;
        config.shadow = 4;
    }
}

#[derive(Deserialize)]
struct EmotionDebugCache {
    segments: Vec<crate::analysis::AnalysisSegment>,
}

/// Generates a debug ASS overlay from cached emotion analysis, if available.
pub async fn try_generate_emotion_debug_ass(
    source_video: &Path,
    emotion_cache_path: &Path,
    output_ass_path: &Path,
) -> Option<PathBuf> {
    let json_str = std::fs::read_to_string(emotion_cache_path).ok()?;
    let cached: EmotionDebugCache = serde_json::from_str(&json_str).ok()?;

    let probe = crate::video::local::probe_local_video(source_video).await.ok()?;
    let mut v_w = 1920u32;
    let mut v_h = 1080u32;
    for stream in probe.streams {
        if stream.codec_type == Some("video".to_string()) {
            if let Some(w) = stream.width {
                v_w = w as u32;
            }
            if let Some(h) = stream.height {
                v_h = h as u32;
            }
            break;
        }
    }

    generate_debug_ass(&cached.segments, output_ass_path, v_w, v_h)
        .map_err(|e| log::warn!("Gagal generate debug ASS: {}", e))
        .ok()?;

    log::info!("Debug ASS generated at {:?}", output_ass_path);
    Some(output_ass_path.to_path_buf())
}


pub fn generate_ass_file(
    segments: &[TranscriptionSegment],
    output_path: &Path,
    config: &SubtitleConfig,
    resolution: (u32, u32),
) -> Result<(), CliptzyError> {
    let (width, height) = resolution;

    let mut ass_content = format!(
        "[Script Info]\n\
        ScriptType: v4.00+\n\
        PlayResX: {}\n\
        PlayResY: {}\n\
        WrapStyle: 1\n\
        \n\
        [V4+ Styles]\n\
        Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding\n\
        Style: Default,{},{},{},{},{},{},1,0,0,0,100,100,0,0,{},{},{},{},10,10,{},1\n\
        \n\
        [Events]\n\
        Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text\n",
        width, height,
        config.font, config.font_size,
        config.primary_color, config.secondary_color,
        config.outline_color, config.back_color,
        config.border_style, config.outline, config.shadow,
        config.alignment, config.margin_v
    );

    for segment in segments {
        // Basic Hormozi style: highlight active word
        // ASS format for active word: {\c&H0000FFFF&}word{\c&H00FFFFFF&}
        // In a real Karaoke implementation we'd output multiple event lines for the same segment,
        // each highlighting a different word.

        let words = &segment.words;
        if words.is_empty() {
            continue;
        }

        let max_words = if config.max_words_per_line > 0 {
            config.max_words_per_line
        } else {
            5
        };
        let word_chunks: Vec<&[crate::transcription::models::WordTiming]> =
            words.chunks(max_words).collect();

        for chunk in word_chunks {
            let chunk_start_time = chunk.first().map(|w| w.start).unwrap_or(0.0);
            let chunk_end_time = chunk.last().map(|w| w.end).unwrap_or(0.0);

            if config.animation == "hormozi" || config.animation == "karaoke" {
                // For Karaoke or Hormozi, we generate a dialogue line per word
                for (i, target_word) in chunk.iter().enumerate() {
                    let line_start = format_timestamp(target_word.start);
                    let line_end = if i + 1 < chunk.len() {
                        format_timestamp(chunk[i + 1].start)
                    } else {
                        format_timestamp(target_word.end)
                    };

                    let mut text_parts = Vec::new();
                    let is_upper = config.animation == "hormozi" || config.border_style == 3;
                    for (j, word) in chunk.iter().enumerate() {
                        let mut w_text = word.word.trim().to_string();
                        if is_upper {
                            w_text = w_text.to_uppercase();
                        }
                        if i == j {
                            if config.animation == "hormozi" {
                                text_parts.push(format!(
                                    "{{\\c{}}}{}{{\\c{}}}",
                                    config.active_word_color, w_text, config.primary_color
                                ));
                            } else {
                                // karaoke
                                text_parts.push(format!(
                                    "{{\\c{}}}{{\\k10}}{}{{\\c{}}}",
                                    config.active_word_color, w_text, config.primary_color
                                ));
                            }
                        } else {
                            text_parts.push(w_text);
                        }
                    }

                    let dialogue_text = text_parts.join(" ");
                    ass_content.push_str(&format!(
                        "Dialogue: 0,{},{},Default,,0,0,0,,{}\n",
                        line_start, line_end, dialogue_text
                    ));
                }
            } else {
                // "none" animation - just display the whole chunk at once
                let line_start = format_timestamp(chunk_start_time);
                let line_end = format_timestamp(chunk_end_time);

                let is_upper = config.border_style == 3;
                let text_parts: Vec<String> = chunk
                    .iter()
                    .map(|w| {
                        let mut w_text = w.word.trim().to_string();
                        if is_upper {
                            w_text = w_text.to_uppercase();
                        }
                        w_text
                    })
                    .collect();
                let dialogue_text = text_parts.join(" ");

                ass_content.push_str(&format!(
                    "Dialogue: 0,{},{},Default,,0,0,0,,{}\n",
                    line_start, line_end, dialogue_text
                ));
            }
        }
    }

    fs::write(output_path, ass_content).map_err(CliptzyError::Io)?;

    Ok(())
}

fn format_timestamp(seconds: f64) -> String {
    let hours = (seconds / 3600.0).floor() as u32;
    let minutes = ((seconds % 3600.0) / 60.0).floor() as u32;
    let secs = (seconds % 60.0).floor() as u32;
    let centisecs = ((seconds % 1.0) * 100.0).round() as u32;

    format!("{:01}:{:02}:{:02}.{:02}", hours, minutes, secs, centisecs)
}

pub fn generate_debug_ass(
    segments: &[crate::analysis::AnalysisSegment],
    output_path: &std::path::Path,
    video_width: u32,
    video_height: u32,
) -> Result<(), CliptzyError> {
    use std::io::Write;

    let mut file = std::fs::File::create(output_path).map_err(|e| CliptzyError::Io(e))?;

    // Header ASS
    writeln!(file, "[Script Info]")?;
    writeln!(file, "ScriptType: v4.00+")?;
    writeln!(file, "PlayResX: {}", video_width)?;
    writeln!(file, "PlayResY: {}", video_height)?;
    writeln!(file, "WrapStyle: 0")?;
    writeln!(file, "")?;
    writeln!(file, "[V4+ Styles]")?;
    writeln!(file, "Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding")?;
    writeln!(file, "Style: DebugBox,Arial,48,&HFF0000FF,&H000000FF,&H000000FF,&H00000000,0,0,0,0,100,100,0,0,1,3,0,7,0,0,0,1")?;
    writeln!(file, "Style: DebugText,Arial,36,&H00FFFFFF,&H000000FF,&H00000000,&H80000000,1,0,0,0,100,100,0,0,1,2,2,7,0,0,0,1")?;
    writeln!(file, "")?;
    writeln!(file, "[Events]")?;
    writeln!(file, "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text")?;

    for seg in segments {
        if let Some(bbox) = &seg.bounding_box {
            let start = format_timestamp(seg.start_time);
            let end = format_timestamp(seg.end_time);

            // Konversi dari normalisasi (0.0 - 1.0) ke resolusi video sumber
            let x1 = (bbox.x * video_width as f32) as i32;
            let y1 = (bbox.y * video_height as f32) as i32;
            let w = (bbox.w * video_width as f32) as i32;
            let h = (bbox.h * video_height as f32) as i32;

            let x2 = x1 + w;
            let y2 = y1 + h;

            // Draw vector box: m x1 y1 l x2 y1 l x2 y2 l x1 y2
            let draw_cmd = format!(
                "{{\\p1\\pos(0,0)}}m {} {} l {} {} l {} {} l {} {}",
                x1, y1, x2, y1, x2, y2, x1, y2
            );

            writeln!(file, "Dialogue: 0,{},{},DebugBox,,0,0,0,,{}", start, end, draw_cmd)?;

            let text = format!("{:?} ({:.1}%)", seg.emotion, seg.score * 100.0);
            writeln!(file, "Dialogue: 0,{},{},DebugText,,0,0,0,,{{\\pos({},{})}}{}", start, end, x1, y1 - 40, text)?;
        }
    }

    Ok(())
}
