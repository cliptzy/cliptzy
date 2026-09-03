pub mod detector;
pub mod gemini;
pub mod metadata;
pub mod models;
pub mod ollama;
pub mod openai;
pub mod prompts;
pub mod provider;
pub mod tools;

use crate::config::models::AIConfig;
use gemini::GeminiProvider;
use ollama::OllamaProvider;
use openai::OpenAIProvider;
use provider::AIProvider;

pub fn create_provider(config: &AIConfig) -> Box<dyn AIProvider> {
    match config.provider.as_str() {
        "ollama" => Box::new(OllamaProvider::new(
            &config.ollama_host,
            &config.ollama_model,
        )),
        "gemini" => Box::new(GeminiProvider::new(
            &config.gemini_key,
            &config.gemini_model,
        )),
        "openai" => Box::new(OpenAIProvider::new(
            &config.openai_key,
            &config.openai_model,
            &config.openai_base_url,
        )),
        _ => Box::new(GeminiProvider::new(
            &config.gemini_key,
            &config.gemini_model,
        )),
    }
}
pub mod onnx;
