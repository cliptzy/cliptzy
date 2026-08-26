use super::models::*;

impl Default for SubtitleConfig {
    fn default() -> Self {
        Self {
            whisper_model: "small".to_string(),
            font: "Bangers".to_string(),
            fonts_dir: Some("assets/fonts".to_string()),
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
            session: Some("".to_string()),
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
            output_dir: "clips".to_string(),
            min_duration: 60,
            min_score: 0.40,
            max_clips: 10,
            padding: 10,
            top_height: 960,
            bottom_height: 320,

            intro_video: None,
            outro_video: None,
            watermark_image: None,
            video_frame: None,
            watermark_position: "center".to_string(),

            output_ratio: "9:16".to_string(),
            out_width: Some(720),
            out_height: Some(1280),

            job_dir: "".to_string(),
            crop_mode: "default".to_string(),
            merge_clips: false,
            ui_locked: false,

            upload_interval: 0.0,
            hw_accel: "cpu".to_string(),
            debug_mode: false,
            max_workers: 2,

            tts_language: "default".to_string(),
            tts_voice: "female".to_string(),
            default_hashtags: "#Shorts #Viral #Cliptzy #fyp".to_string(),

            subtitle: SubtitleConfig::default(),
            ai: AIConfig::default(),
            youtube: PlatformConfig::default(),
            tiktok: PlatformConfig::default(),
            instagram: PlatformConfig::default(),
            compilation: CompilationConfig::default(),
        }
    }
}
