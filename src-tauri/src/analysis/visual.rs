use super::{AnalysisSegment, EmotionAnalyzer, EmotionLabel};
use crate::ai::onnx::OnnxModelManager;
use crate::error::CliptzyError;
use crate::orchestrator::pipeline::ProgressEvent;
use ndarray::{Array, Array4};
use std::path::Path;
use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;

// The ONNX model loader
pub struct VisualEmotionAnalyzer {
    model: OnnxModelManager,
}

impl VisualEmotionAnalyzer {
    pub fn new() -> Self {
        Self {
            model: OnnxModelManager::from_registry("visual")
                .unwrap_or_else(|_| OnnxModelManager::new("face_emotion_detection.onnx", "")),
        }
    }

    async fn ensure_model(&self, _progress: &Sender<ProgressEvent>) -> Result<(), CliptzyError> {
        self.model.ensure_loaded().await
    }

    pub fn run_inference(
        &self,
        image: &image::DynamicImage,
    ) -> Result<(EmotionLabel, f32), CliptzyError> {
        let results = self.run_batch_inference(&[image.clone()])?;
        if let Some(probs) = results.first() {
            Ok(Self::map_probs_to_emotion(probs))
        } else {
            Err(CliptzyError::Model("Empty result from inference".into()))
        }
    }

    pub fn run_batch_inference(
        &self,
        images: &[image::DynamicImage],
    ) -> Result<Vec<[f32; 7]>, CliptzyError> {
        if images.is_empty() {
            return Ok(Vec::new());
        }

        let mut guard = self.model.get_session()?;
        let session = &mut *guard;
        let batch_size = images.len();

        let mut input_tensor: Array4<f32> = Array::zeros((batch_size, 3, 224, 224));

        for (b, image) in images.iter().enumerate() {
            let resized = image.resize_exact(224, 224, image::imageops::FilterType::Triangle);
            let rgb = resized.into_rgb8();
            for (x, y, pixel) in rgb.enumerate_pixels() {
                // ViT Normalize: mean=[0.5, 0.5, 0.5], std=[0.5, 0.5, 0.5]
                input_tensor[[b, 0, y as usize, x as usize]] =
                    (pixel[0] as f32 / 255.0 - 0.5) / 0.5; // R
                input_tensor[[b, 1, y as usize, x as usize]] =
                    (pixel[1] as f32 / 255.0 - 0.5) / 0.5; // G
                input_tensor[[b, 2, y as usize, x as usize]] =
                    (pixel[2] as f32 / 255.0 - 0.5) / 0.5; // B
            }
        }

        let tensor = ort::value::Tensor::from_array(input_tensor)
            .map_err(|e| CliptzyError::Model(format!("Tensor error: {}", e)))?;

        // HuggingFace Optimum exported models typically use "pixel_values" for image inputs
        let inputs = ort::inputs!["pixel_values" => tensor];

        let outputs = session
            .run(inputs)
            .map_err(|e| CliptzyError::Model(format!("Inference error: {}", e)))?;

        // The output for HF sequence classification is usually named "logits"
        let (_shape, output_tensor) = outputs["logits"]
            .try_extract_tensor::<f32>()
            .map_err(|e| CliptzyError::Model(format!("Output error: {}", e)))?;

        let mut results = Vec::with_capacity(batch_size);

        // Calculate softmax for each item in the batch
        for b in 0..batch_size {
            let mut logit_row = [0.0; 7];
            for i in 0..7 {
                logit_row[i] = output_tensor[b * 7 + i];
            }

            let max_logit = logit_row.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            let exp_sum: f32 = logit_row.iter().map(|&x| (x - max_logit).exp()).sum();

            let mut probs = [0.0; 7];
            for i in 0..7 {
                probs[i] = (logit_row[i] - max_logit).exp() / exp_sum;
            }
            results.push(probs);
        }

        Ok(results)
    }

    pub fn map_probs_to_emotion(probs: &[f32; 7]) -> (EmotionLabel, f32) {
        let mut max_idx = 0;
        let mut max_val = probs[0];
        for (i, &val) in probs.iter().enumerate() {
            if val > max_val {
                max_val = val;
                max_idx = i;
            }
        }

        // Jika confidence tertinggi sangat rendah (< 0.28), wajah berada dalam keadaan ambient netral
        if max_val < 0.28 {
            return (EmotionLabel::Neutral, max_val);
        }

        // Mapping id2label onnx-community/face-emotion-detection-ONNX (FER2013):
        // 0: Angry, 1: Disgust, 2: Fear, 3: Happy, 4: Sad, 5: Surprise, 6: Neutral
        let (emotion, score) = match max_idx {
            0 => {
                // Kalibrasi Anti-False Angry:
                // Gamer/streamer sering mengerutkan alis saat fokus melihat monitor.
                // FER2013 sering mendeteksi ini sebagai Angry dengan probabilitas tipis (0.25 - 0.38).
                // Hanya klasifikasikan sebagai Angry jika benar-benar intens (>= 0.42) dan selisih terhadap Neutral >= 0.10.
                let neutral_prob = probs[6];
                if max_val >= 0.42 && (max_val - neutral_prob) >= 0.10 {
                    (EmotionLabel::Angry, max_val)
                } else {
                    // Ekspresi konsentrasi / resting face normal -> Neutral
                    (EmotionLabel::Neutral, neutral_prob.max(max_val * 0.75))
                }
            }
            1 => {
                // Disgust sering kali artefak pergerakan bibir saat berbicara
                (EmotionLabel::Neutral, probs[6].max(0.40))
            }
            2 => (EmotionLabel::Fear, max_val),
            3 => (EmotionLabel::Happy, max_val),
            4 => {
                // Sadness ringan sering kali hanya wajah menunduk melihat keyboard
                let neutral_prob = probs[6];
                if max_val >= 0.36 && (max_val - neutral_prob) >= 0.08 {
                    (EmotionLabel::Sad, max_val)
                } else {
                    (EmotionLabel::Neutral, neutral_prob.max(max_val * 0.75))
                }
            }
            5 => (EmotionLabel::Shock, max_val), // Surprise -> Shock
            6 => (EmotionLabel::Neutral, max_val),
            _ => (EmotionLabel::Neutral, max_val),
        };

        (emotion, score)
    }
}

#[async_trait::async_trait]
impl EmotionAnalyzer for VisualEmotionAnalyzer {
    fn name(&self) -> &str {
        "Visual Emotion Analyzer (ONNX)"
    }

    async fn analyze(
        &self,
        input_path: &Path,
        cancel: &CancellationToken,
        progress: &Sender<ProgressEvent>,
    ) -> Result<Vec<AnalysisSegment>, CliptzyError> {
        self.ensure_model(progress).await?;

        // Gunakan face tracker untuk mengekstrak frame dan mendeteksi wajah
        // Daripada menulis ulang kode ffmpeg dan face detection di sini.
        let (_keyframes, segments_opt) = crate::face::tracker::get_face_keyframes(
            input_path,
            1.0,                // 1 fps
            "fast".to_string(), // mode fast cukup untuk emotion detection
            None,               // app_handle tidak diperlukan karena progress channel berbeda
            cancel.clone(),
            Some(self),
        )
        .await?;

        Ok(segments_opt.unwrap_or_default())
    }
}
