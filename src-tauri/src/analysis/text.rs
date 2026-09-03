use super::{AnalysisSegment, EmotionAnalyzer, EmotionLabel};
use crate::ai::onnx::{ensure_model_downloaded, OnnxModelManager};
use crate::error::CliptzyError;
use crate::orchestrator::pipeline::ProgressEvent;
use log::info;
use ndarray::Array2;
use std::path::Path;
use tokenizers::Tokenizer;
use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;

pub struct TextSentimentAnalyzer {
    model: OnnxModelManager,
}

impl TextSentimentAnalyzer {
    pub fn new() -> Self {
        Self {
            model: OnnxModelManager::new(
                "twitter_roberta_emotion.onnx",
                "https://huggingface.co/onnx-community/twitter-roberta-base-emotion-ONNX/resolve/main/onnx/model.onnx",
            ),
        }
    }

    async fn ensure_assets(
        &self,
        _progress: &Sender<ProgressEvent>,
    ) -> Result<std::path::PathBuf, CliptzyError> {
        self.model.ensure_loaded().await?;

        // Download tokenizer.json
        let tokenizer_path = ensure_model_downloaded(
            "twitter_roberta_tokenizer.onnx",
            "https://huggingface.co/onnx-community/twitter-roberta-base-emotion-ONNX/resolve/main/onnx/model.onnx"
        ).await.map_err(|e| CliptzyError::Model(e))?;

        Ok(tokenizer_path)
    }
}

// Map Twitter Roberta classes to EmotionLabel
fn map_roberta_to_emotion(idx: usize) -> Option<EmotionLabel> {
    // 0: anger, 1: joy, 2: optimism, 3: sadness
    match idx {
        0 => Some(EmotionLabel::Angry),
        1 => Some(EmotionLabel::Happy),
        2 => Some(EmotionLabel::Happy), // Map optimism to happy
        3 => Some(EmotionLabel::Sad),
        _ => None,
    }
}

#[derive(serde::Deserialize)]
struct TranscriptSegment {
    start: f64,
    end: f64,
    text: String,
}

#[async_trait::async_trait]
impl EmotionAnalyzer for TextSentimentAnalyzer {
    fn name(&self) -> &str {
        "Text Sentiment Analyzer (RoBERTa ONNX)"
    }

    async fn analyze(
        &self,
        input_path: &Path,
        cancel: &CancellationToken,
        progress: &Sender<ProgressEvent>,
    ) -> Result<Vec<AnalysisSegment>, CliptzyError> {
        let tokenizer_path = self.ensure_assets(progress).await?;

        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| CliptzyError::Model(format!("Failed to load tokenizer: {}", e)))?;

        // Parse transcript file (assuming JSON array of TranscriptSegment)
        let file_content = std::fs::read_to_string(input_path).map_err(|e| {
            CliptzyError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to read transcript: {}", e),
            ))
        })?;

        let transcript: Vec<TranscriptSegment> = serde_json::from_str(&file_content)?;

        let mut guard = self.model.get_session()?;
        let session = &mut *guard;

        let mut segments = Vec::new();

        for (i, t_seg) in transcript.iter().enumerate() {
            if cancel.is_cancelled() {
                return Err(CliptzyError::Cancelled);
            }

            if i % 10 == 0 {
                let _ = progress.send(ProgressEvent {
                    stage: "text_sentiment".into(),
                    label: format!("Analyzing text sentiment: {}/{}", i + 1, transcript.len()),
                    current: (i as f32 / transcript.len() as f32 * 100.0) as u32,
                    total: 100,
                    detail: None,
                });
            }

            let encoding = tokenizer
                .encode(t_seg.text.as_str(), true)
                .map_err(|e| CliptzyError::Model(format!("Tokenizer encode failed: {}", e)))?;

            let input_ids = encoding.get_ids().to_vec();
            let attention_mask = encoding.get_attention_mask().to_vec();

            // Handle chunking (max 512 tokens)
            let max_tokens = 512;
            let chunks_count = (input_ids.len() + max_tokens - 1) / max_tokens;
            if chunks_count == 0 {
                continue;
            }

            let chunk_duration = (t_seg.end - t_seg.start) / chunks_count as f64;

            for c in 0..chunks_count {
                let start_idx = c * max_tokens;
                let end_idx = (start_idx + max_tokens).min(input_ids.len());
                let slice_ids = &input_ids[start_idx..end_idx];
                let slice_mask = &attention_mask[start_idx..end_idx];

                let seq_len = slice_ids.len();
                let mut input_ids_tensor = Array2::<i64>::zeros((1, seq_len));
                let mut mask_tensor = Array2::<i64>::zeros((1, seq_len));

                for j in 0..seq_len {
                    input_ids_tensor[[0, j]] = slice_ids[j] as i64;
                    mask_tensor[[0, j]] = slice_mask[j] as i64;
                }

                let ids_ort = ort::value::Tensor::from_array(input_ids_tensor)
                    .map_err(|e| CliptzyError::Model(format!("Tensor error: {}", e)))?;
                let mask_ort = ort::value::Tensor::from_array(mask_tensor)
                    .map_err(|e| CliptzyError::Model(format!("Tensor error: {}", e)))?;

                let inputs = ort::inputs![
                    "input_ids" => ids_ort,
                    "attention_mask" => mask_ort
                ];

                if let Ok(outputs) = session.run(inputs) {
                    if let Ok((_shape, logits)) = outputs["logits"].try_extract_tensor::<f32>() {
                        // Logits is [1, 4]
                        let max_logit = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
                        let exp_sum: f32 = logits.iter().map(|&x| (x - max_logit).exp()).sum();

                        let mut max_prob = 0.0;
                        let mut best_label = None;

                        for (idx, &logit) in logits.iter().enumerate() {
                            let prob = (logit - max_logit).exp() / exp_sum;
                            if prob > max_prob {
                                max_prob = prob;
                                if let Some(emotion) = map_roberta_to_emotion(idx) {
                                    best_label = Some(emotion);
                                }
                            }
                        }

                        if let Some(emotion) = best_label {
                            // Only record strong sentiments
                            if max_prob > 0.4 {
                                segments.push(AnalysisSegment {
                                    start_time: t_seg.start + (c as f64 * chunk_duration),
                                    end_time: t_seg.start + ((c + 1) as f64 * chunk_duration),
                                    emotion,
                                    score: max_prob,
                                    bounding_box: None,
                                });
                            }
                        }
                    }
                }
            }
        }

        info!("Extracted {} text sentiments from RoBERTa", segments.len());
        Ok(segments)
    }
}
