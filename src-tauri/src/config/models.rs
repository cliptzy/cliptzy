use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubtitleConfig {
    pub whisper_model: String,
    pub font: String,
    pub fonts_dir: String,
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
pub struct PlatformConfig {
    pub session: String,
    pub auto_upload: bool,
    pub visibility: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompilationConfig {
    pub ordering: String,
    pub numbering_duration: f64,
    pub use_tts: bool,
    pub tts_template: String,
    pub use_subtitle: bool,
    pub crop_mode: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub subtitle: SubtitleConfig,
    pub ai: AIConfig,
    pub youtube: PlatformConfig,
    pub tiktok: PlatformConfig,
    pub instagram: PlatformConfig,
    pub compilation: CompilationConfig,
}
