use crate::ai::provider::AIProvider;
use crate::error::CliptzyError;
use crate::orchestrator::pipeline::ProgressTx;
use async_trait::async_trait;
use rig_core::client::CompletionClient;
use rig_core::completion::CompletionModel;
use rig_core::providers::ollama;

pub struct OllamaProvider {
    client: ollama::Client,
    model: String,
}

impl OllamaProvider {
    pub fn new(host: &str, model: &str) -> Self {
        let host = if host.is_empty() {
            "http://localhost:11434"
        } else {
            host
        };

        let client = ollama::Client::builder()
            .base_url(host)
            .api_key("ollama")
            .build()
            .expect("Failed to initialize Ollama client");

        Self {
            client,
            model: model.to_string(),
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
        self.generate_with_tools(prompt, vec![], _progress).await
    }

    async fn generate_with_tools(
        &self,
        prompt: &str,
        tools: Vec<rig_core::completion::ToolDefinition>,
        _progress: Option<&ProgressTx>,
    ) -> Result<String, CliptzyError> {
        let model = self.client.completion_model(&self.model);

        let mut req = model.completion_request(prompt);
        if !tools.is_empty() {
            req = req.tools(tools);
        }
        let request = req.build();

        let response = match model.completion(request).await {
            Ok(res) => res,
            Err(e) => {
                log::warn!("Ollama request with tools failed: {}. Retrying without tools...", e);
                let req_no_tools = self.client.completion_model(&self.model).completion_request(prompt).build();
                model.completion(req_no_tools).await.map_err(|e2| {
                    CliptzyError::AIProvider(format!("Ollama request error (fallback): {}", e2))
                })?
            }
        };

        for content in response.choice {
            match content {
                rig_core::completion::AssistantContent::ToolCall(call) => {
                    return Ok(call.function.arguments.to_string());
                }
                rig_core::completion::AssistantContent::Text(text) => {
                    return Ok(text.text);
                }
                _ => {}
            }
        }

        Err(CliptzyError::AIProvider(
            "Empty response from Ollama".to_string(),
        ))
    }
}
