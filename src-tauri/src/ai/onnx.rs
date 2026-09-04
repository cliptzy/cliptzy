use crate::error::CliptzyError;
use once_cell::sync::OnceCell;
use ort::session::Session;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Emitter;

/// Direktori penyimpanan seluruh model ONNX / ML lokal.
fn models_dir() -> PathBuf {
    crate::paths::app_data_dir().join("models")
}

/// -------------------------------------------------------------------------
/// SINGLE SOURCE OF TRUTH — registry model ONNX
/// -------------------------------------------------------------------------
///
/// Seluruh daftar model ML lokal (nama file, URL sumber, metadata tampilan)
/// dideklarasikan DI SINI dan dipakai oleh:
///   - modul analyzer (`crate::analysis::*` + `crate::face::tracker`) saat
///     mengunduh model lewat `ensure_model_downloaded`,
///   - command Tauri (`list_onnx_models`, `download_onnx_model`,
///     `delete_onnx_model`) untuk halaman "Model ONNX" di Settings.
///
/// Frontend (`src/constants/onnxModels.ts`) hanyalah cermin pasif dari data
/// ini untuk keperluan render awal; status & ukuran nyata selalu dikembalikan
/// oleh `list_onnx_models`.
#[derive(Debug, Clone, Serialize)]
pub struct OnnxModelInfo {
    /// ID unik — sama persis dengan key yang dipakai frontend (`OnnxModelKind`).
    pub id: &'static str,
    /// Nama file yang disimpan di `AppData/models/`.
    pub file: &'static str,
    /// URL sumber (HuggingFace / GitHub).
    pub url: &'static str,
    /// Nama tampilan ramah pengguna.
    pub display_name: &'static str,
    /// Kategori fungsional.
    pub category: &'static str,
    /// Deskripsi singkat.
    pub description: &'static str,
    /// Estimasi ukuran untuk tampilan.
    pub approx_size: &'static str,
    /// Tag / label output relevan.
    pub tags: &'static [&'static str],
}

pub const ONNX_MODEL_REGISTRY: &[OnnxModelInfo] = &[
    OnnxModelInfo {
        id: "visual",
        file: "emotion_vit.onnx",
        url: "https://huggingface.co/Xenova/facial_emotions_image_detection/resolve/main/onnx/model.onnx",
        display_name: "Visual Emotion (ViT)",
        category: "Visual",
        description: "Vision Transformer untuk mendeteksi emosi dari wajah pada frame video (224×224 RGB).",
        approx_size: "~330 MB",
        tags: &["Happy", "Sad", "Angry", "Fear", "Shock", "Neutral"],
    },
    OnnxModelInfo {
        id: "voice",
        file: "wav2vec2_superb_er.onnx",
        url: "https://huggingface.co/onnx-community/wav2vec2-base-superb-er-ONNX/resolve/main/onnx/model.onnx",
        display_name: "Voice Emotion (Wav2Vec2)",
        category: "Voice",
        description: "Wav2Vec2 SUPERB Emotion Recognition untuk mendeteksi emosi dari suara (16 kHz).",
        approx_size: "~380 MB",
        tags: &["Neutral", "Happy", "Angry", "Sad"],
    },
    OnnxModelInfo {
        id: "audio",
        file: "ast_audioset.onnx",
        url: "https://huggingface.co/onnx-community/ast-finetuned-audioset-10-10-0.4593-ONNX/resolve/main/onnx/model.onnx",
        display_name: "Audio Event (AST)",
        category: "Audio",
        description: "Audio Spectrogram Transformer untuk deteksi event audio (tawa, teriakan, tembakan).",
        approx_size: "~350 MB",
        tags: &["Laughter", "Crying", "Scream", "Explosion"],
    },
    OnnxModelInfo {
        id: "text",
        file: "twitter_roberta_emotion.onnx",
        url: "https://huggingface.co/onnx-community/twitter-roberta-base-emotion-ONNX/resolve/main/onnx/model.onnx",
        display_name: "Text Sentiment (RoBERTa)",
        category: "Text",
        description: "Twitter-RoBERTa untuk mendeteksi sentimen emosi dari transkrip subtitle.",
        approx_size: "~500 MB",
        tags: &["Angry", "Joy", "Optimism", "Sad"],
    },
    OnnxModelInfo {
        id: "text_tokenizer",
        file: "twitter_roberta_tokenizer.onnx",
        url: "https://huggingface.co/onnx-community/twitter-roberta-base-emotion-ONNX/resolve/main/tokenizer.json",
        display_name: "RoBERTa Tokenizer",
        category: "Text",
        description: "Asset tokenizer pendukung model Text Sentiment (sidecar, diunduh otomatis).",
        approx_size: "~3.6 MB",
        tags: &["Tokenizer"],
    },
    OnnxModelInfo {
        id: "face",
        file: "seeta_fd_frontal_v1.0.bin",
        url: "https://github.com/atomashpolskiy/rustface/raw/master/model/seeta_fd_frontal_v1.0.bin",
        display_name: "Face Detector (SeetaFace)",
        category: "Visual",
        description: "Model rustface (SeetaFace) untuk deteksi bounding box wajah pada keyframes.",
        approx_size: "~2 MB",
        tags: &["Face detection"],
    },
];

/// Cari satu model berdasarkan `id`-nya.
pub fn find_model(id: &str) -> Option<&'static OnnxModelInfo> {
    ONNX_MODEL_REGISTRY.iter().find(|m| m.id == id)
}

/// Cari satu model berdasarkan nama file-nya.
fn find_model_by_file(file: &str) -> Option<&'static OnnxModelInfo> {
    ONNX_MODEL_REGISTRY.iter().find(|m| m.file == file)
}

/// Path lengkap sebuah model di dalam `AppData/models/`.
pub fn model_path_for(file: &str) -> PathBuf {
    models_dir().join(file)
}

/// -------------------------------------------------------------------------
/// DOWNLOAD helper
/// -------------------------------------------------------------------------

/// Unduh model ke `AppData/models/{file_name}` bila belum ada.
///
/// Tetap backward-compatible: menerima `file_name` + `url` eksplisit seperti
/// signature lama. Bila `file_name` dikenal di registry, URL registry yang
/// diprioritaskan (single source of truth).
pub async fn ensure_model_downloaded(file_name: &str, url: &str) -> Result<PathBuf, String> {
    let effective_url = find_model_by_file(file_name)
        .map(|m| m.url)
        .unwrap_or(url);

    let model_dir = models_dir();
    std::fs::create_dir_all(&model_dir)
        .map_err(|e| format!("Failed to create models dir: {}", e))?;

    let model_path = model_dir.join(file_name);

    if !model_path.exists() {
        log::info!(
            "Model not found. Downloading {} to {:?}",
            file_name,
            model_path
        );
        let response = reqwest::get(effective_url)
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
        let models_dir = models_dir();
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

/// Unduh model dengan laporan progres streaming (dipakai command `download_onnx_model`).
async fn download_with_progress(
    model: &OnnxModelInfo,
    on_progress: impl Fn(u32) + Send + Sync,
) -> Result<(), CliptzyError> {
    use futures_util::StreamExt;
    use tokio::io::AsyncWriteExt;

    let path = model_path_for(model.file);
    if path.exists() {
        return Ok(());
    }

    std::fs::create_dir_all(models_dir()).map_err(CliptzyError::Io)?;

    let response = reqwest::get(model.url)
        .await
        .map_err(|e| CliptzyError::Model(format!("Download failed for {}: {}", model.file, e)))?;

    let total = response.content_length().unwrap_or(0);
    let tmp = path.with_extension("part");
    let mut file = tokio::fs::File::create(&tmp)
        .await
        .map_err(|e| CliptzyError::Model(format!("Create temp failed: {}", e)))?;

    let mut stream = response.bytes_stream();
    let mut downloaded: u64 = 0;
    while let Some(chunk) = stream
        .next()
        .await
        .transpose()
        .map_err(|e| CliptzyError::Model(format!("Stream error: {}", e)))?
    {
        file.write_all(&chunk)
            .await
            .map_err(|e| CliptzyError::Model(format!("Write failed for {}: {}", model.file, e)))?;
        downloaded += chunk.len() as u64;

        if total > 0 {
            let pct = ((downloaded as f64 / total as f64) * 100.0).clamp(0.0, 100.0) as u32;
            on_progress(pct);
        }
    }

    file.flush()
        .await
        .map_err(|e| CliptzyError::Model(format!("Flush failed: {}", e)))?;
    drop(file);

    std::fs::rename(&tmp, &path)
        .map_err(|e| CliptzyError::Model(format!("Rename failed for {}: {}", model.file, e)))?;

    log::info!("Model {} downloaded successfully.", model.file);
    Ok(())
}

/// -------------------------------------------------------------------------
/// TAURI COMMANDS (dipakai halaman "Model ONNX" di Settings)
/// -------------------------------------------------------------------------

/// Status nyata satu model di disk — kontrak yang dipakai frontend
/// (`list_onnx_models`), agar UI menampilkan ukuran & path aktual.
#[derive(Debug, Serialize)]
pub struct OnnxModelStatus {
    pub id: String,
    pub exists: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// Proyeksikan registry menjadi status disk nyata (exists, size, path).
pub fn model_statuses() -> Vec<OnnxModelStatus> {
    ONNX_MODEL_REGISTRY
        .iter()
        .map(|m| {
            let path = model_path_for(m.file);
            let exists = path.exists();
            let size_bytes = if exists {
                std::fs::metadata(&path).ok().map(|md| md.len())
            } else {
                None
            };
            OnnxModelStatus {
                id: m.id.to_string(),
                exists,
                size_bytes,
                path: exists.then(|| path.to_string_lossy().into_owned()),
            }
        })
        .collect()
}

/// Daftar seluruh model ONNX beserta status keberadaan + ukuran di disk.
#[tauri::command]
pub async fn list_onnx_models() -> Result<Vec<OnnxModelStatus>, CliptzyError> {
    Ok(model_statuses())
}

/// Unduh satu model (streaming) ke `AppData/models/`.
/// Memancarkan event `onnx-download-progress` untuk progress bar frontend.
#[tauri::command]
pub async fn download_onnx_model(
    app: tauri::AppHandle,
    id: String,
) -> Result<(), CliptzyError> {
    let model = find_model(&id).ok_or_else(|| {
        CliptzyError::Model(format!("Model ONNX dengan id '{}' tidak dikenal.", id))
    })?;

    let handle = app.clone();
    download_with_progress(model, move |pct| {
        let _ = handle.emit(
            "onnx-download-progress",
            crate::orchestrator::pipeline::ProgressEvent {
                stage: "onnx_download".into(),
                label: format!("Mengunduh {}...", model.file),
                current: pct,
                total: 100,
                detail: None,
            },
        );
    })
    .await?;

    Ok(())
}

/// Hapus satu model dari disk.
#[tauri::command]
pub async fn delete_onnx_model(id: String) -> Result<(), CliptzyError> {
    let model = find_model(&id).ok_or_else(|| {
        CliptzyError::Model(format!("Model ONNX dengan id '{}' tidak dikenal.", id))
    })?;

    let path = model_path_for(model.file);
    if path.exists() {
        std::fs::remove_file(&path).map_err(CliptzyError::Io)?;
        log::info!("Model {} deleted from disk.", model.file);
    }

    Ok(())
}
