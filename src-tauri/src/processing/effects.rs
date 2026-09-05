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

impl VideoEffect {
    pub fn resolve_path(&self) -> std::path::PathBuf {
        let in_video_effects = crate::paths::app_data_dir()
            .join("assets")
            .join("video_effects")
            .join(&self.file);
        if in_video_effects.exists() {
            in_video_effects
        } else {
            crate::paths::app_data_dir()
                .join("assets")
                .join(&self.file)
        }
    }

    /// Mengembalikan durasi asli file video meme dalam detik untuk sinkronisasi OSD & overlay
    pub fn get_duration(&self) -> f64 {
        let path = self.resolve_path();
        if path.exists() {
            let output = std::process::Command::new("ffprobe")
                .args([
                    "-v",
                    "error",
                    "-show_entries",
                    "format=duration",
                    "-of",
                    "default=noprint_wrappers=1:nokey=1",
                ])
                .arg(&path)
                .output();

            if let Ok(out) = output {
                if out.status.success() {
                    if let Ok(duration_str) = String::from_utf8(out.stdout) {
                        if let Ok(duration) = duration_str.trim().parse::<f64>() {
                            return duration;
                        }
                    }
                }
            }
        }
        
        // Fallback default duration
        2.50
    }
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
            .filter(|e| e.emotions.iter().any(|emo| emo == emotion) && e.resolve_path().exists())
            .collect();

        let mut rng = rand::rng();
        matching.choose(&mut rng).copied()
    }

    pub fn get_effect_by_name(&self, name: &str) -> Option<&VideoEffect> {
        self.effects.iter().find(|e| e.name == name)
    }

    pub fn all_effects(&self) -> &[VideoEffect] {
        &self.effects
    }

    pub fn get_effects_for_timeline(
        &self,
        timeline: &crate::analysis::fusion::EmotionTimeline,
    ) -> Vec<ScheduledEffect> {
        let mut scheduled: Vec<ScheduledEffect> = Vec::new();

        for seg in &timeline.segments {
            // Hanya jadwalkan efek untuk emosi non-neutral dengan score memadai
            if seg.emotion != crate::analysis::EmotionLabel::Neutral && seg.score >= 0.60 {
                let start_t = seg.start_time;

                // Penumpukan VFX: Izinkan hingga maksimal 2 efek bersamaan
                let active_count = scheduled
                    .iter()
                    .filter(|e| e.start_time <= start_t && e.end_time > start_t)
                    .count();

                if active_count < 2 {
                    // Jeda awal antar efek minimal 1.0s agar tidak bertabrakan persis di frame yang sama
                    let too_close_start = scheduled
                        .iter()
                        .any(|e| (e.start_time - start_t).abs() < 1.0);

                    if !too_close_start {
                        let emo_str = format!("{:?}", seg.emotion).to_lowercase();
                        if let Some(effect) = self.get_effect(&emo_str) {
                            // Hindari efek identik dalam jarak 4 detik
                            let same_recently = scheduled.iter().any(|e| {
                                e.effect.name == effect.name && (e.start_time - start_t).abs() < 4.0
                            });

                            if !same_recently {
                                let end_t = start_t + effect.get_duration();
                                scheduled.push(ScheduledEffect {
                                    effect: effect.clone(),
                                    start_time: start_t,
                                    end_time: end_t,
                                });
                            }
                        }
                    }
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
