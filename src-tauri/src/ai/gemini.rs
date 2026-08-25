use crate::ai::provider::AIProvider;
use crate::error::CliptzyError;
use crate::orchestrator::pipeline::ProgressTx;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

pub struct GeminiProvider {
    api_key: String,
    model: String,
    client: Client,
}

impl GeminiProvider {
    pub fn new(api_key: &str, model: &str) -> Self {
        let model = if model.is_empty() { "gemini-1.5-flash" } else { model };
        Self {
            api_key: api_key.to_string(),
            model: model.to_string(),
            client: Client::new(),
        }
    }
}

#[async_trait]
impl AIProvider for GeminiProvider {
    fn name(&self) -> &str {
        "gemini"
    }

    async fn generate(
        &self,
        prompt: &str,
        _progress: Option<&ProgressTx>,
    ) -> Result<String, CliptzyError> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:streamGenerateContent?alt=sse&key={}",
            self.model, self.api_key
        );
        
        let body = json!({
            "contents": [{"parts": [{"text": prompt}]}],
            "generationConfig": {
                "temperature": 0.2,
                "responseMimeType": "application/json"
            }
        });

        let mut res = self.client.post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| CliptzyError::AIProvider(format!("Gemini request error: {}", e)))?;

        if !res.status().is_success() {
            let status = res.status();
            let err_text = res.text().await.unwrap_or_default();
            return Err(CliptzyError::AIProvider(format!("Gemini error ({}): {}", status, err_text)));
        }

        let mut full_response = String::new();
        while let Some(chunk) = res.chunk().await.map_err(|e| CliptzyError::AIProvider(e.to_string()))? {
            let chunk_str = String::from_utf8_lossy(&chunk);
            for line in chunk_str.lines() {
                let line = line.trim();
                if line.starts_with("data: ") {
                    let json_str = &line[6..];
                    if let Ok(data) = serde_json::from_str::<serde_json::Value>(json_str) {
                        if let Some(candidates) = data.get("candidates").and_then(|c| c.as_array()) {
                            if let Some(first) = candidates.first() {
                                if let Some(parts) = first.get("content").and_then(|c| c.get("parts")).and_then(|p| p.as_array()) {
                                    if let Some(part) = parts.first() {
                                        if let Some(text) = part.get("text").and_then(|t| t.as_str()) {
                                            full_response.push_str(text);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(full_response)
    }
}
