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
            model: OnnxModelManager::new(
                "emotion_vit.onnx",
                "https://huggingface.co/Xenova/facial_emotions_image_detection/resolve/main/onnx/model.onnx",
            ),
        }
    }

    async fn ensure_model(&self, _progress: &Sender<ProgressEvent>) -> Result<(), CliptzyError> {
        self.model.ensure_loaded().await
    }

    pub fn run_inference(
        &self,
        image: &image::DynamicImage,
    ) -> Result<(EmotionLabel, f32), CliptzyError> {
        let mut guard = self.model.get_session()?;
        let session = &mut *guard;

        // ViT expects 224x224 RGB
        let resized = image.resize_exact(224, 224, image::imageops::FilterType::Triangle);
        let rgb = resized.into_rgb8();

        let mut input_tensor: Array4<f32> = Array::zeros((1, 3, 224, 224));
        for (x, y, pixel) in rgb.enumerate_pixels() {
            // ViT Normalize: mean=[0.5, 0.5, 0.5], std=[0.5, 0.5, 0.5]
            input_tensor[[0, 0, y as usize, x as usize]] = (pixel[0] as f32 / 255.0 - 0.5) / 0.5; // R
            input_tensor[[0, 1, y as usize, x as usize]] = (pixel[1] as f32 / 255.0 - 0.5) / 0.5; // G
            input_tensor[[0, 2, y as usize, x as usize]] = (pixel[2] as f32 / 255.0 - 0.5) / 0.5;
            // B
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

        let logits = output_tensor;

        // Hitung softmax untuk mendapatkan probabilitas yang benar (0.0 - 1.0)
        let max_logit = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let exp_sum: f32 = logits.iter().map(|&x| (x - max_logit).exp()).sum();
        let softmax: Vec<f32> = logits
            .iter()
            .map(|&x| (x - max_logit).exp() / exp_sum)
            .collect();

        let mut max_idx = 0;
        let mut max_val = softmax[0];
        for (i, &val) in softmax.iter().enumerate() {
            if val > max_val {
                max_val = val;
                max_idx = i;
            }
        }

        // Mapping id2label Xenova:
        // 0: sad, 1: disgust, 2: angry, 3: neutral, 4: fear, 5: surprise, 6: happy
        let emotion = match max_idx {
            0 => EmotionLabel::Sad,
            1 => EmotionLabel::Unknown, // Disgust (Tidak ada label spesifik, map ke Unknown/Neutral)
            2 => EmotionLabel::Angry,
            3 => EmotionLabel::Neutral,
            4 => EmotionLabel::Fear,
            5 => EmotionLabel::Shock, // Surprise
            6 => EmotionLabel::Happy,
            _ => EmotionLabel::Unknown,
        };

        Ok((emotion, max_val))
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
