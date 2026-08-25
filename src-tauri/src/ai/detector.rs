use crate::config::models::AIConfig;
use crate::error::CliptzyError;
use crate::ai::create_provider;
use crate::ai::prompts::DEFAULT_PROMPT_TEMPLATE;
use crate::orchestrator::pipeline::ProgressTx;
use serde_json::{Value, json};
use regex::Regex;
use std::collections::HashMap;

pub struct AIHighlightDetector;

impl AIHighlightDetector {
    pub fn new() -> Self {
        Self
    }

    fn is_local_provider(&self, config: &AIConfig) -> bool {
        let provider = config.provider.to_lowercase();
        if provider == "ollama" {
            return true;
        }
        if provider == "openai" {
            let base_url = config.openai_base_url.to_lowercase();
            if base_url.contains("localhost") || base_url.contains("127.0.0.1") || base_url.contains("lmstudio") {
                return true;
            }
        }
        false
    }

    pub async fn detect_highlights(
        &self,
        transcript_segments: &[HashMap<String, Value>],
        config: &AIConfig,
        progress: Option<&ProgressTx>,
        language: &str,
    ) -> Result<Vec<Value>, CliptzyError> {
        if transcript_segments.is_empty() {
            return Ok(vec![]);
        }

        let mut formatted_lines = Vec::new();
        for seg in transcript_segments {
            let start = seg.get("start").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let end = seg.get("end").and_then(|v| v.as_f64()).unwrap_or(start + 2.0);
            let text = seg.get("text").and_then(|v| v.as_str()).unwrap_or("").trim();
            if !text.is_empty() {
                formatted_lines.push(format!("[{:.1}s - {:.1}s]: {}", start, end, text));
            }
        }

        let is_local = self.is_local_provider(config);
        let max_chunk_chars = if is_local { 12000 } else { 250000 };

        let mut chunks = Vec::new();
        let mut current_chunk = Vec::new();
        let mut current_len = 0;

        for line in formatted_lines {
            let line_len = line.len() + 1;
            if current_len + line_len > max_chunk_chars && !current_chunk.is_empty() {
                chunks.push(current_chunk.join("\n"));
                current_chunk.clear();
                current_chunk.push(line.clone());
                current_len = line_len;
            } else {
                current_chunk.push(line);
                current_len += line_len;
            }
        }
        if !current_chunk.is_empty() {
            chunks.push(current_chunk.join("\n"));
        }

        let provider = create_provider(config);
        let mut all_highlights = Vec::new();

        for chunk_text in chunks {
            let prompt = DEFAULT_PROMPT_TEMPLATE
                .replace("{language}", language)
                .replace("{custom_context}", "")
                .replace("{transcript_text}", &chunk_text);

            let raw_response = provider.generate(&prompt, progress).await?;
            let mut highlights = self.parse_json_highlights(&raw_response);
            all_highlights.append(&mut highlights);
        }

        all_highlights.sort_by(|a, b| {
            let start_a = a.get("start").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let start_b = b.get("start").and_then(|v| v.as_f64()).unwrap_or(0.0);
            start_a.partial_cmp(&start_b).unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(all_highlights)
    }

    fn parse_json_highlights(&self, raw_text: &str) -> Vec<Value> {
        let mut json_str = raw_text.trim().to_string();

        let re_arr = Regex::new(r"(?s)\[\s*\{.*\}\s*\]").unwrap();
        let re_obj = Regex::new(r#"(?s)\{\s*".*"\s*:.*\s*\}"#).unwrap();

        if let Some(mat) = re_arr.find(raw_text) {
            json_str = mat.as_str().to_string();
        } else if let Some(mat) = re_obj.find(raw_text) {
            json_str = mat.as_str().to_string();
        }

        let parsed: Result<Value, _> = serde_json::from_str(&json_str);
        if let Ok(val) = parsed {
            let items = if val.is_object() {
                val.get("segments")
                    .or(val.get("highlights"))
                    .or(val.get("clips"))
                    .or(val.as_object().unwrap().values().next())
                    .cloned()
            } else {
                Some(val)
            };

            if let Some(Value::Array(arr)) = items {
                let mut clean_highlights = Vec::new();
                for item in arr {
                    if let (Some(start), Some(dur)) = (
                        item.get("start").and_then(|v| v.as_f64()),
                        item.get("duration").and_then(|v| v.as_f64())
                    ) {
                        let title = item.get("title").and_then(|v| v.as_str()).unwrap_or("Momen Menarik AI");
                        let reason = item.get("reason").and_then(|v| v.as_str()).unwrap_or("Dideteksi oleh AI model");
                        let score = item.get("score").and_then(|v| v.as_f64()).unwrap_or(0.9);
                        
                        clean_highlights.push(json!({
                            "start": start,
                            "duration": dur,
                            "title": title,
                            "reason": reason,
                            "score": score,
                        }));
                    }
                }
                return clean_highlights;
            }
        }
        vec![]
    }
}
