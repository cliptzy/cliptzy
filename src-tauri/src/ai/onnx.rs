use crate::error::CliptzyError;
use once_cell::sync::OnceCell;
use ort::session::Session;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Mutex;

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
        file: "face_emotion_detection.onnx",
        url: "https://huggingface.co/onnx-community/face-emotion-detection-ONNX/resolve/main/onnx/model.onnx",
        display_name: "Face Emotion Detection (ViT)",
        category: "Visual",
        description: "Vision Transformer (ViT-Base FER2013) untuk mendeteksi 7 ekspresi wajah (224×224 RGB).",
        approx_size: "~343 MB",
        tags: &["Angry", "Disgust", "Fear", "Happy", "Sad", "Surprise", "Neutral"],
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
        file: "multilingual_emotion.onnx",
        url: "https://huggingface.co/onnx-community/tanaos-emotion-detection-v1-ONNX/resolve/main/onnx/model.onnx",
        display_name: "Multilingual Text Emotion (MiniLM)",
        category: "Text",
        description: "Multilingual MiniLM-L12 untuk mendeteksi 8 emosi teks (Indonesia, Inggris, dll).",
        approx_size: "~180 MB",
        tags: &["Joy", "Anger", "Fear", "Sadness", "Surprise", "Disgust", "Neutral"],
    },
    OnnxModelInfo {
        id: "text_tokenizer",
        file: "multilingual_emotion_tokenizer.json",
        url: "https://huggingface.co/onnx-community/tanaos-emotion-detection-v1-ONNX/resolve/main/tokenizer.json",
        display_name: "Multilingual Emotion Tokenizer",
        category: "Text",
        description: "Asset tokenizer pendukung model Multilingual Text Emotion (sidecar, diunduh otomatis).",
        approx_size: "~17 MB",
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
/// STREAMING DOWNLOAD HELPER (Zero-Cost Memory & Safe Atomic Rename)
/// -------------------------------------------------------------------------

/// Unduh file secara streaming chunk-by-chunk ke file `.part` lalu rename
/// secara atomik ke `target_path`. Mencegah lonjakan alokasi RAM (350–500MB)
/// dan menjamin tidak ada file rusak/setengah unduh pada disk.
pub async fn download_file_streaming(
    url: &str,
    target_path: &std::path::Path,
    on_progress: Option<&(dyn Fn(u32) + Send + Sync)>,
) -> Result<(), CliptzyError> {
    use futures_util::StreamExt;
    use tokio::io::AsyncWriteExt;

    if let Some(parent) = target_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(CliptzyError::Io)?;
    }

    let response = reqwest::get(url)
        .await
        .map_err(|e| CliptzyError::Model(format!("Download failed from {}: {}", url, e)))?;

    if !response.status().is_success() {
        return Err(CliptzyError::Model(format!(
            "Download failed with HTTP status: {}",
            response.status()
        )));
    }

    let total = response.content_length().unwrap_or(0);
    let tmp = target_path.with_extension("part");
    let mut file = tokio::fs::File::create(&tmp)
        .await
        .map_err(|e| CliptzyError::Model(format!("Create temp file failed: {}", e)))?;

    let mut stream = response.bytes_stream();
    let mut downloaded: u64 = 0;
    while let Some(chunk) = stream
        .next()
        .await
        .transpose()
        .map_err(|e| CliptzyError::Model(format!("Stream read error: {}", e)))?
    {
        file.write_all(&chunk)
            .await
            .map_err(|e| CliptzyError::Model(format!("Stream write failed: {}", e)))?;
        downloaded += chunk.len() as u64;

        if total > 0 {
            if let Some(cb) = on_progress {
                let pct = ((downloaded as f64 / total as f64) * 100.0).clamp(0.0, 100.0) as u32;
                cb(pct);
            }
        }
    }

    file.flush()
        .await
        .map_err(|e| CliptzyError::Model(format!("Flush failed: {}", e)))?;
    drop(file);

    tokio::fs::rename(&tmp, target_path)
        .await
        .map_err(|e| CliptzyError::Model(format!("Atomic rename failed: {}", e)))?;

    log::info!("Model saved successfully to {:?}", target_path);
    Ok(())
}

/// Unduh model ke `AppData/models/{file_name}` bila belum ada.
/// Menggunakan streaming chunked download terpadu dan mengembalikan `Result<PathBuf, CliptzyError>`.
pub async fn ensure_model_downloaded(
    file_name: &str,
    url: &str,
) -> Result<PathBuf, CliptzyError> {
    let effective_url = find_model_by_file(file_name)
        .map(|m| m.url)
        .unwrap_or(url);

    let model_path = model_path_for(file_name);
    if !model_path.exists() {
        log::info!(
            "Model not found on disk. Downloading {} to {:?}",
            file_name,
            model_path
        );
        download_file_streaming(effective_url, &model_path, None).await?;
        log::info!("Model {} downloaded successfully.", file_name);
    }

    Ok(model_path)
}

/// -------------------------------------------------------------------------
/// ONNX MODEL MANAGER & SESSION MANAGEMENT
/// -------------------------------------------------------------------------

pub struct OnnxModelManager {
    pub model_name: String,
    pub model_url: String,
    pub model_path: PathBuf,
    pub session: OnceCell<Mutex<Session>>,
}

impl OnnxModelManager {
    pub fn new(model_name: impl Into<String>, model_url: impl Into<String>) -> Self {
        let name = model_name.into();
        let path = model_path_for(&name);

        Self {
            model_name: name,
            model_url: model_url.into(),
            model_path: path,
            session: OnceCell::new(),
        }
    }

    /// Konstruktor cerdas berorientasi SSOT: mengambil nama file & URL langsung dari registry.
    pub fn from_registry(id: &str) -> Result<Self, CliptzyError> {
        let info = find_model(id).ok_or_else(|| {
            CliptzyError::Model(format!("Model dengan ID '{}' tidak terdaftar di registry", id))
        })?;
        Ok(Self::new(info.file, info.url))
    }

    pub async fn ensure_loaded(&self) -> Result<(), CliptzyError> {
        if self.session.get().is_some() {
            return Ok(());
        }

        ensure_model_downloaded(&self.model_name, &self.model_url).await?;

        // Double check untuk menghindari kompilasi sesi ganda
        if self.session.get().is_some() {
            return Ok(());
        }

        // DirectML GPU acceleration dengan graceful fallback ke CPU
        let session = {
            #[cfg(target_os = "windows")]
            {
                use ort::ep;
                let try_dml = (|| -> Result<Session, CliptzyError> {
                    let mut builder = Session::builder()
                        .map_err(|e| CliptzyError::Model(format!("DirectML builder error: {}", e)))?;
                    builder = builder
                        .with_execution_providers([ep::DirectML::default().build()])
                        .map_err(|e| CliptzyError::Model(format!("DirectML provider error: {}", e)))?;
                    let sess = builder
                        .commit_from_file(&self.model_path)
                        .map_err(|e| CliptzyError::Model(format!("DirectML commit error: {}", e)))?;
                    Ok(sess)
                })();

                match try_dml {
                    Ok(sess) => {
                        log::info!(
                            "Model ONNX '{}' berhasil dimuat dengan DirectML (GPU)",
                            self.model_name
                        );
                        sess
                    }
                    Err(err) => {
                        log::warn!(
                            "DirectML execution provider gagal untuk '{}' ({}), fallback ke CPU",
                            self.model_name,
                            err
                        );
                        Session::builder()
                            .map_err(|e| CliptzyError::Model(format!("Failed to build CPU session: {}", e)))?
                            .commit_from_file(&self.model_path)
                            .map_err(|e| CliptzyError::Model(format!("Failed to load ONNX model on CPU: {}", e)))?
                    }
                }
            }
            #[cfg(not(target_os = "windows"))]
            {
                Session::builder()
                    .map_err(|e| CliptzyError::Model(format!("Failed to build session: {}", e)))?
                    .commit_from_file(&self.model_path)
                    .map_err(|e| CliptzyError::Model(format!("Failed to load ONNX model: {}", e)))?
            }
        };

        // Jika thread lain telah mengisi OnceCell secara bersamaan, abaikan Err
        let _ = self.session.set(Mutex::new(session));

        Ok(())
    }

    pub fn get_session(&self) -> Result<std::sync::MutexGuard<'_, Session>, CliptzyError> {
        self.session
            .get()
            .ok_or_else(|| CliptzyError::Model(format!("Model '{}' belum dimuat", self.model_name)))?
            .lock()
            .map_err(|_| CliptzyError::Model(format!("Gagal mengunci mutex sesi model '{}'", self.model_name)))
    }
}

/// -------------------------------------------------------------------------
/// DOMAIN MODEL STATUS CONTRACT
/// -------------------------------------------------------------------------

/// Status nyata satu model di disk beserta metadata lengkap dari registry
/// untuk dikirim langsung ke frontend via `list_onnx_models` (Single Source of Truth).
#[derive(Debug, Serialize, Clone)]
pub struct OnnxModelStatus {
    pub id: String,
    pub file: String,
    pub url: String,
    pub display_name: String,
    pub category: String,
    pub description: String,
    pub approx_size: String,
    pub tags: Vec<String>,
    pub exists: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// Proyeksikan registry menjadi status disk nyata dan metadata lengkap (exists, size, path).
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
                file: m.file.to_string(),
                url: m.url.to_string(),
                display_name: m.display_name.to_string(),
                category: m.category.to_string(),
                description: m.description.to_string(),
                approx_size: m.approx_size.to_string(),
                tags: m.tags.iter().map(|&s| s.to_string()).collect(),
                exists,
                size_bytes,
                path: exists.then(|| path.to_string_lossy().into_owned()),
            }
        })
        .collect()
}

/// Unduh model berdasarkan id dengan streaming dan event progress Tauri.
pub async fn download_onnx_model_streaming(
    app: &tauri::AppHandle,
    id: &str,
) -> Result<(), CliptzyError> {
    use tauri::Emitter;

    let model = find_model(id).ok_or_else(|| {
        CliptzyError::Model(format!("Model ONNX dengan id '{}' tidak dikenal.", id))
    })?;

    let path = model_path_for(model.file);
    if path.exists() {
        return Ok(());
    }

    let handle = app.clone();
    let file_name = model.file;
    download_file_streaming(model.url, &path, Some(&move |pct| {
        let _ = handle.emit(
            "onnx-download-progress",
            crate::orchestrator::pipeline::ProgressEvent {
                stage: "onnx_download".into(),
                label: format!("Mengunduh {}...", file_name),
                current: pct,
                total: 100,
                detail: None,
            },
        );
    }))
    .await
}

/// Hapus file model dari disk berdasarkan id.
pub fn delete_model_file(id: &str) -> Result<(), CliptzyError> {
    let model = find_model(id).ok_or_else(|| {
        CliptzyError::Model(format!("Model ONNX dengan id '{}' tidak dikenal.", id))
    })?;

    let path = model_path_for(model.file);
    if path.exists() {
        std::fs::remove_file(&path).map_err(CliptzyError::Io)?;
        log::info!("Model {} berhasil dihapus dari disk.", model.file);
    }

    Ok(())
}

// Re-export commands from commands::ai for backward compatibility
pub use crate::commands::ai::{delete_onnx_model, download_onnx_model, list_onnx_models};
