use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct SubtitleConfig {
    pub whisper_model: String,
    pub font: String,
    pub fonts_dir: Option<String>,
    pub location: String,
    pub delay: f64,
    pub font_size: u32,
    pub color: String,
    pub bg_color: String,
    pub border_style: u32,
    pub animation: String,
    pub style: String,
    pub max_words: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct AIConfig {
    pub provider: String,
    pub ollama_host: String,
    pub ollama_model: String,
    pub gemini_key: String,
    pub gemini_model: String,
    pub openai_key: String,
    pub openai_model: String,
    pub openai_base_url: String,
    pub use_highlight: bool,
    pub use_generate_intro: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct PlatformConfig {
    pub session: Option<String>,
    pub auto_upload: bool,
    pub visibility: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct CompilationConfig {
    pub ordering: String,
    pub numbering_duration: f64,
    pub use_tts: bool,
    pub tts_template: String,
    pub use_subtitle: bool,
    pub crop_mode: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct AppConfig {
    pub output_dir: String,
    pub min_duration: u32,
    pub min_score: f64,
    pub max_clips: u32,
    pub padding: u32,
    pub top_height: u32,
    pub bottom_height: u32,

    pub intro_video: Option<String>,
    pub outro_video: Option<String>,
    pub watermark_image: Option<String>,
    pub video_frame: Option<String>,
    pub watermark_position: String,

    pub output_ratio: String,
    pub out_width: Option<u32>,
    pub out_height: Option<u32>,

    pub job_dir: String,
    pub crop_mode: String,
    pub merge_clips: bool,
    pub ui_locked: bool,

    pub upload_interval: f64,
    pub hw_accel: String,
    pub debug_mode: bool,
    pub max_workers: u32,

    pub tts_language: String,
    pub tts_voice: String,
    pub default_hashtags: String,

    pub subtitle: SubtitleConfig,
    pub ai: AIConfig,
    pub youtube: PlatformConfig,
    pub tiktok: PlatformConfig,
    pub instagram: PlatformConfig,
    pub compilation: CompilationConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, crate::error::CliptzyError> {
        let app_dir = crate::paths::app_data_dir();
        let config_path = app_dir.join("config.json");

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(&config_path)?;
        let config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), crate::error::CliptzyError> {
        let app_dir = crate::paths::app_data_dir();
        let config_path = app_dir.join("config.json");

        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }
}
