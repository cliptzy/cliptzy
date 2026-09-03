use crate::error::CliptzyError;
use reqwest::Client;

#[tauri::command]
pub async fn fetch_openai_models(
    base_url: String,
    api_key: String,
) -> Result<Vec<String>, CliptzyError> {
    log::info!("Fetching models from: {}", base_url);

    let client = Client::new();
    let base = base_url.trim_end_matches('/');
    let url = if base.ends_with("/models") {
        base.to_string()
    } else if base.ends_with("/v1") {
        format!("{}/models", base)
    } else {
        format!("{}/v1/models", base)
    };

    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("HTTP-Referer", "https://cliptzy.local")
        .header("X-Title", "Cliptzy")
        .send()
        .await
        .map_err(|e| CliptzyError::AIProvider(format!("HTTP request failed: {}", e)))?;

    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await.unwrap_or_default();
        return Err(CliptzyError::AIProvider(format!(
            "API returned status {}: {}",
            status, text
        )));
    }

    let body: serde_json::Value = res
        .json()
        .await
        .map_err(|e| CliptzyError::AIProvider(format!("Failed to parse response: {}", e)))?;

    let mut models: Vec<String> = Vec::new();

    if let Some(data) = body.get("data").and_then(|d| d.as_array()) {
        for model in data {
            if let Some(id) = model.get("id").and_then(|i| i.as_str()) {
                models.push(id.to_string());
            }
        }
    }

    // Deduplicate and sort
    models.sort();
    models.dedup();

    Ok(models)
}

#[tauri::command]
pub async fn ask_agent(prompt: String) -> Result<String, CliptzyError> {
    log::info!("Agent processing prompt: {}", prompt);
    let config = crate::config::models::AppConfig::load().unwrap_or_default();
    let ai_provider = crate::ai::create_provider(&config.ai);

    // Uji jalannya Tool AnalyzeTranscriptTool
    let tools = vec![crate::ai::tools::analyze::analyze_transcript_tool()];

    match ai_provider.generate_with_tools(&prompt, tools, None).await {
        Ok(result) => Ok(result),
        Err(e) => Err(CliptzyError::AIProvider(format!("Agent error: {}", e))),
    }
}
