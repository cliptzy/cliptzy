use crate::ai::provider::AIProvider;
use crate::error::CliptzyError;
use crate::orchestrator::pipeline::ProgressTx;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

pub struct OpenAIProvider {
    api_key: String,
    model: String,
    base_url: String,
    client: Client,
}

impl OpenAIProvider {
    pub fn new(api_key: &str, model: &str, base_url: &str) -> Self {
        let model = if model.is_empty() { "gpt-4o-mini" } else { model };
        let base_url = if base_url.is_empty() { "https://api.openai.com/v1" } else { base_url };
        Self {
            api_key: api_key.to_string(),
            model: model.to_string(),
            base_url: base_url.trim_end_matches('/').to_string(),
            client: Client::new(),
        }
    }
}

#[async_trait]
impl AIProvider for OpenAIProvider {
    fn name(&self) -> &str {
        "openai"
    }

    async fn generate(
        &self,
        prompt: &str,
        _progress: Option<&ProgressTx>,
    ) -> Result<String, CliptzyError> {
        let url = format!("{}/chat/completions", self.base_url);
        
        let body = json!({
            "model": self.model,
            "messages": [
                {
                    "role": "system",
                    "content": "You are a professional video editor and JSON highlight generator."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "response_format": { "type": "json_object" },
            "temperature": 0.3,
            "stream": true
        });

        let mut res = self.client.post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| CliptzyError::AIProvider(format!("OpenAI request error: {}", e)))?;

        if !res.status().is_success() {
            let status = res.status();
            let err_text = res.text().await.unwrap_or_default();
            return Err(CliptzyError::AIProvider(format!("OpenAI error ({}): {}", status, err_text)));
        }

        let mut full_response = String::new();
        while let Some(chunk) = res.chunk().await.map_err(|e| CliptzyError::AIProvider(e.to_string()))? {
            let chunk_str = String::from_utf8_lossy(&chunk);
            for line in chunk_str.lines() {
                let line = line.trim();
                if line.starts_with("data: ") && line != "data: [DONE]" {
                    let json_str = &line[6..];
                    if let Ok(data) = serde_json::from_str::<serde_json::Value>(json_str) {
                        if let Some(choices) = data.get("choices").and_then(|c| c.as_array()) {
                            if let Some(first) = choices.first() {
                                if let Some(content) = first.get("delta").and_then(|d| d.get("content")).and_then(|c| c.as_str()) {
                                    full_response.push_str(content);
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
