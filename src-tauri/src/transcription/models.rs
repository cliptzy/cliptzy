use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordTiming {
    pub word: String,
    pub start: f64,
    pub end: f64,
    pub probability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionSegment {
    pub id: usize,
    pub text: String,
    pub start: f64,
    pub end: f64,
    pub words: Vec<WordTiming>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleConfig {
    pub font: String,
    pub font_size: u32,
    pub primary_color: String,   // ASS format e.g. &H00FFFFFF
    pub secondary_color: String, // ASS format (for inactive karaoke text)
    pub outline_color: String,
    pub back_color: String,
    pub outline: u32,
    pub shadow: u32,
    pub margin_v: u32,
    pub alignment: u32, // 2: bottom center, 5: top left, etc.
    pub max_words_per_line: usize,
    pub active_word_color: String, // E.g., &H0000FFFF for yellow
    pub border_style: u32,
    pub animation: String, // "hormozi", "karaoke", "none"
}

impl Default for SubtitleConfig {
    fn default() -> Self {
        Self {
            font: "Arial".to_string(),
            font_size: 24,
            primary_color: "&H00FFFFFF".to_string(),
            secondary_color: "&H00FFFFFF".to_string(),
            outline_color: "&H00000000".to_string(),
            back_color: "&H80000000".to_string(),
            outline: 2,
            shadow: 0,
            margin_v: 20,
            alignment: 2,
            max_words_per_line: 5,
            active_word_color: "&H0000FFFF".to_string(),
            border_style: 1,
            animation: "hormozi".to_string(),
        }
    }
}
