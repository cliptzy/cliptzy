// src-tauri/src/supabase.rs
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

use crate::error::CliptzyError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub access_token: String,
    pub refresh_token: String,
    pub user: Option<User>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub email: Option<String>,
    pub user_metadata: Option<serde_json::Value>,
}

pub struct SupabaseClient {
    pub url: String,
    pub key: String,
    pub reqwest: Client,
    pub session: std::sync::Mutex<Option<Session>>,
    available: bool,
}

impl SupabaseClient {
    pub fn new() -> Result<Self, CliptzyError> {
        dotenvy::dotenv().ok();
        let url = std::env::var("SUPABASE_URL")
            .map_err(|_| CliptzyError::Config("SUPABASE_URL not set".into()))?;
        let key = std::env::var("SUPABASE_KEY")
            .or_else(|_| std::env::var("SUPABASE_PUBLISHABLE_KEY"))
            .map_err(|_| CliptzyError::Config("SUPABASE_KEY not set".into()))?;

        let client = Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "apikey",
                    key.parse().map_err(|_| {
                        CliptzyError::Config("invalid SUPABASE_KEY header value".into())
                    })?,
                );
                headers.insert(
                    "Authorization",
                    format!("Bearer {}", key).parse().map_err(|_| {
                        CliptzyError::Config("invalid Authorization header value".into())
                    })?,
                );
                headers
            })
            .build()
            .map_err(|e| CliptzyError::Supabase(format!("failed to build http client: {}", e)))?;

        let instance = Self {
            url,
            key,
            reqwest: client,
            session: std::sync::Mutex::new(None),
            available: true,
        };

        instance.load_session();
        Ok(instance)
    }

    /// Offline stub when Supabase env vars are missing or invalid at startup.
    pub fn offline() -> Self {
        Self {
            url: String::new(),
            key: String::new(),
            reqwest: Client::new(),
            session: std::sync::Mutex::new(None),
            available: false,
        }
    }

    pub fn is_available(&self) -> bool {
        self.available
    }

    fn require_available(&self) -> Result<(), CliptzyError> {
        if self.available {
            Ok(())
        } else {
            Err(CliptzyError::Supabase(
                "supabase tidak dikonfigurasi. aplikasi berjalan dalam mode offline".into(),
            ))
        }
    }

    fn require_session(&self) -> Result<(String, String), CliptzyError> {
        let user_id = self
            .get_user_id()
            .ok_or_else(|| CliptzyError::Supabase("not logged in".into()))?;
        let token = self
            .get_access_token()
            .ok_or_else(|| CliptzyError::Supabase("not logged in".into()))?;
        Ok((user_id, token))
    }

    pub fn session_file_path() -> PathBuf {
        crate::paths::app_data_dir()
            .join("cred")
            .join("supabase_session.json")
    }

    pub fn load_session(&self) {
        if let Ok(data) = std::fs::read_to_string(Self::session_file_path()) {
            if let Ok(session) = serde_json::from_str::<Session>(&data) {
                *self.session.lock().unwrap_or_else(|e| e.into_inner()) = Some(session);
            }
        }
    }

    pub fn save_session(&self) {
        if let Some(session) = &*self.session.lock().unwrap_or_else(|e| e.into_inner()) {
            if let Ok(data) = serde_json::to_string(session) {
                let path = Self::session_file_path();
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent).ok();
                }
                std::fs::write(path, data).ok();
            }
        } else {
            let path = Self::session_file_path();
            if path.exists() {
                std::fs::remove_file(path).ok();
            }
        }
    }

    pub fn get_access_token(&self) -> Option<String> {
        self.session
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
            .map(|s| s.access_token.clone())
    }

    pub fn get_user_id(&self) -> Option<String> {
        self.session
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
            .and_then(|s| s.user.as_ref().map(|u| u.id.clone()))
    }

    pub fn get_user_email(&self) -> Option<String> {
        self.session
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
            .and_then(|s| s.user.as_ref().and_then(|u| u.email.clone()))
    }

    pub fn get_user_display_name(&self) -> Option<String> {
        self.session
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
            .and_then(|s| {
                s.user.as_ref().and_then(|u| {
                    u.user_metadata.as_ref().and_then(|m| {
                        m.get("full_name")
                            .or_else(|| m.get("name"))
                            .and_then(|v| v.as_str().map(String::from))
                    })
                })
            })
    }

    pub fn get_user_avatar_url(&self) -> Option<String> {
        self.session
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
            .and_then(|s| {
                s.user.as_ref().and_then(|u| {
                    u.user_metadata.as_ref().and_then(|m| {
                        m.get("avatar_url")
                            .or_else(|| m.get("picture"))
                            .and_then(|v| v.as_str().map(String::from))
                    })
                })
            })
    }

    pub async fn login_with_google(&self) -> Result<bool, CliptzyError> {
        self.require_available()?;

        let verifier_bytes: [u8; 32] = rand::random();
        let code_verifier = URL_SAFE_NO_PAD.encode(verifier_bytes);

        let mut hasher = Sha256::new();
        hasher.update(code_verifier.as_bytes());
        let code_challenge = URL_SAFE_NO_PAD.encode(hasher.finalize());

        let auth_url = crate::auth::oauth_server::build_google_auth_url(&self.url, &code_challenge);

        opener::open(&auth_url)
            .map_err(|e| CliptzyError::Supabase(format!("failed to open browser: {}", e)))?;

        let code = crate::auth::oauth_server::listen_for_auth_code().await?;

        let token_url = format!("{}/auth/v1/token?grant_type=pkce", self.url);
        let resp = self
            .reqwest
            .post(&token_url)
            .json(&serde_json::json!({
                "auth_code": code,
                "code_verifier": code_verifier,
            }))
            .send()
            .await
            .map_err(|e| CliptzyError::Supabase(format!("token exchange request failed: {}", e)))?;

        if !resp.status().is_success() {
            let error = resp.text().await.unwrap_or_default();
            return Err(CliptzyError::Supabase(format!(
                "failed to exchange token: {}",
                error
            )));
        }

        let session: Session = resp
            .json()
            .await
            .map_err(|e| CliptzyError::Supabase(format!("invalid token response: {}", e)))?;
        *self.session.lock().unwrap_or_else(|e| e.into_inner()) = Some(session);
        self.save_session();

        Ok(true)
    }

    pub async fn logout(&self) -> Result<(), CliptzyError> {
        if let Some(t) = self.get_access_token() {
            let logout_url = format!("{}/auth/v1/logout", self.url);
            let _ = self
                .reqwest
                .post(&logout_url)
                .header("Authorization", format!("Bearer {}", t))
                .send()
                .await;
        }
        *self.session.lock().unwrap_or_else(|e| e.into_inner()) = None;
        self.save_session();
        Ok(())
    }

    pub async fn sync_config_up(
        &self,
        config_dict: serde_json::Value,
    ) -> Result<bool, CliptzyError> {
        self.require_available()?;
        let (user_id, token) = self.require_session()?;

        let url = format!("{}/rest/v1/user_configs", self.url);
        let payload = serde_json::json!({
            "user_id": user_id,
            "config": config_dict
        });

        let resp = self
            .reqwest
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Prefer", "resolution=merge-duplicates")
            .json(&payload)
            .send()
            .await
            .map_err(|e| CliptzyError::Supabase(format!("sync config up request failed: {}", e)))?;

        if resp.status().is_success() {
            Ok(true)
        } else {
            Err(CliptzyError::Supabase(format!(
                "failed to sync config up: {}",
                resp.status()
            )))
        }
    }

    pub async fn sync_config_down(&self) -> Result<Option<serde_json::Value>, CliptzyError> {
        self.require_available()?;
        let (user_id, token) = self.require_session()?;

        let url = format!(
            "{}/rest/v1/user_configs?user_id=eq.{}&select=config",
            self.url, user_id
        );

        let resp = self
            .reqwest
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| {
                CliptzyError::Supabase(format!("sync config down request failed: {}", e))
            })?;

        if !resp.status().is_success() {
            return Err(CliptzyError::Supabase(format!(
                "failed to sync config down: {}",
                resp.status()
            )));
        }

        let data: Vec<serde_json::Value> = resp
            .json()
            .await
            .map_err(|e| CliptzyError::Supabase(format!("invalid sync config response: {}", e)))?;

        if let Some(row) = data.into_iter().next() {
            if let Some(config) = row.get("config") {
                return Ok(Some(config.clone()));
            }
        }

        Ok(None)
    }

    pub async fn upload_file(
        &self,
        local_path: &PathBuf,
        remote_filename: &str,
    ) -> Result<bool, CliptzyError> {
        self.require_available()?;
        let (user_id, token) = self.require_session()?;

        if !local_path.exists() {
            return Ok(false);
        }

        let file_data = std::fs::read(local_path)?;
        if file_data.is_empty() {
            return Ok(false);
        }

        let bucket_path = format!("{}/{}", user_id, remote_filename);
        let url = format!("{}/storage/v1/object/user_files/{}", self.url, bucket_path);

        let resp = self
            .reqwest
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("x-upsert", "true")
            .header("Content-Type", "application/octet-stream")
            .body(file_data)
            .send()
            .await
            .map_err(|e| CliptzyError::Supabase(format!("upload request failed: {}", e)))?;

        if resp.status().is_success() {
            Ok(true)
        } else {
            let status = resp.status();
            let error = resp.text().await.unwrap_or_default();
            Err(CliptzyError::Supabase(format!(
                "upload failed: {} - {}",
                status, error
            )))
        }
    }

    pub async fn download_file(
        &self,
        remote_filename: &str,
        local_path: &PathBuf,
    ) -> Result<bool, CliptzyError> {
        self.require_available()?;
        let (user_id, token) = self.require_session()?;

        let bucket_path = format!("{}/{}", user_id, remote_filename);
        let url = format!("{}/storage/v1/object/user_files/{}", self.url, bucket_path);

        let resp = self
            .reqwest
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| CliptzyError::Supabase(format!("download request failed: {}", e)))?;

        if resp.status().is_success() {
            let bytes = resp
                .bytes()
                .await
                .map_err(|e| CliptzyError::Supabase(format!("download body read failed: {}", e)))?;
            if bytes.is_empty() {
                return Ok(false);
            }

            if let Some(parent) = local_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(local_path, bytes)?;
            Ok(true)
        } else {
            Err(CliptzyError::Supabase(format!(
                "download failed: {}",
                resp.status()
            )))
        }
    }
}
