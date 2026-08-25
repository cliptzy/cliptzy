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
        Style: Default,{},{},{},{},{},{},1,0,0,0,100,100,0,0,1,{},{},{},10,10,{},1\n\
        \n\
        [Events]\n\
        Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text\n",
        width, height,
        config.font, config.font_size,
        config.primary_color, config.secondary_color,
        config.outline_color, config.back_color,
        config.outline, config.shadow,
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

        // Generate a line for each word being spoken
        for (i, target_word) in words.iter().enumerate() {
            let line_start = format_timestamp(target_word.start);
            let line_end = if i + 1 < words.len() {
                format_timestamp(words[i + 1].start)
            } else {
                format_timestamp(target_word.end)
            };
            
            let mut text_parts = Vec::new();
            for (j, word) in words.iter().enumerate() {
                if i == j {
                    // Active word
                    text_parts.push(format!("{{\\c{}}}{}{{\\c{}}}", 
                        config.active_word_color, word.word.trim(), config.primary_color));
                } else {
                    // Inactive word
                    text_parts.push(word.word.trim().to_string());
                }
            }

            let dialogue_text = text_parts.join(" ");
            ass_content.push_str(&format!(
                "Dialogue: 0,{},{},Default,,0,0,0,,{}\n",
                line_start, line_end, dialogue_text
            ));
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
