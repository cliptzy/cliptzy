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
        let model_info = crate::ai::onnx::find_model("text");
        let file = model_info.map(|m| m.file).unwrap_or("multilingual_emotion.onnx");
        let url = model_info.map(|m| m.url).unwrap_or("");
        Self {
            model: OnnxModelManager::new(file, url),
        }
    }

    async fn ensure_assets(
        &self,
        _progress: &Sender<ProgressEvent>,
    ) -> Result<std::path::PathBuf, CliptzyError> {
        self.model.ensure_loaded().await?;

        let tokenizer_info = crate::ai::onnx::find_model("text_tokenizer");
        let tok_file = tokenizer_info
            .map(|m| m.file)
            .unwrap_or("multilingual_emotion_tokenizer.json");
        let tok_url = tokenizer_info.map(|m| m.url).unwrap_or("");

        let tokenizer_path = ensure_model_downloaded(tok_file, tok_url).await?;
        Ok(tokenizer_path)
    }
}

// Map tanaos-emotion-detection-v1 (Multilingual MiniLM) classes to EmotionLabel:
// 0: joy, 1: anger, 2: fear, 3: sadness, 4: surprise, 5: disgust, 6: excitement, 7: neutral
fn map_multilingual_emotion(idx: usize) -> Option<EmotionLabel> {
    match idx {
        0 => Some(EmotionLabel::Happy),
        1 => Some(EmotionLabel::Angry),
        2 => Some(EmotionLabel::Fear),
        3 => Some(EmotionLabel::Sad),
        4 => Some(EmotionLabel::Shock),
        5 => Some(EmotionLabel::Angry),
        6 => Some(EmotionLabel::Happy),
        7 => Some(EmotionLabel::Neutral),
        _ => None,
    }
}

#[derive(serde::Deserialize)]
struct TranscriptSegment {
    start: f64,
    end: f64,
    text: String,
}

#[derive(serde::Deserialize)]
struct TranscriptWrapper {
    #[serde(default)]
    segments: Vec<TranscriptSegment>,
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum TranscriptData {
    Wrapped(TranscriptWrapper),
    Direct(Vec<TranscriptSegment>),
}

#[async_trait::async_trait]
impl EmotionAnalyzer for TextSentimentAnalyzer {
    fn name(&self) -> &str {
        "Multilingual Text Emotion Analyzer (MiniLM ONNX)"
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

        // Parse transcript file: support either SegmentTranscriptCacheEntry or flat array
        let file_content = std::fs::read_to_string(input_path).map_err(|e| {
            CliptzyError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to read transcript at {:?}: {}", input_path, e),
            ))
        })?;

        let transcript = match serde_json::from_str::<TranscriptData>(&file_content) {
            Ok(TranscriptData::Wrapped(w)) => w.segments,
            Ok(TranscriptData::Direct(d)) => d,
            Err(e) => {
                return Err(CliptzyError::Model(format!(
                    "Failed to parse transcript JSON at {:?}: {}",
                    input_path, e
                )));
            }
        };

        if transcript.is_empty() {
            info!("Transcript is empty, returning 0 text emotions");
            return Ok(vec![]);
        }

        let mut guard = self.model.get_session()?;
        let session = &mut *guard;
        let has_token_type_ids = session.inputs().iter().any(|inp| inp.name() == "token_type_ids");

        let mut segments = Vec::new();

        for (i, t_seg) in transcript.iter().enumerate() {
            if cancel.is_cancelled() {
                return Err(CliptzyError::Cancelled);
            }

            let text = t_seg.text.trim();
            if text.is_empty() {
                continue;
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
                .encode(text, true)
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

                let outputs_res = if has_token_type_ids {
                    let type_ids = Array2::<i64>::zeros((1, seq_len));
                    let type_ort = ort::value::Tensor::from_array(type_ids)
                        .map_err(|e| CliptzyError::Model(format!("Tensor error: {}", e)))?;
                    session.run(ort::inputs![
                        "input_ids" => ids_ort,
                        "attention_mask" => mask_ort,
                        "token_type_ids" => type_ort
                    ])
                } else {
                    session.run(ort::inputs![
                        "input_ids" => ids_ort,
                        "attention_mask" => mask_ort
                    ])
                };

                if let Ok(outputs) = outputs_res {
                    if let Ok((_shape, logits)) = outputs["logits"].try_extract_tensor::<f32>() {
                        let max_logit = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
                        let exp_sum: f32 = logits.iter().map(|&x| (x - max_logit).exp()).sum();

                        let mut max_prob = 0.0;
                        let mut best_label = None;

                        for (idx, &logit) in logits.iter().enumerate() {
                            let prob = (logit - max_logit).exp() / exp_sum;
                            if prob > max_prob {
                                max_prob = prob;
                                if let Some(emotion) = map_multilingual_emotion(idx) {
                                    best_label = Some(emotion);
                                }
                            }
                        }

                        if let Some(emotion) = best_label {
                            // Only record distinct emotional expressions
                            if max_prob > 0.35 {
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

        info!("Extracted {} text emotions from Multilingual MiniLM", segments.len());
        Ok(segments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_multilingual_emotion() {
        assert_eq!(map_multilingual_emotion(0), Some(EmotionLabel::Happy)); // joy
        assert_eq!(map_multilingual_emotion(1), Some(EmotionLabel::Angry)); // anger
        assert_eq!(map_multilingual_emotion(2), Some(EmotionLabel::Fear)); // fear
        assert_eq!(map_multilingual_emotion(3), Some(EmotionLabel::Sad)); // sadness
        assert_eq!(map_multilingual_emotion(4), Some(EmotionLabel::Shock)); // surprise
        assert_eq!(map_multilingual_emotion(6), Some(EmotionLabel::Happy)); // excitement
        assert_eq!(map_multilingual_emotion(7), Some(EmotionLabel::Neutral)); // neutral
        assert_eq!(map_multilingual_emotion(99), None);
    }

    #[test]
    fn test_deserialize_transcript_formats() {
        // Flat array
        let flat_json = r#"[{"start": 1.0, "end": 2.5, "text": "Ampun bang!"}]"#;
        let res: TranscriptData = serde_json::from_str(flat_json).unwrap();
        match res {
            TranscriptData::Direct(d) => {
                assert_eq!(d.len(), 1);
                assert_eq!(d[0].text, "Ampun bang!");
            }
            _ => panic!("Expected Direct variant"),
        }

        // Wrapped cache object
        let wrapped_json = r#"{
            "whisper_model": "large-v3-turbo",
            "segments": [
                {"id": 0, "start": 0.0, "end": 2.0, "text": "Lorong kematian"}
            ]
        }"#;
        let res2: TranscriptData = serde_json::from_str(wrapped_json).unwrap();
        match res2 {
            TranscriptData::Wrapped(w) => {
                assert_eq!(w.segments.len(), 1);
                assert_eq!(w.segments[0].text, "Lorong kematian");
            }
            _ => panic!("Expected Wrapped variant"),
        }
    }

    #[test]
    fn test_load_real_tokenizer_if_present() {
        let path = crate::paths::app_data_dir()
            .join("models")
            .join("multilingual_emotion_tokenizer.json");
        if path.exists() {
            let tok = tokenizers::Tokenizer::from_file(&path).expect("Failed to load tokenizer");
            let encoding = tok
                .encode("Ampun bang, jangan bunuh saya!", true)
                .expect("Failed to encode Indonesian text");
            assert!(!encoding.get_ids().is_empty());
            println!("Tokens: {:?}", encoding.get_tokens());
        }
    }
}

