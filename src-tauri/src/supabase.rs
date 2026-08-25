// src-tauri/src/supabase.rs
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const REDIRECT_URI: &str = "http://localhost:54321";

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
}

impl SupabaseClient {
    pub fn new() -> Result<Self, String> {
        dotenvy::dotenv().ok();
        let url = std::env::var("SUPABASE_URL").map_err(|_| "SUPABASE_URL not set")?;
        let key = std::env::var("SUPABASE_KEY")
            .or_else(|_| std::env::var("SUPABASE_PUBLISHABLE_KEY"))
            .map_err(|_| "SUPABASE_KEY not set")?;

        let client = Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert("apikey", key.parse().unwrap());
                headers.insert("Authorization", format!("Bearer {}", key).parse().unwrap());
                headers
            })
            .build()
            .map_err(|e| e.to_string())?;

        let instance = Self {
            url,
            key,
            reqwest: client,
            session: std::sync::Mutex::new(None),
        };

        instance.load_session();
        Ok(instance)
    }

    pub fn session_file_path() -> PathBuf {
        crate::paths::app_data_dir()
            .join("cred")
            .join("supabase_session.json")
    }

    pub fn load_session(&self) {
        if let Ok(data) = std::fs::read_to_string(Self::session_file_path()) {
            if let Ok(session) = serde_json::from_str::<Session>(&data) {
                *self.session.lock().unwrap() = Some(session);
            }
        }
    }

    pub fn save_session(&self) {
        if let Some(session) = &*self.session.lock().unwrap() {
            if let Ok(data) = serde_json::to_string(session) {
                let path = Self::session_file_path();
                std::fs::create_dir_all(path.parent().unwrap()).ok();
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
            .unwrap()
            .as_ref()
            .map(|s| s.access_token.clone())
    }

    pub fn get_user_id(&self) -> Option<String> {
        self.session
            .lock()
            .unwrap()
            .as_ref()
            .and_then(|s| s.user.as_ref().map(|u| u.id.clone()))
    }

    pub fn get_user_email(&self) -> Option<String> {
        self.session
            .lock()
            .unwrap()
            .as_ref()
            .and_then(|s| s.user.as_ref().and_then(|u| u.email.clone()))
    }

    pub fn get_user_display_name(&self) -> Option<String> {
        self.session.lock().unwrap().as_ref().and_then(|s| {
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
        self.session.lock().unwrap().as_ref().and_then(|s| {
            s.user.as_ref().and_then(|u| {
                u.user_metadata.as_ref().and_then(|m| {
                    m.get("avatar_url")
                        .or_else(|| m.get("picture"))
                        .and_then(|v| v.as_str().map(String::from))
                })
            })
        })
    }

    pub async fn login_with_google(&self) -> Result<bool, String> {
        let verifier_bytes: [u8; 32] = rand::random();
        let code_verifier = URL_SAFE_NO_PAD.encode(verifier_bytes);

        let mut hasher = Sha256::new();
        hasher.update(code_verifier.as_bytes());
        let code_challenge = URL_SAFE_NO_PAD.encode(hasher.finalize());

        let auth_url = format!(
            "{}/auth/v1/authorize?provider=google&redirect_to={}&code_challenge={}&code_challenge_method=s256&scopes=https://www.googleapis.com/auth/youtube.upload",
            self.url, REDIRECT_URI, code_challenge
        );

        let listener = TcpListener::bind("127.0.0.1:54321")
            .await
            .map_err(|e| e.to_string())?;

        if let Err(e) = opener::open(&auth_url) {
            return Err(format!("Failed to open browser: {}", e));
        }

        let (mut stream, _) = listener.accept().await.map_err(|e| e.to_string())?;
        let mut buffer = [0; 4096];
        let bytes_read = stream.read(&mut buffer).await.map_err(|e| e.to_string())?;
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);

        let mut auth_code = None;
        if let Some(line) = request.lines().next() {
            if line.starts_with("GET ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 1 {
                    let path = parts[1];
                    if let Ok(url) = url::Url::parse(&format!("http://localhost{}", path)) {
                        for (key, val) in url.query_pairs() {
                            if key == "code" {
                                auth_code = Some(val.into_owned());
                                break;
                            }
                        }
                    }
                }
            }
        }

        let response = if auth_code.is_some() {
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<h1>Login Berhasil!</h1><p>Anda dapat menutup jendela ini.</p>"
        } else {
            "HTTP/1.1 400 Bad Request\r\nContent-Type: text/html\r\n\r\n<h1>Login Gagal</h1><p>Kode otorisasi tidak ditemukan.</p>"
        };
        let _ = stream.write_all(response.as_bytes()).await;

        let code = auth_code.ok_or("Auth code not found in request")?;

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
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            let error = resp.text().await.unwrap_or_default();
            return Err(format!("Failed to exchange token: {}", error));
        }

        let session: Session = resp.json().await.map_err(|e| e.to_string())?;
        *self.session.lock().unwrap() = Some(session);
        self.save_session();

        Ok(true)
    }

    pub async fn logout(&self) -> Result<(), String> {
        let token = self.get_access_token();
        if let Some(t) = token {
            let logout_url = format!("{}/auth/v1/logout", self.url);
            let _ = self
                .reqwest
                .post(&logout_url)
                .header("Authorization", format!("Bearer {}", t))
                .send()
                .await;
        }
        *self.session.lock().unwrap() = None;
        self.save_session();
        Ok(())
    }

    pub async fn sync_config_up(&self, config_dict: serde_json::Value) -> Result<bool, String> {
        let user_id = self.get_user_id().ok_or("Not logged in")?;
        let token = self.get_access_token().ok_or("Not logged in")?;

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
            .map_err(|e| e.to_string())?;

        if resp.status().is_success() {
            Ok(true)
        } else {
            Err(format!("Failed to sync config up: {}", resp.status()))
        }
    }

    pub async fn sync_config_down(&self) -> Result<Option<serde_json::Value>, String> {
        let user_id = self.get_user_id().ok_or("Not logged in")?;
        let token = self.get_access_token().ok_or("Not logged in")?;

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
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("Failed to sync config down: {}", resp.status()));
        }

        let data: Vec<serde_json::Value> = resp.json().await.map_err(|e| e.to_string())?;
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
    ) -> Result<bool, String> {
        let user_id = self.get_user_id().ok_or("Not logged in")?;
        let token = self.get_access_token().ok_or("Not logged in")?;

        if !local_path.exists() {
            return Ok(false);
        }

        let file_data = std::fs::read(local_path).map_err(|e| e.to_string())?;
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
            .map_err(|e| e.to_string())?;

        if resp.status().is_success() {
            Ok(true)
        } else {
            let status = resp.status();
            let error = resp.text().await.unwrap_or_default();
            Err(format!("Upload failed: {} - {}", status, error))
        }
    }

    pub async fn download_file(
        &self,
        remote_filename: &str,
        local_path: &PathBuf,
    ) -> Result<bool, String> {
        let user_id = self.get_user_id().ok_or("Not logged in")?;
        let token = self.get_access_token().ok_or("Not logged in")?;

        let bucket_path = format!("{}/{}", user_id, remote_filename);
        let url = format!("{}/storage/v1/object/user_files/{}", self.url, bucket_path);

        let resp = self
            .reqwest
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if resp.status().is_success() {
            let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
            if bytes.is_empty() {
                return Ok(false);
            }

            if let Some(parent) = local_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            std::fs::write(local_path, bytes).map_err(|e| e.to_string())?;
            Ok(true)
        } else {
            Err(format!("Download failed: {}", resp.status()))
        }
    }
}
