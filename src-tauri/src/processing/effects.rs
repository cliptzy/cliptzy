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

    pub fn get_effects_for_timeline(
        &self,
        timeline: &crate::analysis::fusion::EmotionTimeline,
    ) -> Vec<ScheduledEffect> {
        let mut scheduled = Vec::new();
        // Keep track of the last effect to avoid spamming too many effects
        let mut last_effect_time = -10.0;

        for seg in &timeline.segments {
            // Only add an effect if it has high confidence and minimum 5s gap
            if seg.score > 0.6 && seg.start_time - last_effect_time > 5.0 {
                let emo_str = format!("{:?}", seg.emotion).to_lowercase();

                if let Some(effect) = self.get_effect(&emo_str) {
                    scheduled.push(ScheduledEffect {
                        effect: effect.clone(),
                        start_time: seg.start_time,
                        end_time: seg.end_time,
                    });
                    last_effect_time = seg.end_time;
                }
            }
        }

        scheduled
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledEffect {
    pub effect: VideoEffect,
    pub start_time: f64,
    pub end_time: f64,
}

impl Default for EffectsManager {
    fn default() -> Self {
        Self::new()
    }
}
