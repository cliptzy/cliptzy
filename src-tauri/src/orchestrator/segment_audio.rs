use crate::config::models::AppConfig;
use crate::deps::AppDependencies;
use crate::error::CliptzyError;
use crate::transcription::models::TranscriptionSegment;
use serde::Serialize;

#[derive(Serialize)]
pub struct SegmentAudioAnalysisResult {
    pub transcript: Vec<TranscriptionSegment>,
    pub ai_effects: Vec<serde_json::Value>,
}

pub struct AnalyzeSegmentAudioUseCase;

impl AnalyzeSegmentAudioUseCase {
    pub async fn execute(
        url: &str,
        start: f64,
        end: f64,
    ) -> Result<SegmentAudioAnalysisResult, CliptzyError> {
        let app_dir = crate::paths::app_data_dir();
        let temp_dir = app_dir.join("temp");
        std::fs::create_dir_all(&temp_dir).ok();

        let file_name = format!(
            "seg_{}_{}_{}.wav",
            uuid::Uuid::new_v4()
                .to_string()
                .chars()
                .take(8)
                .collect::<String>(),
            start,
            end
        );
        let audio_wav_path = temp_dir.join(file_name);

        log::info!(
            "Analisis segmen audio dimulai | Start: {}s, End: {}s",
            start,
            end
        );

        let config = AppConfig::load().unwrap_or_default();
        let cookies_path = config.browser.as_deref().filter(|s| !s.is_empty());
        let deps = AppDependencies::check().map_err(CliptzyError::Download)?;

        log::info!("Tahap 1: Ekstraksi WAV melalui yt-dlp/FFmpeg...");
        crate::transcription::audio::extract_audio_segment(
            url,
            start,
            end,
            &audio_wav_path,
            cookies_path,
            &deps.ytdlp,
        )
        .await?;

        let whisper_model = if config.subtitle.whisper_model.is_empty() {
            "tiny".to_string()
        } else {
            config.subtitle.whisper_model.clone()
        };

        log::info!(
            "Tahap 2: Memeriksa dan memuat model Whisper ({})...",
            whisper_model
        );
        let model_path = crate::transcription::whisper::ensure_model_exists(&whisper_model).await?;

        log::info!("Tahap 3: Menjalankan transkripsi Whisper (local)...");
        let transcriber = crate::transcription::whisper::WhisperTranscriber::new(&model_path)?;
        let transcript = transcriber.transcribe(&audio_wav_path).await?;

        log::info!("Tahap 4: Membersihkan file audio sementara...");
        let _ = std::fs::remove_file(&audio_wav_path);

        log::info!(
            "Analisis segmen audio selesai. Ditemukan {} blok teks.",
            transcript.len()
        );

        Ok(SegmentAudioAnalysisResult {
            transcript,
            ai_effects: vec![],
        })
    }
}
