use super::{AnalysisSegment, EmotionAnalyzer, EmotionLabel};
use crate::ai::onnx::OnnxModelManager;
use crate::error::CliptzyError;
use crate::orchestrator::pipeline::ProgressEvent;
use hound::WavReader;
use log::info;
use ndarray::Array3;
use rustfft::{num_complex::Complex, FftPlanner};
use std::collections::HashMap;
use std::path::Path;
use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;

pub struct AudioEventAnalyzer {
    model: OnnxModelManager,
}

impl AudioEventAnalyzer {
    pub fn new() -> Self {
        let model_info = crate::ai::onnx::find_model("audio");
        let file = model_info.map(|m| m.file).unwrap_or("ast_audioset.onnx");
        let url = model_info.map(|m| m.url).unwrap_or("");
        Self {
            model: OnnxModelManager::new(file, url),
        }
    }

    async fn ensure_model(&self, _progress: &Sender<ProgressEvent>) -> Result<(), CliptzyError> {
        self.model.ensure_loaded().await
    }
}

// Map AudioSet label index to EmotionLabel based on Google AudioSet ontology
fn map_audioset_to_emotion(idx: usize) -> Option<EmotionLabel> {
    match idx {
        // Shouting, yelling, belligerent vocalizations -> Angry
        8 | 9 | 11 | 12 | 13 => Some(EmotionLabel::Angry), // Shout, Bellow, Yell, Battle cry, Children shouting
        39 => Some(EmotionLabel::Angry),                    // Grunt
        396 => Some(EmotionLabel::Angry),                   // Slap, smack

        // Screaming -> Fear
        14 => Some(EmotionLabel::Fear), // Screaming

        // Laughter, joyful vocalizations -> Happy
        10 => Some(EmotionLabel::Happy), // Whoop
        16 | 17 | 18 | 19 | 20 | 21 => Some(EmotionLabel::Happy), // Laughter, Baby laughter, Giggle, Snicker, Belly laugh, Chuckle
        49 | 50 => Some(EmotionLabel::Happy),                     // Cheering, Applause

        // Crying, groaning, sadness -> Sad
        22 | 23 | 24 | 25 | 26 => Some(EmotionLabel::Sad), // Crying/sobbing, Baby cry, Whimper, Wail/moan, Sigh
        38 => Some(EmotionLabel::Sad),                     // Groan

        // Explosive events, gunshots -> Shock
        418 | 419 | 420 | 421 => Some(EmotionLabel::Shock), // Gunshot, Machine gun, Fusillade, Artillery fire
        426 | 427 => Some(EmotionLabel::Shock),             // Explosion, Boom

        _ => None,
    }
}

/// Helper to build a standard triangular Mel filterbank matrix (num_mel_bins, num_fft_bins).
fn build_mel_filterbank(
    num_mel_bins: usize,
    n_fft: usize,
    sample_rate: usize,
    f_min: f32,
    f_max: f32,
) -> Vec<Vec<f32>> {
    let num_fft_bins = n_fft / 2 + 1;
    let hz_to_mel = |hz: f32| 1127.0 * (1.0 + hz / 700.0).ln();
    let mel_to_hz = |mel: f32| 700.0 * ((mel / 1127.0).exp() - 1.0);

    let mel_min = hz_to_mel(f_min);
    let mel_max = hz_to_mel(f_max);

    let mut hz_points = Vec::with_capacity(num_mel_bins + 2);
    for i in 0..=(num_mel_bins + 1) {
        let frac = i as f32 / (num_mel_bins + 1) as f32;
        let mel = mel_min + frac * (mel_max - mel_min);
        hz_points.push(mel_to_hz(mel));
    }

    let mut filterbank = vec![vec![0.0f32; num_fft_bins]; num_mel_bins];
    for m in 0..num_mel_bins {
        let f_left = hz_points[m];
        let f_center = hz_points[m + 1];
        let f_right = hz_points[m + 2];

        for k in 0..num_fft_bins {
            let f_k = k as f32 * sample_rate as f32 / n_fft as f32;
            if f_k >= f_left && f_k <= f_center {
                if f_center > f_left {
                    filterbank[m][k] = (f_k - f_left) / (f_center - f_left);
                }
            } else if f_k > f_center && f_k <= f_right {
                if f_right > f_center {
                    filterbank[m][k] = (f_right - f_k) / (f_right - f_center);
                }
            }
        }

        // If filter is narrower than one FFT bin, assign 1.0 to nearest bin
        let sum: f32 = filterbank[m].iter().sum();
        if sum == 0.0 {
            let nearest_bin = ((f_center * n_fft as f32 / sample_rate as f32).round() as usize)
                .min(num_fft_bins - 1);
            filterbank[m][nearest_bin] = 1.0;
        }
    }

    filterbank
}

#[async_trait::async_trait]
impl EmotionAnalyzer for AudioEventAnalyzer {
    fn name(&self) -> &str {
        "Audio Event Analyzer (AST ONNX)"
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
                format!("Failed to read WAV at {:?}: {}", input_path, e),
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

        // Convert stereo to mono if needed
        if spec.channels == 2 {
            let mono: Vec<f32> = samples
                .chunks(2)
                .map(|c| {
                    if c.len() == 2 {
                        (c[0] + c[1]) / 2.0
                    } else {
                        c[0]
                    }
                })
                .collect();
            samples = mono;
        }

        // 2. Resample to 16000 Hz if needed (AST requirement)
        let target_sr = 16000;
        let mut resampled = Vec::new();
        if sample_rate != target_sr {
            let ratio = sample_rate as f32 / target_sr as f32;
            let mut i = 0.0;
            while (i as usize) < samples.len() {
                resampled.push(samples[i as usize]);
                i += ratio;
            }
        } else {
            resampled = samples;
        }

        let mut segments = Vec::new();
        if resampled.is_empty() {
            return Ok(segments);
        }

        // 3. Audio Spectrogram extraction using RustFFT + Mel Filterbank
        let mut guard = self.model.get_session()?;
        let session = &mut *guard;

        let window_size = 400; // 25ms at 16kHz
        let hop_length = 160; // 10ms at 16kHz
        let mel_bins = 128;

        let chunk_frames = 1024; // ~10.24 seconds per inference chunk
        let chunk_samples = (chunk_frames - 1) * hop_length + window_size;

        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(window_size);
        let hann_window: Vec<f32> = (0..window_size)
            .map(|i| {
                0.5 * (1.0
                    - (2.0 * std::f32::consts::PI * i as f32 / (window_size as f32 - 1.0)).cos())
            })
            .collect();

        let filterbank = build_mel_filterbank(mel_bins, window_size, target_sr, 20.0, 8000.0);
        let total_chunks = (resampled.len() / chunk_samples).max(1);

        for (c_idx, chunk) in resampled.chunks(chunk_samples).enumerate() {
            if cancel.is_cancelled() {
                return Err(CliptzyError::Cancelled);
            }

            if c_idx % 2 == 0 {
                let _ = progress.send(ProgressEvent {
                    stage: "audio_ast".into(),
                    label: format!("Analyzing audio events: {}/{}", c_idx + 1, total_chunks),
                    current: (c_idx as f32 / total_chunks as f32 * 100.0) as u32,
                    total: 100,
                    detail: None,
                });
            }

            // Create input tensor: shape (1, 1024, 128)
            let mut input_tensor = Array3::<f32>::zeros((1, chunk_frames, mel_bins));

            for f_idx in 0..chunk_frames {
                let start = f_idx * hop_length;
                if start + window_size > chunk.len() {
                    break;
                }

                let mut buffer = vec![Complex { re: 0.0, im: 0.0 }; window_size];
                for i in 0..window_size {
                    buffer[i].re = chunk[start + i] * hann_window[i];
                }

                fft.process(&mut buffer);

                // Power spectrum
                let mut power = vec![0.0f32; window_size / 2 + 1];
                for i in 0..power.len() {
                    power[i] = buffer[i].re.powi(2) + buffer[i].im.powi(2);
                }

                // Mel filterbank projection
                for m in 0..mel_bins {
                    let mut mel_energy = 0.0f32;
                    for (k, &w) in filterbank[m].iter().enumerate() {
                        if w > 0.0 {
                            mel_energy += power[k] * w;
                        }
                    }
                    input_tensor[[0, f_idx, m]] = (mel_energy.max(1e-10)).ln();
                }
            }

            // Mean/Std Normalization for AST (mean=-4.2677393, std=4.5689974)
            let mean = -4.2677393;
            let std = 4.5689974;
            for val in input_tensor.iter_mut() {
                *val = (*val - mean) / (std * 2.0);
            }

            let tensor = ort::value::Tensor::from_array(input_tensor)
                .map_err(|e| CliptzyError::Model(format!("Tensor error: {}", e)))?;

            let inputs = ort::inputs!["input_values" => tensor];
            let outputs_res = session.run(inputs);

            if let Ok(outputs) = outputs_res {
                if let Ok((_shape, logits)) = outputs["logits"].try_extract_tensor::<f32>() {
                    // Apply sigmoid (multi-label)
                    let probs: Vec<f32> =
                        logits.iter().map(|&x| 1.0 / (1.0 + (-x).exp())).collect();

                    let mut chunk_emotions: HashMap<EmotionLabel, f32> = HashMap::new();

                    for (i, &p) in probs.iter().enumerate() {
                        if p > 0.15 {
                            if let Some(emotion) = map_audioset_to_emotion(i) {
                                let entry = chunk_emotions.entry(emotion).or_insert(0.0);
                                if p > *entry {
                                    *entry = p;
                                }
                            }
                        }
                    }

                    let start_time = c_idx as f64 * (chunk_samples as f64 / target_sr as f64);
                    let end_time = (start_time + (chunk.len() as f64 / target_sr as f64))
                        .min(resampled.len() as f64 / target_sr as f64);

                    for (emotion, score) in chunk_emotions {
                        segments.push(AnalysisSegment {
                            start_time,
                            end_time,
                            emotion,
                            score,
                            bounding_box: None,
                        });
                    }
                }
            }
        }

        info!("Extracted {} audio events from AST", segments.len());
        Ok(segments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_audioset_to_emotion() {
        assert_eq!(map_audioset_to_emotion(8), Some(EmotionLabel::Angry)); // Shout
        assert_eq!(map_audioset_to_emotion(11), Some(EmotionLabel::Angry)); // Yell
        assert_eq!(map_audioset_to_emotion(14), Some(EmotionLabel::Fear)); // Screaming
        assert_eq!(map_audioset_to_emotion(16), Some(EmotionLabel::Happy)); // Laughter
        assert_eq!(map_audioset_to_emotion(22), Some(EmotionLabel::Sad)); // Crying
        assert_eq!(map_audioset_to_emotion(426), Some(EmotionLabel::Shock)); // Explosion
        assert_eq!(map_audioset_to_emotion(999), None);
    }

    #[test]
    fn test_build_mel_filterbank() {
        let fb = build_mel_filterbank(128, 400, 16000, 20.0, 8000.0);
        assert_eq!(fb.len(), 128);
        assert_eq!(fb[0].len(), 201);
        // Each filter should have at least one non-zero coefficient
        for (i, row) in fb.iter().enumerate() {
            let sum: f32 = row.iter().sum();
            assert!(sum > 0.0, "Filter {} has zero energy", i);
        }
    }
}
