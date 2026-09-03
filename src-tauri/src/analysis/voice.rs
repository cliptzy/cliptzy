use super::{AnalysisSegment, EmotionAnalyzer, EmotionLabel};
use crate::ai::onnx::OnnxModelManager;
use crate::error::CliptzyError;
use crate::orchestrator::pipeline::ProgressEvent;
use hound::WavReader;
use log::info;
use ndarray::Array2;
use std::path::Path;
use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;

pub struct VoiceEmotionAnalyzer {
    model: OnnxModelManager,
}

impl VoiceEmotionAnalyzer {
    pub fn new() -> Self {
        Self {
            model: OnnxModelManager::new(
                "wav2vec2_superb_er.onnx",
                "https://huggingface.co/onnx-community/wav2vec2-base-superb-er-ONNX/resolve/main/onnx/model.onnx",
            ),
        }
    }

    async fn ensure_model(&self, _progress: &Sender<ProgressEvent>) -> Result<(), CliptzyError> {
        self.model.ensure_loaded().await
    }
}

// Map SUPERB ER classes to EmotionLabel
fn map_superb_to_emotion(idx: usize) -> Option<EmotionLabel> {
    // 0: neu, 1: hap, 2: ang, 3: sad
    match idx {
        0 => Some(EmotionLabel::Neutral),
        1 => Some(EmotionLabel::Happy),
        2 => Some(EmotionLabel::Angry),
        3 => Some(EmotionLabel::Sad),
        _ => None,
    }
}

// Simple Audio Resampling Helper
fn resample(samples: &[f32], from_sr: usize, to_sr: usize) -> Vec<f32> {
    if from_sr == to_sr {
        return samples.to_vec();
    }
    let ratio = from_sr as f32 / to_sr as f32;
    let mut resampled = Vec::with_capacity((samples.len() as f32 / ratio) as usize + 1);
    let mut i = 0.0;
    while (i as usize) < samples.len() {
        resampled.push(samples[i as usize]);
        i += ratio;
    }
    resampled
}

// Voice Activity Detection (Energy-based)
fn run_vad(samples: &[f32], sample_rate: usize, threshold: f32) -> Vec<(usize, usize)> {
    let window_size = sample_rate / 10; // 100ms
    let mut chunks = Vec::new();
    let mut in_speech = false;
    let mut start_idx = 0;

    for (i, window) in samples.chunks(window_size).enumerate() {
        let energy: f32 = window.iter().map(|&x| x.powi(2)).sum::<f32>() / window.len() as f32;
        let is_speech = energy > threshold;

        if is_speech && !in_speech {
            in_speech = true;
            start_idx = i * window_size;
        } else if !is_speech && in_speech {
            in_speech = false;
            let end_idx = i * window_size;
            if end_idx - start_idx > sample_rate / 2 {
                // min 0.5s duration
                chunks.push((start_idx, end_idx));
            }
        }
    }

    if in_speech {
        let end_idx = samples.len();
        if end_idx - start_idx > sample_rate / 2 {
            chunks.push((start_idx, end_idx));
        }
    }

    chunks
}

#[async_trait::async_trait]
impl EmotionAnalyzer for VoiceEmotionAnalyzer {
    fn name(&self) -> &str {
        "Voice Emotion Analyzer (Wav2Vec2 SER ONNX)"
    }

    async fn analyze(
        &self,
        input_path: &Path,
        cancel: &CancellationToken,
        progress: &Sender<ProgressEvent>,
    ) -> Result<Vec<AnalysisSegment>, CliptzyError> {
        self.ensure_model(progress).await?;

        // 1. Read WAV file
        let mut reader = WavReader::open(input_path).map_err(|e| {
            CliptzyError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to read WAV: {}", e),
            ))
        })?;

        let spec = reader.spec();
        let sample_rate = spec.sample_rate as usize;

        let mut samples: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Int => reader
                .samples::<i16>()
                .map(|s| s.unwrap_or(0) as f32 / 32768.0)
                .collect(),
            hound::SampleFormat::Float => {
                reader.samples::<f32>().map(|s| s.unwrap_or(0.0)).collect()
            }
        };

        if spec.channels == 2 {
            samples = samples
                .chunks(2)
                .map(|c| {
                    if c.len() == 2 {
                        (c[0] + c[1]) / 2.0
                    } else {
                        c[0]
                    }
                })
                .collect();
        }

        // 2. Resample to 16kHz
        let target_sr = 16000;
        let resampled = resample(&samples, sample_rate, target_sr);

        let mut segments = Vec::new();
        if resampled.is_empty() {
            return Ok(segments);
        }

        // 3. VAD
        let speech_chunks = run_vad(&resampled, target_sr, 0.0005); // threshold empirically determined

        let mut guard = self.model.get_session()?;
        let session = &mut *guard;

        for (i, &(start_idx, end_idx)) in speech_chunks.iter().enumerate() {
            if cancel.is_cancelled() {
                return Err(CliptzyError::Cancelled);
            }

            if i % 2 == 0 {
                let _ = progress.send(ProgressEvent {
                    stage: "voice_ser".into(),
                    label: format!("Analyzing voice emotion: {}/{}", i + 1, speech_chunks.len()),
                    current: (i as f32 / speech_chunks.len() as f32 * 100.0) as u32,
                    total: 100,
                    detail: None,
                });
            }

            let chunk = &resampled[start_idx..end_idx];
            let max_len = target_sr * 10; // 10 seconds max input to avoid OOM
            let input_len = chunk.len().min(max_len);

            // Input shape: (batch_size, sequence_length)
            let mut input_tensor = Array2::<f32>::zeros((1, input_len));
            for j in 0..input_len {
                input_tensor[[0, j]] = chunk[j];
            }

            let tensor = ort::value::Tensor::from_array(input_tensor)
                .map_err(|e| CliptzyError::Model(format!("Tensor error: {}", e)))?;

            // Input is typically 'input_values' or 'input_features'
            let inputs = ort::inputs!["input_values" => tensor];
            let outputs_res = session.run(inputs);

            if let Ok(outputs) = outputs_res {
                if let Ok((_shape, logits)) = outputs["logits"].try_extract_tensor::<f32>() {
                    // Softmax over 4 classes
                    let max_logit = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
                    let exp_sum: f32 = logits.iter().map(|&x| (x - max_logit).exp()).sum();

                    let mut max_prob = 0.0;
                    let mut best_label = None;

                    for (c, &logit) in logits.iter().enumerate() {
                        let prob = (logit - max_logit).exp() / exp_sum;
                        if prob > max_prob {
                            max_prob = prob;
                            if let Some(emotion) = map_superb_to_emotion(c) {
                                best_label = Some(emotion);
                            }
                        }
                    }

                    if let Some(emotion) = best_label {
                        segments.push(AnalysisSegment {
                            start_time: start_idx as f64 / target_sr as f64,
                            end_time: end_idx as f64 / target_sr as f64,
                            emotion,
                            score: max_prob,
                            bounding_box: None,
                        });
                    }
                }
            }
        }

        info!(
            "Extracted {} voice emotion segments from Wav2Vec2",
            segments.len()
        );
        Ok(segments)
    }
}
