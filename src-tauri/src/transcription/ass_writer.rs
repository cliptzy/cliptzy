use crate::error::CliptzyError;
use crate::transcription::models::{SubtitleConfig, TranscriptionSegment};
use std::fs;
use std::path::Path;

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
            let chunk_start_time = chunk.first().unwrap().start;
            let chunk_end_time = chunk.last().unwrap().end;

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
