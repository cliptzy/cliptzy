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
        match self.name.as_str() {
            "sfx_huh" => 0.44,
            "sfx_uhh" => 0.49,
            "sfx_kerja bagus" => 0.79,
            "sfx_vineboom" => 0.95,
            "sfx_cashregister" => 1.18,
            "sfx_rizz" => 1.37,
            "ainsley harriott_hehe buoi" => 1.49,
            "man_dayum daniel akakakakakakak" => 1.73,
            "sfx_spongebob disappoint" => 1.74,
            "michael rosen_niceee" => 1.95,
            "shaq_spicy" => 1.95,
            "anthony fantano_dayum boi he thic" => 2.02,
            "cat_laugh" => 2.02,
            "gta_mission success" => 2.02,
            "man_shocked" => 2.02,
            "overlay_big brain time" => 2.02,
            "overlay_internet error" => 2.44,
            "overlay_ooohhhhhhhh" => 2.02,
            "overlay_windows error" => 2.02,
            "the rock_sus" => 2.02,
            "ksi_nononono" => 2.25,
            "cat_slamming table" => 2.32,
            "iDubbz_hey thats pretty good" => 2.44,
            "ishowspeed_scream" => 2.44,
            "ksi_yesyesyesyes" => 2.44,
            "gta_ah shit here we go again" => 2.53,
            "jhon travolta_confused" => 3.40,
            "caster_excited" => 3.47,
            "cat_firing ak47" => 3.47,
            "tyler1_scream" => 3.53,
            "flightreact_woah woah hey hey" => 3.83,
            "penguiz0_wooo yeah babyyy" => 3.83,
            "tyler1_woooo woooo" => 4.00,
            "man_crying" => 4.03,
            "grandpa_what oh hel no" => 4.40,
            "dog_scared" => 4.88,
            "patrick bateman_sigma" => 5.07,
            "risitas_laugh" => 5.97,
            _ => 2.50,
        }
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
