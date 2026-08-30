use crate::ai::create_provider;
use crate::ai::prompts::METADATA_PROMPT_TEMPLATE;
use crate::config::models::AIConfig;
use crate::error::CliptzyError;
use crate::orchestrator::pipeline::ProgressTx;
use regex::Regex;
use serde_json::{json, Value};

pub struct MetadataGenerator;

impl MetadataGenerator {
    pub async fn generate_metadata(
        &self,
        clip_text: &str,
        youtube_title: &str,
        channel_name: &str,
        youtube_url: &str,
        config: &AIConfig,
        progress: Option<&ProgressTx>,
        language: &str,
        words_data: Option<&Vec<Value>>,
    ) -> Result<Value, CliptzyError> {
        let mut text = clip_text.trim().to_string();
        if text.len() > 10000 {
            text.truncate(10000);
            text.push_str("...");
        }

        let is_local = config.provider.to_lowercase() == "ollama";
        let chunk_size = if is_local { 150 } else { 1000 };

        let words_chunks = if let Some(words) = words_data {
            words
                .chunks(chunk_size)
                .map(|c| c.to_vec())
                .collect::<Vec<_>>()
        } else {
            vec![vec![]]
        };

        let provider = create_provider(config);
        let mut global_metadata = json!({});
        let mut all_enriched = Vec::new();
        let mut all_standalone = Vec::new();

        for (idx, chunk) in words_chunks.iter().enumerate() {
            let chunk_info = if words_chunks.len() > 1 {
                format!(
                    "\n(IMPORTANT: This is part {} of {} of the total words...)\n",
                    idx + 1,
                    words_chunks.len()
                )
            } else {
                "".to_string()
            };

            let words_json = if chunk.is_empty() {
                "None.".to_string()
            } else {
                serde_json::to_string(chunk).unwrap_or_else(|_| "None.".to_string())
            };

            let prompt = METADATA_PROMPT_TEMPLATE
                .replace("{language}", language)
                .replace("{channel_name}", channel_name)
                .replace("{youtube_title}", youtube_title)
                .replace("{youtube_url}", youtube_url)
                .replace("{context_str}", "")
                .replace("{visual_str}", "")
                .replace("{audio_str}", "")
                .replace("{chunk_info}", &chunk_info)
                .replace(
                    "{emotion_str}",
                    "neutral, happy, angry, shock, fear, sad, confused",
                )
                .replace(
                    "{effects_str}",
                    "none, random, vineboom, tyler1_scream, bruh",
                )
                .replace("{clip_text}", &text)
                .replace("{local_tz}", "UTC")
                .replace("{part}", &(idx + 1).to_string())
                .replace("{total}", &words_chunks.len().to_string())
                .replace("{words_data}", &words_json);

            let raw_response = provider.generate(&prompt, progress).await?;

            let re = Regex::new(r#"(?s)\{\s*".*"\s*:.*\s*\}"#).unwrap();
            let json_str = if let Some(mat) = re.find(&raw_response) {
                mat.as_str()
            } else {
                raw_response.trim()
            };

            if let Ok(metadata) = serde_json::from_str::<Value>(json_str) {
                if idx == 0 {
                    global_metadata["title"] = metadata.get("title").cloned().unwrap_or(json!(""));
                    global_metadata["tags"] = metadata.get("tags").cloned().unwrap_or(json!(""));
                    global_metadata["highlight"] =
                        metadata.get("highlight").cloned().unwrap_or(json!(""));
                    global_metadata["recommended_publish_time"] = metadata
                        .get("recommended_publish_time")
                        .cloned()
                        .unwrap_or(json!(""));
                }
                if let Some(arr) = metadata
                    .get("enriched_transcript")
                    .and_then(|v| v.as_array())
                {
                    all_enriched.extend(arr.clone());
                }
                if let Some(arr) = metadata
                    .get("standalone_video_effects")
                    .and_then(|v| v.as_array())
                {
                    all_standalone.extend(arr.clone());
                }
            }

            if idx < words_chunks.len() - 1 && !is_local {
                if let Some(p) = progress {
                    let _ = p.send(crate::orchestrator::pipeline::ProgressEvent {
                        stage: "ai".to_string(),
                        label: format!("Menunggu 30 detik (Anti-RateLimit Gemini) sebelum memproses part {}...", idx + 2),
                        current: 0,
                        total: 100,
                        detail: None
                    });
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
            }
        }

        global_metadata["enriched_transcript"] = json!(all_enriched);
        global_metadata["standalone_video_effects"] = json!(all_standalone);

        Ok(global_metadata)
    }
}
