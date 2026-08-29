use crate::config::models::AIConfig;
use crate::error::CliptzyError;
use crate::orchestrator::pipeline::ProgressTx;
use crate::transcription::models::TranscriptionSegment;
use ort::{session::{builder::GraphOptimizationLevel, Session}, value::Tensor};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use tokenizers::Tokenizer;
use ndarray::Array2;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmotionData {
    pub text: Vec<TextEmotion>,
    #[serde(default)]
    pub voice: Vec<Value>, 
    #[serde(default)]
    pub face: Vec<Value>,  
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TextEmotion {
    pub id: usize,
    pub start: f64,
    pub end: f64,
    pub emotion: String, 
    pub intensity: f32,  
}

pub struct EmotionAnalyzer;

const EMOTION_LABELS: [&str; 7] = ["anger", "disgust", "fear", "joy", "neutral", "sadness", "surprise"];

impl EmotionAnalyzer {
    pub async fn ensure_model_exists(progress: Option<&ProgressTx>) -> Result<(PathBuf, PathBuf), CliptzyError> {
        let app_dir = crate::paths::app_data_dir();
        let models_dir = app_dir.join("models").join("emotion");
        fs::create_dir_all(&models_dir).ok();

        let model_path = models_dir.join("model.onnx");
        let tokenizer_path = models_dir.join("tokenizer.json");

        if !model_path.exists() || !tokenizer_path.exists() {
            if let Some(p) = progress {
                let _ = p.send(crate::orchestrator::pipeline::ProgressEvent {
                    stage: "emotion".into(),
                    label: "Mengunduh Model Text Emotion (Local)...".into(),
                    current: 0,
                    total: 100,
                    detail: None,
                });
            }

            let model_url = "https://huggingface.co/j-hartmann/emotion-english-distilroberta-base/resolve/main/onnx/model.onnx";
            let tok_url = "https://huggingface.co/j-hartmann/emotion-english-distilroberta-base/resolve/main/tokenizer.json";

            let model_bytes = reqwest::get(model_url).await.map_err(|e| CliptzyError::AIProvider(format!("Download err: {}", e)))?.bytes().await.map_err(|e| CliptzyError::AIProvider(format!("Download err: {}", e)))?;
            fs::write(&model_path, model_bytes).map_err(|e| CliptzyError::Io(e))?;
            
            let tok_bytes = reqwest::get(tok_url).await.map_err(|e| CliptzyError::AIProvider(format!("Download err: {}", e)))?.bytes().await.map_err(|e| CliptzyError::AIProvider(format!("Download err: {}", e)))?;
            fs::write(&tokenizer_path, tok_bytes).map_err(|e| CliptzyError::Io(e))?;
        }

        Ok((model_path, tokenizer_path))
    }

    pub async fn analyze_text_emotion(
        job_dir: &Path,
        idx: u32,
        transcript: &[TranscriptionSegment],
        _config: &AIConfig,
        progress: Option<&ProgressTx>,
    ) -> Result<EmotionData, CliptzyError> {
        let mut txt_content = String::new();
        for seg in transcript {
            txt_content.push_str(&format!("[{:.2} - {:.2}] {}\n", seg.start, seg.end, seg.text.trim()));
        }

        let txt_path = job_dir.join(format!("transcript_{}.txt", idx));
        fs::write(&txt_path, &txt_content).map_err(|e| CliptzyError::Io(e))?;

        if let Some(p) = progress {
            let _ = p.send(crate::orchestrator::pipeline::ProgressEvent {
                stage: "emotion".into(),
                label: "Menyiapkan Local Model (ONNX)...".into(),
                current: 72,
                total: 100,
                detail: None,
            });
        }

        let (model_path, tokenizer_path) = Self::ensure_model_exists(progress).await?;
        
        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| CliptzyError::AIProvider(format!("Gagal load tokenizer: {}", e)))?;
            
        let mut session = Session::builder()
            .map_err(|e| CliptzyError::AIProvider(format!("Ort err: {}", e)))?
            .with_optimization_level(GraphOptimizationLevel::Level3)
            .map_err(|e| CliptzyError::AIProvider(format!("Ort err: {}", e)))?
            .with_intra_threads(4)
            .map_err(|e| CliptzyError::AIProvider(format!("Ort err: {}", e)))?
            .commit_from_file(&model_path)
            .map_err(|e| CliptzyError::AIProvider(format!("Gagal load model ONNX: {}", e)))?;

        let mut emotion_data = EmotionData::default();

        if let Some(p) = progress {
            let _ = p.send(crate::orchestrator::pipeline::ProgressEvent {
                stage: "emotion".into(),
                label: "Menjalankan Inferensi Emosi (Lokal)...".into(),
                current: 74,
                total: 100,
                detail: None,
            });
        }

        for seg in transcript {
            let text = seg.text.trim();
            if text.is_empty() { continue; }

            let encoding = tokenizer.encode(text, true)
                .map_err(|e| CliptzyError::AIProvider(format!("Tokenize err: {}", e)))?;

            let input_ids: Vec<i64> = encoding.get_ids().iter().map(|&x| x as i64).collect();
            let attention_mask: Vec<i64> = encoding.get_attention_mask().iter().map(|&x| x as i64).collect();
            
            let shape = [1, input_ids.len()];
            
            let input_ids_array = Array2::from_shape_vec(shape, input_ids)
                .map_err(|e| CliptzyError::AIProvider(format!("Ndarray err: {}", e)))?;
                
            let attention_mask_array = Array2::from_shape_vec(shape, attention_mask)
                .map_err(|e| CliptzyError::AIProvider(format!("Ndarray err: {}", e)))?;

            let input_tensor = Tensor::from_array(input_ids_array).map_err(|e| CliptzyError::AIProvider(format!("Ort err: {}", e)))?;
            let attention_tensor = Tensor::from_array(attention_mask_array).map_err(|e| CliptzyError::AIProvider(format!("Ort err: {}", e)))?;
            let inputs = ort::inputs![
                "input_ids" => input_tensor,
                "attention_mask" => attention_tensor
            ];

            let outputs = session.run(inputs)
                .map_err(|e| CliptzyError::AIProvider(format!("Ort run err: {}", e)))?;

            let output_tensor = outputs["logits"].try_extract_tensor::<f32>()
                .map_err(|e| CliptzyError::AIProvider(format!("Ort output err: {}", e)))?;
                
            let logits = output_tensor.1;
            
            let max_logit = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            let exps: Vec<f32> = logits.iter().map(|&x| (x - max_logit).exp()).collect();
            let sum_exp: f32 = exps.iter().sum();
            let probs: Vec<f32> = exps.iter().map(|&x| x / sum_exp).collect();

            let mut max_prob = 0.0;
            let mut best_idx = 4;
            for (i, &p) in probs.iter().enumerate() {
                if p > max_prob {
                    max_prob = p;
                    best_idx = i;
                }
            }

            emotion_data.text.push(TextEmotion {
                id: seg.id as usize,
                start: seg.start,
                end: seg.end,
                emotion: EMOTION_LABELS[best_idx].to_string(),
                intensity: max_prob,
            });
        }

        let emotion_path = job_dir.join(format!("emotion_{}.json", idx));
        let serialized = serde_json::to_string_pretty(&emotion_data).unwrap();
        fs::write(&emotion_path, serialized).map_err(|e| CliptzyError::Io(e))?;

        Ok(emotion_data)
    }
}




