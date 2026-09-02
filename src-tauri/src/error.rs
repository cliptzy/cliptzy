use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliptzyError {
    #[error("Video download gagal: {0}")]
    Download(String),

    #[error("FFmpeg error (exit code {code}): {message}")]
    FFmpeg { code: i32, message: String },

    #[error("Transcription error: {0}")]
    Transcription(String),

    #[error("AI provider error: {0}")]
    AIProvider(String),

    #[error("Model/AI inference error: {0}")]
    Model(String),


    #[error("Upload error ({platform}): {message}")]
    Upload { platform: String, message: String },

    #[error("Config error: {0}")]
    Config(String),

    #[error("Supabase error: {0}")]
    Supabase(String),

    #[error("Operasi dibatalkan oleh pengguna")]
    Cancelled,

    #[error("File tidak ditemukan: {0}")]
    FileNotFound(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

impl From<CliptzyError> for String {
    fn from(e: CliptzyError) -> String {
        log::error!("Error converted to String: {:?}", e);
        e.to_string()
    }
}

impl serde::Serialize for CliptzyError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        log::error!("Command Error: {:?}", self);
        serializer.serialize_str(&self.to_string())
    }
}
