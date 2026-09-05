use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct SubtitleConfig {
    pub enabled: bool,
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
    pub use_emotion_detection: bool,
    pub use_voice_analysis: bool,
    pub use_audio_analysis: bool,
    pub use_text_analysis: bool,
    pub use_add_meme: bool,
    pub use_builtin_fx: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct YoutubeConfig {
    pub upload: bool,
    pub session: Option<String>,
    pub client_id: String,
    pub client_secret: String,
    pub visibility: String,
    pub auto_upload: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct TikTokConfig {
    pub upload: bool,
    pub session: Option<String>,
    pub privacy: String,
    pub auto_upload: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct InstagramConfig {
    pub upload: bool,
    pub business_id: String,
    pub access_token: String,
    pub session: Option<String>,
    pub auto_upload: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct CompilationConfig {
    /// `meme_shorts` (vertikal) atau `reaction` (horizontal tanpa crop)
    pub compilation_type: String,
    pub ordering: String,
    pub numbering_duration: f64,
    pub use_tts: bool,
    pub tts_template: String,
    pub use_subtitle: bool,
    /// `none` = tanpa crop, pertahankan resolusi asli (mode reaksi)
    pub crop_mode: String,
    /// Batas durasi segmen dalam detik. `0` = tidak terbatas (mode reaksi).
    pub max_segment_duration: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct AppConfig {
    pub output_dir: String,
    pub min_duration: u32,
    pub min_score: f64,
    pub max_clips: u32,
    pub padding: i32,
    pub top_height: u32,
    pub bottom_height: u32,

    pub intro_video: Option<String>,
    pub outro_video: Option<String>,
    pub watermark_image: Option<String>,
    pub video_frame: Option<String>,
    pub watermark_position: String,
    pub burn_watermark: bool,
    pub burn_subtitle: bool,

    pub output_ratio: String,
    pub out_width: Option<u32>,
    pub out_height: Option<u32>,

    pub job_dir: String,
    pub crop_mode: String,
    pub face_tracking_mode: String,
    pub merge_clips: bool,
    pub ui_locked: bool,
    pub broll_dir: String,

    pub emotion: EmotionConfig,
    pub upload_interval: f64,
    pub hw_accel: String,
    pub debug_mode: bool,
    pub max_workers: u32,

    pub tts_language: String,
    pub tts_voice: String,
    pub default_hashtags: String,
    pub browser: Option<String>,

    pub subtitle: SubtitleConfig,
    pub ai: AIConfig,
    pub youtube: YoutubeConfig,
    pub tiktok: TikTokConfig,
    pub instagram: InstagramConfig,
    pub compilation: CompilationConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, crate::error::CliptzyError> {
        let app_dir = crate::paths::app_data_dir();
        let config_path = app_dir.join("config.json");

        if !config_path.exists() {
            log::info!(
                "config.json tidak ditemukan di {:?}, menggunakan nilai default.",
                config_path
            );
            return Ok(Self::default());
        }

        log::info!("Membaca konfigurasi dari {:?}", config_path);
        let content = std::fs::read_to_string(&config_path)?;
        match serde_json::from_str(&content) {
            Ok(config) => {
                log::info!("Berhasil memuat konfigurasi.");
                Ok(config)
            }
            Err(e) => {
                log::warn!(
                    "Gagal parsing config.json: {}. Menggunakan fallback default.",
                    e
                );
                // Return default on parse failure to prevent UI breaking completely
                Ok(Self::default())
            }
        }
    }

    pub fn save(&self) -> Result<(), crate::error::CliptzyError> {
        let app_dir = crate::paths::app_data_dir();
        if let Err(e) = std::fs::create_dir_all(&app_dir) {
            log::error!("Gagal membuat direktori data aplikasi {:?}: {}", app_dir, e);
        }

        let config_path = app_dir.join("config.json");
        log::info!("Menyimpan konfigurasi ke {:?}", config_path);

        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        log::info!("Konfigurasi berhasil disimpan.");
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct EmotionConfig {
    pub enabled: bool,
    pub enable_visual: bool,
    pub enable_audio: bool,
    pub enable_voice: bool,
    pub enable_text: bool,
}

impl Default for EmotionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            enable_visual: true,
            enable_audio: true,
            enable_voice: true,
            enable_text: true,
        }
    }
}
