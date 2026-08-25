pub mod provider;
pub mod ollama;
pub mod gemini;
pub mod openai;
pub mod detector;
pub mod metadata;
pub mod prompts;

use crate::config::models::AIConfig;
use provider::AIProvider;
use ollama::OllamaProvider;
use gemini::GeminiProvider;
use openai::OpenAIProvider;

pub fn create_provider(config: &AIConfig) -> Box<dyn AIProvider> {
    match config.provider.as_str() {
        "ollama" => Box::new(OllamaProvider::new(&config.ollama_host, &config.ollama_model)),
        "gemini" => Box::new(GeminiProvider::new(&config.gemini_key, &config.gemini_model)),
        "openai" => Box::new(OpenAIProvider::new(
            &config.openai_key,
            &config.openai_model,
            &config.openai_base_url,
        )),
        _ => Box::new(GeminiProvider::new(&config.gemini_key, &config.gemini_model)),
    }
}
