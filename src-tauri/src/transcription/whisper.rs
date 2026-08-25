use crate::error::CliptzyError;
use crate::transcription::models::{TranscriptionSegment, WordTiming};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};
use std::path::Path;
use std::sync::Arc;
use tokio::task;

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
                for j in 0..num_tokens {
                    let token = segment.get_token(j).ok_or_else(|| {
                        CliptzyError::Transcription("Gagal mendapatkan token".into())
                    })?;
                    
                    let token_data = token.token_data();
                    let token_text = token.to_str().unwrap_or("");
                    
                    if token_text.trim().is_empty() {
                        continue;
                    }

                    words.push(WordTiming {
                        word: token_text.to_string(),
                        start: token_data.t0 as f64 / 100.0, // whisper gives ms / 10
                        end: token_data.t1 as f64 / 100.0,
                        probability: token_data.p,
                    });
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
