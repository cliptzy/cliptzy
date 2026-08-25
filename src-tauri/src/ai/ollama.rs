use crate::ai::provider::AIProvider;
use crate::error::CliptzyError;
use crate::orchestrator::pipeline::ProgressTx;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
pub struct OllamaProvider {
    host: String,
    model: String,
    client: Client,
}

impl OllamaProvider {
    pub fn new(host: &str, model: &str) -> Self {
        let host = if host.is_empty() { "http://localhost:11434" } else { host };
        Self {
            host: host.trim_end_matches('/').to_string(),
            model: model.to_string(),
            client: Client::new(),
        }
    }
}

#[async_trait]
impl AIProvider for OllamaProvider {
    fn name(&self) -> &str {
        "ollama"
    }

    async fn generate(
        &self,
        prompt: &str,
        _progress: Option<&ProgressTx>,
    ) -> Result<String, CliptzyError> {
        let url = format!("{}/api/generate", self.host);
        
        let body = json!({
            "model": self.model,
            "prompt": prompt,
            "stream": true,
            "options": {
                "temperature": 0.3,
                "num_predict": 8192,
                "num_ctx": 16384
            }
        });

        let mut res = self.client.post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| CliptzyError::AIProvider(format!("Ollama request error: {}", e)))?;

        if !res.status().is_success() {
            let status = res.status();
            let err_text = res.text().await.unwrap_or_default();
            return Err(CliptzyError::AIProvider(format!("Ollama error ({}): {}", status, err_text)));
        }

        let mut full_response = String::new();
        while let Some(chunk) = res.chunk().await.map_err(|e| CliptzyError::AIProvider(e.to_string()))? {
            if let Ok(data) = serde_json::from_slice::<serde_json::Value>(&chunk) {
                if let Some(resp) = data.get("response").and_then(|r| r.as_str()) {
                    full_response.push_str(resp);
                }
            }
        }

        Ok(full_response)
    }
}
