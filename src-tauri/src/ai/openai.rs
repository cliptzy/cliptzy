use crate::ai::provider::AIProvider;
use crate::error::CliptzyError;
use crate::orchestrator::pipeline::ProgressTx;
use async_trait::async_trait;
use rig_core::client::CompletionClient;
use rig_core::completion::CompletionModel;
use rig_core::providers::openai;

pub struct OpenAIProvider {
    client: openai::CompletionsClient,
    model: String,
}

impl OpenAIProvider {
    pub fn new(api_key: &str, model: &str, base_url: &str) -> Self {
        let model = if model.is_empty() {
            "gpt-4o-mini"
        } else {
            model
        };

        let builder = openai::Client::builder().api_key(api_key);
        let builder = if !base_url.is_empty() {
            let mut base = base_url.trim_end_matches('/').to_string();
            if !base.ends_with("/v1") {
                base.push_str("/v1");
            }
            builder.base_url(&base)
        } else {
            builder
        };

        let client = builder.build().expect("Failed to initialize OpenAI client");
        let client = client.completions_api();

        Self {
            client,
            model: model.to_string(),
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
        self.generate_with_tools(prompt, vec![], _progress).await
    }

    async fn generate_with_tools(
        &self,
        prompt: &str,
        tools: Vec<rig_core::completion::ToolDefinition>,
        _progress: Option<&ProgressTx>,
    ) -> Result<String, CliptzyError> {
        let model = self.client.completion_model(&self.model);

        let mut req = model.completion_request(prompt)
            .additional_params(serde_json::json!({ "stream": false }));
        
        if !tools.is_empty() {
            req = req.tools(tools);
        }
        let request = req.build();

        let response = match model.completion(request).await {
            Ok(res) => res,
            Err(e) => {
                log::warn!("OpenAI request with tools failed: {}. Retrying without tools...", e);
                let req_no_tools = self.client.completion_model(&self.model)
                    .completion_request(prompt)
                    .additional_params(serde_json::json!({ "stream": false }))
                    .build();
                model.completion(req_no_tools).await.map_err(|e2| {
                    CliptzyError::AIProvider(format!("OpenAI request error (fallback): {}", e2))
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
            "Empty response from OpenAI".to_string(),
        ))
    }
}
