use crate::config::models::AIConfig;
use crate::error::CliptzyError;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

#[derive(Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FileFingerprint {
    pub size: u64,
    pub modified_secs: u64,
}

pub fn cache_dir(job_dir: &Path) -> PathBuf {
    job_dir.join("cache")
}

pub fn cache_file(job_dir: &Path, name: &str) -> PathBuf {
    cache_dir(job_dir).join(name)
}

pub fn fingerprint(path: &Path) -> Option<FileFingerprint> {
    let meta = std::fs::metadata(path).ok()?;
    let modified_secs = meta
        .modified()
        .ok()?
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_secs();
    Some(FileFingerprint {
        size: meta.len(),
        modified_secs,
    })
}

pub fn hash_payload(payload: &str) -> String {
    let mut hasher = DefaultHasher::new();
    payload.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

pub fn sanitize_cache_token(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

pub fn read_json_cache<T: DeserializeOwned>(path: &Path) -> Option<T> {
    if !path.exists() {
        return None;
    }
    let content = std::fs::read_to_string(path).ok()?;
    match serde_json::from_str(&content) {
        Ok(value) => Some(value),
        Err(e) => {
            log::warn!("Cache rusak di {:?}, akan diabaikan: {}", path, e);
            None
        }
    }
}

pub fn write_json_cache<T: Serialize>(path: &Path, value: &T) -> Result<(), CliptzyError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(value)?;
    std::fs::write(path, content).map_err(CliptzyError::from)?;
    log::info!("Cache disimpan ke {:?}", path);
    Ok(())
}

pub fn is_fingerprint_valid(cached: &FileFingerprint, source_path: &Path) -> bool {
    fingerprint(source_path)
        .map(|current| current == *cached)
        .unwrap_or(false)
}

pub fn ai_model_name(config: &AIConfig) -> String {
    match config.provider.as_str() {
        "ollama" => {
            if config.ollama_model.is_empty() {
                "default".to_string()
            } else {
                config.ollama_model.clone()
            }
        }
        "openai" => {
            if config.openai_model.is_empty() {
                "gpt-4o-mini".to_string()
            } else {
                config.openai_model.clone()
            }
        }
        "gemini" => {
            if config.gemini_model.is_empty() {
                "gemini-1.5-flash".to_string()
            } else {
                config.gemini_model.clone()
            }
        }
        other => other.to_string(),
    }
}
