use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoEffect {
    pub name: String,
    pub file: String,
    #[serde(rename = "type")]
    pub effect_type: String,
    pub key_color: String,
    pub audio_filter: String,
    pub emotions: Vec<String>,
}

pub static EFFECTS_CATALOG_JSON: &str = include_str!("../../../assets/video_effects.json");

pub struct EffectsManager {
    effects: Vec<VideoEffect>,
}

impl EffectsManager {
    pub fn new() -> Self {
        let effects: Vec<VideoEffect> =
            serde_json::from_str(EFFECTS_CATALOG_JSON).unwrap_or_default();
        Self { effects }
    }

    pub fn get_effect(&self, emotion: &str) -> Option<&VideoEffect> {
        let matching: Vec<&VideoEffect> = self
            .effects
            .iter()
            .filter(|e| e.emotions.iter().any(|emo| emo == emotion))
            .collect();

        let mut rng = rand::rng();
        matching.choose(&mut rng).copied()
    }
}

impl Default for EffectsManager {
    fn default() -> Self {
        Self::new()
    }
}
