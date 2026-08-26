use crate::error::CliptzyError;
use crate::transcription::models::{TranscriptionSegment, WordTiming};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::task;

pub async fn ensure_model_exists(model_name: &str) -> Result<PathBuf, CliptzyError> {
    let app_dir = crate::paths::app_data_dir();
    let models_dir = app_dir.join("models");
    std::fs::create_dir_all(&models_dir).ok();
    
    let model_filename = format!("ggml-{}.bin", model_name);
    let model_path = models_dir.join(&model_filename);
    
    if !model_path.exists() {
        let url = format!("https://huggingface.co/ggerganov/whisper.cpp/resolve/main/{}", model_filename);
        let response = reqwest::get(&url).await.map_err(|e| CliptzyError::Config(format!("Download failed: {}", e)))?;
        let bytes = response.bytes().await.map_err(|e| CliptzyError::Config(format!("Download failed: {}", e)))?;
        std::fs::write(&model_path, bytes).map_err(|e| CliptzyError::Io(e))?;
    }
    
    Ok(model_path)
}

pub struct WhisperTranscriber {
    context: Arc<WhisperContext>,
}

impl WhisperTranscriber {
    pub fn new(model_path: &Path) -> Result<Self, CliptzyError> {
        let params = WhisperContextParameters::default();
        let ctx = WhisperContext::new_with_params(&*model_path.to_string_lossy(), params)
            .map_err(|e| CliptzyError::Transcription(format!("Gagal load model: {}", e)))?;
        Ok(Self {
            context: Arc::new(ctx),
        })
    }

    pub async fn transcribe(&self, audio_wav_path: &Path) -> Result<Vec<TranscriptionSegment>, CliptzyError> {
        let path_buf = audio_wav_path.to_path_buf();
        let ctx_clone = self.context.clone();

        let segments = task::spawn_blocking(move || {
            let mut reader = hound::WavReader::open(path_buf)
                .map_err(|e| CliptzyError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
            let spec = reader.spec();
            
            if spec.sample_rate != 16000 || spec.channels != 1 {
                return Err(CliptzyError::Transcription("Format audio harus 16kHz mono".into()));
            }

            let audio_data: Vec<f32> = match spec.sample_format {
                hound::SampleFormat::Int => {
                    reader
                        .samples::<i16>()
                        .map(|s| s.unwrap_or(0) as f32 / 32768.0)
                        .collect()
                }
                hound::SampleFormat::Float => {
                    reader
                        .samples::<f32>()
                        .map(|s| s.unwrap_or(0.0))
                        .collect()
                }
            };

            let mut state = ctx_clone
                .create_state()
                .map_err(|e| CliptzyError::Transcription(e.to_string()))?;

            let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
            params.set_print_progress(false);
            params.set_print_special(false);
            params.set_print_realtime(false);
            params.set_print_timestamps(false);
            params.set_token_timestamps(true); // Word-level timestamps
            params.set_language(Some("auto")); // Auto-detect language instead of default English

            state
                .full(params, &audio_data[..])
                .map_err(|e| CliptzyError::Transcription(e.to_string()))?;

            let num_segments = state.full_n_segments();

            let mut segments = Vec::new();

            for i in 0..num_segments {
                let segment = state.get_segment(i).ok_or_else(|| {
                    CliptzyError::Transcription("Gagal mendapatkan segmen".into())
                })?;
                
                let segment_text = segment.to_str().unwrap_or("").to_string();
                let start_ts = segment.start_timestamp();
                let end_ts = segment.end_timestamp();
                let num_tokens = segment.n_tokens();

                let mut words = Vec::new();
                let mut current_word: Option<WordTiming> = None;

                for j in 0..num_tokens {
                    let token = segment.get_token(j).ok_or_else(|| {
                        CliptzyError::Transcription("Gagal mendapatkan token".into())
                    })?;
                    
                    let token_data = token.token_data();
                    let token_text = token.to_str().unwrap_or("").to_string();
                    
                    // Skip whisper special tokens (usually wrapped in [_*_] or [*])
                    if (token_text.starts_with("[_") && token_text.ends_with("_]")) || 
                       (token_text.starts_with('[') && token_text.ends_with(']')) {
                        continue;
                    }
                    
                    if token_text.trim().is_empty() && !token_text.contains(' ') {
                        continue;
                    }

                    let start_time = token_data.t0 as f64 / 100.0;
                    let end_time = token_data.t1 as f64 / 100.0;
                    let starts_with_space = token_text.starts_with(' ');

                    if starts_with_space || current_word.is_none() {
                        // Push previous word if it exists
                        if let Some(cw) = current_word.take() {
                            if !cw.word.trim().is_empty() {
                                words.push(cw);
                            }
                        }
                        
                        current_word = Some(WordTiming {
                            word: token_text,
                            start: start_time,
                            end: end_time,
                            probability: token_data.p,
                        });
                    } else {
                        // Append to the current word
                        if let Some(mut cw) = current_word.take() {
                            cw.word.push_str(&token_text);
                            cw.end = end_time;
                            cw.probability = cw.probability.min(token_data.p); 
                            current_word = Some(cw);
                        }
                    }
                }
                
                // Push the very last word
                if let Some(cw) = current_word.take() {
                    if !cw.word.trim().is_empty() {
                        words.push(cw);
                    }
                }

                segments.push(TranscriptionSegment {
                    id: i as usize,
                    text: segment_text,
                    start: start_ts as f64 / 100.0,
                    end: end_ts as f64 / 100.0,
                    words,
                });
            }

            Ok(segments)
        })
        .await
        .map_err(|e| CliptzyError::Transcription(format!("Task panic: {}", e)))??;

        Ok(segments)
    }
}
