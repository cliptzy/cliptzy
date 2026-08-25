use super::models::*;

impl Default for SubtitleConfig {
    fn default() -> Self {
        Self {
            whisper_model: "small".to_string(),
            font: "Bangers".to_string(),
            fonts_dir: "assets/fonts".to_string(),
            location: "bottom".to_string(),
            delay: 0.0,
            font_size: 24,
            color: "&H00FFFFFF".to_string(),
            bg_color: "&H80000000".to_string(),
            border_style: 1,
            animation: "hormozi".to_string(),
            style: "full_color".to_string(),
            max_words: 5,
        }
    }
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            provider: "gemini".to_string(),
            ollama_host: "http://localhost:11434".to_string(),
            ollama_model: "llama3".to_string(),
            gemini_key: "".to_string(),
            gemini_model: "gemini-1.5-flash".to_string(),
            openai_key: "".to_string(),
            openai_model: "gpt-4o-mini".to_string(),
            openai_base_url: "https://api.openai.com/v1".to_string(),
            use_highlight: false,
            use_generate_intro: false,
        }
    }
}

impl Default for PlatformConfig {
    fn default() -> Self {
        Self {
            session: "".to_string(),
            auto_upload: false,
            visibility: "private".to_string(),
        }
    }
}

impl Default for CompilationConfig {
    fn default() -> Self {
        Self {
            ordering: "countdown".to_string(),
            numbering_duration: 3.0,
            use_tts: false,
            tts_template: "Klip nomor {}".to_string(),
            use_subtitle: true,
            crop_mode: "default".to_string(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            subtitle: SubtitleConfig::default(),
            ai: AIConfig::default(),
            youtube: PlatformConfig::default(),
            tiktok: PlatformConfig::default(),
            instagram: PlatformConfig::default(),
            compilation: CompilationConfig::default(),
        }
    }
}
