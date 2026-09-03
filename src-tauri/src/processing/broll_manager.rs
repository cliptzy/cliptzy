use crate::error::CliptzyError;
use rand::prelude::IndexedRandom;
use std::fs;
use std::path::{Path, PathBuf};

/// Manages B-roll asset selection and retrieval
pub struct BrollManager {
    /// Directory where B-roll videos are stored
    pub broll_dir: PathBuf,
}

impl BrollManager {
    /// Create a new BrollManager with the specified directory
    pub fn new(broll_dir: &Path) -> Self {
        Self {
            broll_dir: broll_dir.to_path_buf(),
        }
    }

    /// List all B-roll video files in the directory
    pub fn list_broll_files(&self) -> Result<Vec<PathBuf>, CliptzyError> {
        let mut files = Vec::new();

        if !self.broll_dir.exists() {
            return Err(CliptzyError::Config(format!(
                "B-roll directory does not exist: {}",
                self.broll_dir.display()
            )));
        }

        for entry in fs::read_dir(&self.broll_dir)? {
            let entry = entry?;
            let path = entry.path();

            // Check if it's a file and has a video extension
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext_lower = ext.to_string_lossy().to_lowercase();
                    if matches!(ext_lower.as_str(), "mp4" | "mov" | "avi" | "mkv" | "webm") {
                        files.push(path);
                    }
                }
            }
        }

        if files.is_empty() {
            return Err(CliptzyError::Config(format!(
                "No B-roll video files found in directory: {}",
                self.broll_dir.display()
            )));
        }

        Ok(files)
    }

    /// Pick a random B-roll file from the available files
    pub fn pick_random(&self) -> Result<PathBuf, CliptzyError> {
        let files = self.list_broll_files()?;
        let mut rng = rand::rng();
        files
            .choose(&mut rng)
            .cloned()
            .ok_or_else(|| CliptzyError::Config("No B-roll files available".to_string()))
    }

    /// Pick a B-roll file by keyword (placeholder for future enhancement)
    #[allow(dead_code)]
    pub fn pick_by_keyword(&self, _keyword: &str) -> Result<PathBuf, CliptzyError> {
        // For now, just pick randomly - keyword matching can be implemented later
        self.pick_random()
    }
}
