use crate::error::CliptzyError;
use once_cell::sync::OnceCell;
use ort::session::Session;
use std::path::PathBuf;
use std::sync::Mutex;

pub async fn ensure_model_downloaded(file_name: &str, url: &str) -> Result<PathBuf, String> {
    let model_dir = crate::paths::app_data_dir().join("models");
    std::fs::create_dir_all(&model_dir)
        .map_err(|e| format!("Failed to create models dir: {}", e))?;

    let model_path = model_dir.join(file_name);

    if !model_path.exists() {
        log::info!(
            "Model not found. Downloading {} to {:?}",
            file_name,
            model_path
        );
        let response = reqwest::get(url)
            .await
            .map_err(|e| format!("Download failed for {}: {}", file_name, e))?;

        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read bytes for {}: {}", file_name, e))?;

        std::fs::write(&model_path, bytes)
            .map_err(|e| format!("Write failed for {}: {}", file_name, e))?;

        log::info!("Model {} downloaded successfully.", file_name);
    }

    Ok(model_path)
}

pub struct OnnxModelManager {
    pub model_name: String,
    pub model_url: String,
    pub model_path: std::path::PathBuf,
    pub session: OnceCell<Mutex<Session>>,
}

impl OnnxModelManager {
    pub fn new(model_name: impl Into<String>, model_url: impl Into<String>) -> Self {
        let name = model_name.into();
        let models_dir = crate::paths::app_data_dir().join("models");
        std::fs::create_dir_all(&models_dir).ok();
        let path = models_dir.join(&name);

        Self {
            model_name: name,
            model_url: model_url.into(),
            model_path: path,
            session: OnceCell::new(),
        }
    }

    pub async fn ensure_loaded(&self) -> Result<(), CliptzyError> {
        let _ = ensure_model_downloaded(&self.model_name, &self.model_url)
            .await
            .map_err(|e| CliptzyError::Internal(e))?;

        if self.session.get().is_none() {
            let mut builder = Session::builder()
                .map_err(|e| CliptzyError::Model(format!("Failed to build session: {}", e)))?;

            #[cfg(target_os = "windows")]
            {
                use ort::ep;
                builder = builder
                    .with_execution_providers([ep::DirectML::default().build()])
                    .map_err(|e| {
                        CliptzyError::Model(format!("Failed to set execution provider: {}", e))
                    })?;
            }

            let session = builder
                .commit_from_file(&self.model_path)
                .map_err(|e| CliptzyError::Model(format!("Failed to load ONNX model: {}", e)))?;

            self.session
                .set(Mutex::new(session))
                .map_err(|_| CliptzyError::Internal("Failed to set session".into()))?;
        }

        Ok(())
    }

    pub fn get_session(&self) -> Result<std::sync::MutexGuard<'_, Session>, CliptzyError> {
        self.session
            .get()
            .ok_or_else(|| CliptzyError::Model("Model not loaded".into()))?
            .lock()
            .map_err(|_| CliptzyError::Model("Failed to lock session".into()))
    }
}
