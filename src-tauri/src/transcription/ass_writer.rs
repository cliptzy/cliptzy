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

    let probe = crate::video::local::probe_local_video(source_video)
        .await
        .ok()?;
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

/// Generates an MSI Afterburner / RTSS styled on-screen display (OSD) ASS overlay
/// showing real-time multi-modal AI metrics (Fusion, Vision, Voice, Audio, Whisper STT, VFX meme, Engine).
pub async fn generate_msi_afterburner_osd(
    video_path: &Path,
    emotion_cache_path: &Path,
    scheduled_effects: &[crate::processing::effects::ScheduledEffect],
    config: &crate::config::models::AppConfig,
    _hw_accel: &crate::processing::ffmpeg::hwaccel::HwAccel,
    transcript: Option<&[crate::transcription::models::TranscriptionSegment]>,
    total_duration: f64,
    output_ass_path: &Path,
) -> Option<PathBuf> {
    use std::io::Write;

    let probe = crate::video::local::probe_local_video(video_path)
        .await
        .ok();
    let mut v_w = 1080u32;
    let mut v_h = 1920u32;
    if let Some(ref p) = probe {
        for stream in &p.streams {
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
    }

    let cache_entry: Option<crate::orchestrator::clip::EmotionCacheEntry> =
        if emotion_cache_path.exists() {
            std::fs::read_to_string(emotion_cache_path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
        } else {
            None
        };

    let font_size = if v_h >= 1920 {
        24
    } else if v_h >= 1080 {
        20
    } else {
        16
    };

    let mut file = std::fs::File::create(output_ass_path).ok()?;

    writeln!(file, "[Script Info]").ok()?;
    writeln!(file, "ScriptType: v4.00+").ok()?;
    writeln!(file, "PlayResX: {}", v_w).ok()?;
    writeln!(file, "PlayResY: {}", v_h).ok()?;
    writeln!(file, "WrapStyle: 0").ok()?;
    writeln!(file, "").ok()?;
    writeln!(file, "[V4+ Styles]").ok()?;
    writeln!(
        file,
        "Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding"
    ).ok()?;
    writeln!(
        file,
        "Style: MSI_OSD,Consolas,{},&H00FFFFFF,&H00000000,&H00000000,&HA0101010,1,0,0,0,100,100,0,0,3,6,0,7,25,25,25,1",
        font_size
    ).ok()?;
    writeln!(file, "").ok()?;
    writeln!(file, "[Events]").ok()?;
    writeln!(
        file,
        "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text"
    ).ok()?;

    let duration_secs = if total_duration > 0.0 {
        total_duration
    } else {
        60.0
    };
    let num_slices = duration_secs.ceil() as usize;

    let hw_label = match config.hw_accel.to_lowercase().as_str() {
        "nvidia" => "NVENC (GPU)",
        "amd" => "AMF (GPU)",
        "mac" => "VIDEOTOOLBOX (Apple)",
        _ => "CPU (libx264)",
    };

    for b in 0..num_slices {
        let start_t = b as f64;
        let end_t = ((b + 1) as f64).min(duration_secs);
        let start_ts = format_timestamp(start_t);
        let end_ts = format_timestamp(end_t);

        // 1. FUSION
        let fusion_line = if let Some(ref entry) = cache_entry {
            if let Some(seg) = entry.segments.iter().find(|s| s.start_time <= end_t && s.end_time >= start_t) {
                let emo_str = format!("{:?}", seg.emotion).to_uppercase();
                let score_pct = (seg.score * 100.0).clamp(0.0, 100.0);
                format!("{{\\c&H002080FF&}}FUSION   {{\\c&H00A0A0A0&}}: {{\\c&H00FFFF00&}}{} {{\\c&H0000FF00&}}[{:.1}%]", emo_str, score_pct)
            } else {
                "{\\c&H002080FF&}FUSION   {\\c&H00A0A0A0&}: {\\c&H00FFFF00&}NEUTRAL {\\c&H0000FF00&}[0.0%]".to_string()
            }
        } else {
            "{\\c&H002080FF&}FUSION   {\\c&H00A0A0A0&}: {\\c&H00A0A0A0&}NO DATA".to_string()
        };

        // 2. VISION (Face / Bbox / ViT)
        let vision_line = if let Some(ref entry) = cache_entry {
            let vis_seg = entry.visual.iter().find(|s| s.start_time <= end_t && s.end_time >= start_t)
                .or_else(|| entry.segments.iter().find(|s| s.start_time <= end_t && s.end_time >= start_t && s.bounding_box.is_some()));

            if let Some(seg) = vis_seg {
                let emo_str = format!("{:?}", seg.emotion).to_uppercase();
                let score_pct = (seg.score * 100.0).clamp(0.0, 100.0);
                if let Some(ref bbox) = seg.bounding_box {
                    let xp = (bbox.x * 100.0) as i32;
                    let yp = (bbox.y * 100.0) as i32;
                    let wp = (bbox.w * 100.0) as i32;
                    let hp = (bbox.h * 100.0) as i32;
                    format!("{{\\c&H002080FF&}}VISION   {{\\c&H00A0A0A0&}}: {{\\c&H00FFFF00&}}FACE [{}%X {}%Y {}x{}%] {{\\c&H0000FF00&}}[{} {:.0}%]", xp, yp, wp, hp, emo_str, score_pct)
                } else {
                    format!("{{\\c&H002080FF&}}VISION   {{\\c&H00A0A0A0&}}: {{\\c&H00FFFF00&}}FACE DETECTED {{\\c&H0000FF00&}}[{} {:.0}%]", emo_str, score_pct)
                }
            } else {
                "{\\c&H002080FF&}VISION   {\\c&H00A0A0A0&}: {\\c&H00A0A0A0&}NO FACE / AMBIENT".to_string()
            }
        } else {
            "{\\c&H002080FF&}VISION   {\\c&H00A0A0A0&}: {\\c&H00A0A0A0&}NO DATA".to_string()
        };

        // 3. VOICE (Tone / Pitch)
        let voice_line = if let Some(ref entry) = cache_entry {
            if let Some(seg) = entry.voice.iter().find(|s| s.start_time <= end_t && s.end_time >= start_t) {
                let emo_str = format!("{:?}", seg.emotion).to_uppercase();
                let score_pct = (seg.score * 100.0).clamp(0.0, 100.0);
                format!("{{\\c&H002080FF&}}VOICE    {{\\c&H00A0A0A0&}}: {{\\c&H00FFFF00&}}{} {{\\c&H0000FF00&}}[{:.1}%] {{\\c&H00A0A0A0&}}| PITCH: ACTIVE", emo_str, score_pct)
            } else {
                "{\\c&H002080FF&}VOICE    {\\c&H00A0A0A0&}: {\\c&H00A0A0A0&}CALM / NORMAL".to_string()
            }
        } else {
            "{\\c&H002080FF&}VOICE    {\\c&H00A0A0A0&}: {\\c&H00A0A0A0&}NO DATA".to_string()
        };

        // 4. AUDIO (Energy / Sound Events)
        let audio_line = if let Some(ref entry) = cache_entry {
            if let Some(seg) = entry.audio.iter().find(|s| s.start_time <= end_t && s.end_time >= start_t) {
                let emo_str = format!("{:?}", seg.emotion).to_uppercase();
                let score_pct = (seg.score * 100.0).clamp(0.0, 100.0);
                format!("{{\\c&H002080FF&}}AUDIO    {{\\c&H00A0A0A0&}}: {{\\c&H00FFFF00&}}{} {{\\c&H0000FF00&}}[{:.1}%] {{\\c&H00A0A0A0&}}| ENERGY: {:.2}", emo_str, score_pct, seg.score)
            } else {
                "{\\c&H002080FF&}AUDIO    {\\c&H00A0A0A0&}: {\\c&H00A0A0A0&}NORMAL / AMBIENT".to_string()
            }
        } else {
            "{\\c&H002080FF&}AUDIO    {\\c&H00A0A0A0&}: {\\c&H00A0A0A0&}NO DATA".to_string()
        };

        // 5. TEXT / WHISPER
        let whisper_line = if let Some(trans) = transcript {
            let matching_words: Vec<&str> = trans.iter()
                .flat_map(|seg| &seg.words)
                .filter(|w| w.start <= end_t && w.end >= start_t)
                .map(|w| w.word.trim())
                .filter(|s| !s.is_empty())
                .collect();

            if !matching_words.is_empty() {
                let words_preview = matching_words.iter().take(4).cloned().collect::<Vec<_>>().join(" ");
                format!("{{\\c&H002080FF&}}WHISPER  {{\\c&H00A0A0A0&}}: {{\\c&H00FFFFFF&}}\"{}\"", words_preview)
            } else {
                "{\\c&H002080FF&}WHISPER  {\\c&H00A0A0A0&}: {\\c&H00A0A0A0&}[NO SPEECH]".to_string()
            }
        } else {
            "{\\c&H002080FF&}WHISPER  {\\c&H00A0A0A0&}: {\\c&H00A0A0A0&}[INACTIVE]".to_string()
        };

        // 6. VFX MEME & CAMERA FX
        let active_builtin = if let Some(ref entry) = cache_entry {
            entry
                .scheduled_builtin_effects
                .iter()
                .find(|e| e.start_time <= end_t && e.end_time >= start_t)
        } else {
            None
        };
        let active_meme = scheduled_effects
            .iter()
            .find(|e| e.start_time <= end_t && e.start_time + 4.0 >= start_t);

        let vfx_line = match (active_meme, active_builtin) {
            (Some(m), Some(b)) => format!(
                "{{\\c&H002080FF&}}VFX      {{\\c&H00A0A0A0&}}: {{\\c&H0000FF00&}}{} {{\\c&H00FFFF00&}}+ {:?} [ACTIVE]",
                m.effect.name, b.effect_type
            ),
            (Some(m), None) => format!(
                "{{\\c&H002080FF&}}VFX      {{\\c&H00A0A0A0&}}: {{\\c&H0000FF00&}}{} [ACTIVE]",
                m.effect.name
            ),
            (None, Some(b)) => format!(
                "{{\\c&H002080FF&}}VFX      {{\\c&H00A0A0A0&}}: {{\\c&H00FFFF00&}}{:?} [CAMERA FX]",
                b.effect_type
            ),
            (None, None) => "{\\c&H002080FF&}VFX      {\\c&H00A0A0A0&}: {\\c&H00A0A0A0&}IDLE".to_string(),
        };

        // 7. ENGINE STATS
        let engine_line = format!("{{\\c&H00A0A0A0&}}ENGINE   : {} | {}x{} | THR: {}", hw_label, v_w, v_h, config.max_workers);

        let full_text = format!(
            "{{\\an7\\pos(30,40)}}{{\\c&H002080FF&}}CLIPTZY AI ENGINE OSD\\N{}\\N{}\\N{}\\N{}\\N{}\\N{}\\N{}",
            fusion_line,
            vision_line,
            voice_line,
            audio_line,
            whisper_line,
            vfx_line,
            engine_line
        );

        writeln!(
            file,
            "Dialogue: 0,{},{},MSI_OSD,,0,0,0,,{}",
            start_ts, end_ts, full_text
        ).ok()?;
    }

    log::info!("MSI Afterburner Debug OSD ASS generated at {:?}", output_ass_path);
    Some(output_ass_path.to_path_buf())
}

fn get_emotion_color(
    time: f64,
    timeline: Option<&crate::analysis::fusion::EmotionTimeline>,
    default_color: &str,
) -> String {
    if let Some(tl) = timeline {
        for seg in &tl.segments {
            if time >= seg.start_time && time <= seg.end_time {
                return match seg.emotion {
                    crate::analysis::EmotionLabel::Happy => "&H0000FFFF&".to_string(),
                    crate::analysis::EmotionLabel::Angry => "&H000000FF&".to_string(),
                    crate::analysis::EmotionLabel::Sad => "&H00FF0000&".to_string(),
                    _ => default_color.to_string(),
                };
            }
        }
    }
    default_color.to_string()
}

pub fn generate_ass_file(
    segments: &[TranscriptionSegment],
    output_path: &Path,
    config: &SubtitleConfig,
    resolution: (u32, u32),
    emotion_timeline: Option<&crate::analysis::fusion::EmotionTimeline>,
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
                            let dynamic_color = get_emotion_color(
                                word.start,
                                emotion_timeline,
                                &config.active_word_color,
                            );
                            if config.animation == "hormozi" {
                                text_parts.push(format!(
                                    "{{\\c{}}}{}{{\\c{}}}",
                                    dynamic_color, w_text, config.primary_color
                                ));
                            } else {
                                // karaoke
                                text_parts.push(format!(
                                    "{{\\c{}}}{{\\k10}}{}{{\\c{}}}",
                                    dynamic_color, w_text, config.primary_color
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
    writeln!(
        file,
        "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text"
    )?;

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

            writeln!(
                file,
                "Dialogue: 0,{},{},DebugBox,,0,0,0,,{}",
                start, end, draw_cmd
            )?;

            let text = format!("{:?} ({:.1}%)", seg.emotion, seg.score * 100.0);
            writeln!(
                file,
                "Dialogue: 0,{},{},DebugText,,0,0,0,,{{\\pos({},{})}}{}",
                start,
                end,
                x1,
                y1 - 40,
                text
            )?;
        }
    }

    Ok(())
}
