use rustface::{Detector, FaceInfo};
use std::path::Path;

pub struct FaceDetectorWrapper {
    detector: Box<dyn rustface::Detector>,
}

unsafe impl Send for FaceDetectorWrapper {}
unsafe impl Sync for FaceDetectorWrapper {}

impl FaceDetectorWrapper {
    pub fn new(model_path: &Path) -> Result<Self, String> {
        if !model_path.exists() {
            return Err(format!("Model not found at {:?}", model_path));
        }

        let mut detector = rustface::create_detector(model_path.to_str().unwrap())
            .map_err(|e| format!("Failed to create face detector: {:?}", e))?;

        detector.set_min_face_size(40);
        detector.set_score_thresh(3.5);
        detector.set_pyramid_scale_factor(0.8);
        detector.set_slide_window_step(4, 4);

        Ok(Self { detector })
    }

    pub fn detect_faces(&mut self, image_data: &[u8], width: u32, height: u32) -> Vec<rustface::FaceInfo> {
        let mut img = rustface::ImageData::new(image_data, width, height);
        self.detector.detect(&mut img)
    }
}

