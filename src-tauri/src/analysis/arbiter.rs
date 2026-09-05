use super::{AnalysisSegment, BoundingBox, EmotionLabel};
use crate::ai::create_provider;
use crate::config::models::AIConfig;
use crate::orchestrator::pipeline::ProgressEvent;
use crate::processing::burner::builtin::{BuiltinEffectType, ScheduledBuiltinEffect};
use crate::processing::effects::{EffectsManager, ScheduledEffect};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;
use std::time::Duration;
use tokio::sync::broadcast::Sender;

static RE_JSON_FENCE: OnceLock<Regex> = OnceLock::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptSnippet {
    pub start: f64,
    pub end: f64,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct MultimodalBucket {
    pub start_time: f64,
    pub end_time: f64,
    pub spoken_text: String,
    pub visual_emotion: Option<EmotionLabel>,
    pub visual_score: f32,
    pub visual_bbox: Option<BoundingBox>,
    pub voice_emotion: Option<EmotionLabel>,
    pub voice_score: f32,
    pub audio_emotion: Option<EmotionLabel>,
    pub audio_score: f32,
    pub text_emotion: Option<EmotionLabel>,
    pub text_score: f32,
}

#[derive(Debug, Deserialize)]
struct AiArbiterResponse {
    #[serde(default)]
    segments: Vec<AiSegmentResponse>,
    #[serde(default)]
    dominant_emotion: Option<String>,
    #[serde(default)]
    vfx_schedule: Vec<AiVfxScheduleResponse>,
    #[serde(default)]
    builtin_schedule: Vec<AiBuiltinScheduleResponse>,
}

#[derive(Debug, Deserialize)]
struct AiSegmentResponse {
    start: f64,
    end: f64,
    emotion: String,
    score: f32,
    #[serde(default)]
    #[allow(dead_code)]
    reasoning: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AiVfxScheduleResponse {
    start_time: f64,
    effect_name: String,
    #[serde(default)]
    #[allow(dead_code)]
    reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AiBuiltinScheduleResponse {
    start_time: f64,
    effect_type: String,
    #[serde(default)]
    #[allow(dead_code)]
    duration: Option<f64>,
}

/// Helper untuk memuat transkrip Whisper dari file cache jika tersedia
pub fn load_transcript_snippets(transcript_path: &Path) -> Vec<TranscriptSnippet> {
    if !transcript_path.exists() {
        return vec![];
    }

    let Ok(content) = std::fs::read_to_string(transcript_path) else {
        return vec![];
    };

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct RawWord {
        word: String,
        start: f64,
        end: f64,
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct RawSegment {
        start: f64,
        end: f64,
        text: String,
        #[serde(default)]
        words: Vec<RawWord>,
    }

    #[derive(Deserialize)]
    struct RawWrapper {
        #[serde(default)]
        segments: Vec<RawSegment>,
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum RawData {
        Wrapped(RawWrapper),
        Direct(Vec<RawSegment>),
    }

    let segments = match serde_json::from_str::<RawData>(&content) {
        Ok(RawData::Wrapped(w)) => w.segments,
        Ok(RawData::Direct(d)) => d,
        Err(_) => return vec![],
    };

    let mut snippets = Vec::new();
    for seg in segments {
        let trimmed = seg.text.trim().to_string();
        if !trimmed.is_empty() {
            snippets.push(TranscriptSnippet {
                start: seg.start,
                end: seg.end,
                text: trimmed,
            });
        }
    }

    snippets
}

/// Deskripsi semantik humor, reaksi emosional, dan konteks meme dari video effects
pub fn get_effect_semantic_description(name: &str) -> &'static str {
    match name {
        "cat_slamming table" => "Rage / keyboard smash / slamming table violently in pure gamer anger (Angry)",
        "tyler1_scream" => "Loud unhinged rage scream at the monitor (Angry)",
        "cat_firing ak47" => "Aggressive shooting rampage / revenge spree (Angry)",
        "grandpa_what oh hel no" => "Disgusted rejection / 'what oh hell no' (Angry/Disgusted)",
        "gta_ah shit here we go again" => "Surrender / repeated failure / giving up in defeat / 'ah shit here we go again' (Sad/Fear/Surrender)",
        "sfx_spongebob disappoint" => "Disappointment / fail / sad trombone womp womp (Sad/Fail)",
        "man_crying" => "Crying uncontrollably in despair / tragic defeat (Sad)",
        "overlay_internet error" => "Lag / disconnect / connection lost / fail moment (Confused/Sad)",
        "the rock_sus" => "The Rock Dwayne Johnson eyebrow raise vine boom, suspicious / questioning / sus moment (Confused/Suspicious)",
        "jhon travolta_confused" => "Looking around completely lost / clueless / empty room (Confused)",
        "sfx_huh" => "Dog turning head 'huh?' confusion sound (Confused)",
        "sfx_uhh" => "Minecraft/Roblox 'uhh' damage / bewildered sound (Confused)",
        "overlay_big brain time" => "Overthinking / galaxy brain logic / weird calculation (Confused)",
        "overlay_windows error" => "Windows error popup sound / brain crash / freeze (Confused)",
        "ishowspeed_scream" => "Terrified loud scream / jump-scare / extreme panic (Fear/Panic)",
        "flightreact_woah woah hey hey" => "Disbelief / backing away in shock 'woah woah hey hey' (Shock/Disbelief)",
        "dog_scared" => "Trembling scared dog in corner (Fear)",
        "ksi_nononono" => "Pleading in distress 'no no no no' (Fear/Pleading)",
        "sfx_vineboom" => "Loud dramatic shock / sudden plot twist / awkward pause / vine boom (Shock/Suspicious)",
        "man_shocked" => "Jaw dropped in utter disbelief / wide open mouth (Shock)",
        "overlay_ooohhhhhhhh" => "Crowd savage roast 'ooooohhhhh' (Shock/Roast)",
        "ainsley harriott_hehe buoi" => "Smug sneaky grin / mischievous plot / 'hehe buoi' (Happy/Smug)",
        "anthony fantano_dayum boi he thic" => "Hyped funny reaction / 'dayum boi he thic' (Shock/Happy)",
        "caster_excited" => "Esports shoutcaster hype scream (Happy/Excited)",
        "cat_laugh" => "Laughing cat pointing at screen (Happy/Laugh)",
        "gta_mission success" => "Mission passed victory fanfare (Happy/Success)",
        "iDubbz_hey thats pretty good" => "Satisfied approval / 'hey that's pretty good' (Happy/Approved)",
        "ksi_yesyesyesyes" => "Celebration hype 'yes yes yes yes' (Happy/Celebration)",
        "man_dayum daniel akakakakakakak" => "Hysterical wheezing laughter (Happy/Laugh)",
        "michael rosen_niceee" => "Cheek pop *click* 'nice' / smooth play (Happy)",
        "patrick bateman_sigma" => "Sigma smirk and confident head tilt (Happy/Sigma)",
        "penguiz0_wooo yeah babyyy" => "MoistCr1tikal jumping 'woo yeah baby that's what I was waiting for' (Happy/Victory)",
        "risitas_laugh" => "El Risitas contagious wheezing laughter (Happy/Laugh)",
        "sfx_cashregister" => "Cha-ching money profit sound (Happy/Money)",
        "sfx_kerja bagus" => "Indonesian wholesome praise 'kerja bagus' (Happy/Wholesome)",
        "sfx_rizz" => "Smooth guitar rizz sound effect (Happy/Flirt)",
        "shaq_spicy" => "Shaq hot wing reaction tongue wiggle (Happy/Shock)",
        "tyler1_woooo woooo" => "Tyler1 high-pitched panic howling (Fear/Panic)",
        _ => "General reaction meme (Neutral)",
    }
}

pub struct ContextArbiter {
    effects_mgr: EffectsManager,
}

impl ContextArbiter {
    pub fn new() -> Self {
        Self {
            effects_mgr: EffectsManager::new(),
        }
    }

    /// Melakukan arbitrase multimodal dan penentuan efek visual (VFX)
    pub async fn arbitrate(
        &self,
        visual: &[AnalysisSegment],
        audio: &[AnalysisSegment],
        voice: &[AnalysisSegment],
        text: &[AnalysisSegment],
        transcripts: &[TranscriptSnippet],
        config: &AIConfig,
        progress: &Sender<ProgressEvent>,
    ) -> (
        Vec<AnalysisSegment>,
        EmotionLabel,
        HashMap<String, f32>,
        Vec<ScheduledEffect>,
        Vec<ScheduledBuiltinEffect>,
    ) {
        let buckets = self.build_buckets(visual, audio, voice, text, transcripts);
        if buckets.is_empty() {
            return (
                vec![],
                EmotionLabel::Neutral,
                HashMap::new(),
                vec![],
                vec![],
            );
        }

        // Cek apakah ada provider AI (OpenAI/Gemini/Ollama) yang terkonfigurasi
        let is_configured = match config.provider.to_lowercase().as_str() {
            "openai" => !config.openai_key.trim().is_empty(),
            "gemini" => !config.gemini_key.trim().is_empty(),
            "ollama" => !config.ollama_host.trim().is_empty(),
            _ => false,
        };

        if is_configured {
            let _ = progress.send(ProgressEvent {
                stage: "fusion".into(),
                label: "AI Context Arbiter: Menentukan emosi kontekstual & VFX...".into(),
                current: 85,
                total: 100,
                detail: Some(format!("Provider: {}", config.provider)),
            });

            // Beri timeout 20 detik untuk menghindari blocking pipeline
            match tokio::time::timeout(
                Duration::from_secs(20),
                self.call_ai_arbiter(&buckets, config, progress),
            )
            .await
            {
                Ok(Ok(result)) => {
                    log::info!(
                        "AI Context Arbiter sukses: {} segmen, dominant: {:?}, {} vfx, {} builtin fx dijadwalkan",
                        result.0.len(),
                        result.1,
                        result.3.len(),
                        result.4.len()
                    );
                    return result;
                }
                Ok(Err(e)) => {
                    log::warn!(
                        "AI Context Arbiter gagal ({}), beralih ke Smart Contextual Heuristics Arbiter.",
                        e
                    );
                }
                Err(_) => {
                    log::warn!(
                        "AI Context Arbiter timeout (>20s), beralih ke Smart Contextual Heuristics Arbiter."
                    );
                }
            }
        } else {
            log::info!(
                "AI Provider belum dikonfigurasi, menggunakan Smart Contextual Heuristics Arbiter (Offline)."
            );
        }

        // Fallback: Smart Contextual Heuristics Arbiter (Offline Engine)
        self.heuristics_arbitrate(&buckets)
    }

    /// Membangun bucket waktu 1.0 detik yang menyatukan semua modalitas
    fn build_buckets(
        &self,
        visual: &[AnalysisSegment],
        audio: &[AnalysisSegment],
        voice: &[AnalysisSegment],
        text: &[AnalysisSegment],
        transcripts: &[TranscriptSnippet],
    ) -> Vec<MultimodalBucket> {
        let mut max_end_time = 0.0f64;
        for s in visual
            .iter()
            .chain(audio.iter())
            .chain(voice.iter())
            .chain(text.iter())
        {
            if s.end_time > max_end_time {
                max_end_time = s.end_time;
            }
        }
        for t in transcripts {
            if t.end > max_end_time {
                max_end_time = t.end;
            }
        }

        if max_end_time <= 0.0 {
            return vec![];
        }

        let bucket_duration = 1.0f64;
        let num_buckets = (max_end_time / bucket_duration).ceil() as usize;
        let mut buckets = Vec::with_capacity(num_buckets);

        for b in 0..num_buckets {
            let start = b as f64 * bucket_duration;
            let end = start + bucket_duration;

            // Kumpulkan kata-kata yang diucapkan pada jendela waktu ini
            let mut words_in_bucket = Vec::new();
            for t in transcripts {
                if t.start < end && t.end > start {
                    words_in_bucket.push(t.text.clone());
                }
            }
            let spoken_text = words_in_bucket.join(" ");

            // Visual
            let vis = visual
                .iter()
                .filter(|s| s.start_time < end && s.end_time > start)
                .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal));
            let visual_emotion = vis.map(|s| s.emotion.clone());
            let visual_score = vis.map(|s| s.score).unwrap_or(0.0);
            let visual_bbox = vis.and_then(|s| s.bounding_box.clone());

            // Voice
            let voi = voice
                .iter()
                .filter(|s| s.start_time < end && s.end_time > start)
                .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal));
            let voice_emotion = voi.map(|s| s.emotion.clone());
            let voice_score = voi.map(|s| s.score).unwrap_or(0.0);

            // Audio
            let aud = audio
                .iter()
                .filter(|s| s.start_time < end && s.end_time > start)
                .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal));
            let audio_emotion = aud.map(|s| s.emotion.clone());
            let audio_score = aud.map(|s| s.score).unwrap_or(0.0);

            // Text Sentiment
            let txt_seg = text
                .iter()
                .filter(|s| s.start_time < end && s.end_time > start)
                .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal));
            let text_emotion = txt_seg.map(|s| s.emotion.clone());
            let text_score = txt_seg.map(|s| s.score).unwrap_or(0.0);

            buckets.push(MultimodalBucket {
                start_time: start,
                end_time: end,
                spoken_text,
                visual_emotion,
                visual_score,
                visual_bbox,
                voice_emotion,
                voice_score,
                audio_emotion,
                audio_score,
                text_emotion,
                text_score,
            });
        }

        buckets
    }

    /// Panggil AI Provider untuk melakukan arbitrase emosi dan memilih VFX meme
    async fn call_ai_arbiter(
        &self,
        buckets: &[MultimodalBucket],
        config: &AIConfig,
        _progress: &Sender<ProgressEvent>,
    ) -> Result<
        (
            Vec<AnalysisSegment>,
            EmotionLabel,
            HashMap<String, f32>,
            Vec<ScheduledEffect>,
            Vec<ScheduledBuiltinEffect>,
        ),
        crate::error::CliptzyError,
    > {
        let provider = create_provider(config);

        // Format katalog efek
        let mut effects_desc = String::new();
        for eff in self.effects_mgr.all_effects() {
            let desc = get_effect_semantic_description(&eff.name);
            effects_desc.push_str(&format!("- \"{}\": {}\n", eff.name, desc));
        }

        // Format timeline multimodal (hanya sertakan bucket yang memiliki aktivitas)
        let mut timeline_lines = Vec::new();
        for b in buckets {
            let mut parts = Vec::new();
            if !b.spoken_text.is_empty() {
                parts.push(format!("Text: \"{}\"", b.spoken_text));
            }
            if let Some(ref vis) = b.visual_emotion {
                if *vis != EmotionLabel::Neutral || b.visual_score > 0.5 {
                    parts.push(format!("Visual: {:?} ({:.0}%)", vis, b.visual_score * 100.0));
                }
            }
            if let Some(ref voi) = b.voice_emotion {
                if *voi != EmotionLabel::Neutral || b.voice_score > 0.5 {
                    parts.push(format!("Voice: {:?} ({:.0}%)", voi, b.voice_score * 100.0));
                }
            }
            if let Some(ref aud) = b.audio_emotion {
                if *aud != EmotionLabel::Neutral || b.audio_score > 0.5 {
                    parts.push(format!("Audio: {:?} ({:.0}%)", aud, b.audio_score * 100.0));
                }
            }
            if let Some(ref txt) = b.text_emotion {
                if *txt != EmotionLabel::Neutral || b.text_score > 0.5 {
                    parts.push(format!("TextNLP: {:?} ({:.0}%)", txt, b.text_score * 100.0));
                }
            }

            if !parts.is_empty() {
                timeline_lines.push(format!(
                    "[{:.1}s - {:.1}s]: {}",
                    b.start_time,
                    b.end_time,
                    parts.join(" | ")
                ));
            }
        }

        let timeline_text = if timeline_lines.is_empty() {
            "[All intervals calm / neutral]".to_string()
        } else {
            timeline_lines.join("\n")
        };

        let prompt = format!(
r#"You are the Cliptzy AI Director, an elite video editor, comedic timing specialist, and multimodal emotion arbiter.
Your task is to analyze multimodal sensory observations from a video clip and:
1. Determine the TRUE contextual emotion for each active segment.
2. Direct comedic timing by choosing 2 to 6 impactful Video Meme VFX from the catalog to trigger at key moments.
3. Optionally schedule camera and color visual effects (FFmpeg filters) to punch up the action.

AVAILABLE VIDEO EFFECTS (MEMES):
{}

CRITICAL MULTIMODAL REASONING RULES:
- Anti-False Angry (CRITICAL):
  * Gamer/webcam faces frequently squint, frown, or furrow brows while focusing on games or speaking. THIS IS NOT RAGE!
  * NEVER classify a segment as 'Angry' unless there is clear corroboration in audio (screaming/shouting) or text (raging/swearing/slang rage).
  * A silent or calm person with a frowning face is NEUTRAL (focused gamer face).
  * DO NOT schedule rage memes ('cat_slamming table', 'tyler1_scream') for focused or calm faces!
- Context Overrides Facial Expression:
  * Scowling or grimacing while begging/surrendering ("udah bang", "ampun bang", "jangan bang", "mati gua") is FEAR / SURRENDER / COMEDIC PANIC, NOT genuine anger!
  * Laughing or smirking during awkward gameplay moments is HAPPY or CONFUSED, not neutral.
  * Shouting + jump-scare words ("kaget", "anjir", "astaga", screaming audio) is SHOCK or FEAR.
  * Sarcastic gamer slang: understand Indonesian & English gamer culture ("lah kok", "gimana dah" = Confused; "wkwk", "ez", "gg" = Happy; swearing + rage = Angry).
- Valid Emotion Labels: Happy, Angry, Shock, Fear, Sad, Confused, Neutral.
- VFX STACKING & PACING:
  * VFX STACKING IS ALLOWED AND ENCOURAGED! Multiple meme effects can overlap and stack (e.g. dramatic sound effect like 'sfx_vineboom' layered with a reaction meme like 'the rock_sus' or 'flightreact').
  * Fast reactions: Effects can trigger rapidly (0.8s - 2.0s apart) during intense climaxes or funny moments.
  * Maximum 2 effects active simultaneously at any given second.
  * Variety: Select varied effects from the catalog (do NOT repeat the exact same effect consecutively).
  * Select between 2 to 6 effects per clip to keep the video engaging and dynamic.
- BUILT-IN CAMERA & VISUAL EFFECTS (FFmpeg Filters):
  * "screen_shake": Shakes camera view intensely during shouting, rage, or explosion (duration ~0.6s)
  * "white_flash": Flashbang impact on jumpscare, sudden shock, or vineboom punchline (duration ~0.2s)
  * "dramatic_bw": Grayscale + vignette on surrender, tragic defeat, or death (duration ~2.0s)
  * "deep_fried": High contrast/saturation on chaos, ear-rape shouting, or intense anger (duration ~1.2s)
  * "punch_zoom": Snap zoom into face on confusion, suspicion, or awkward pause (duration ~0.8s)
  * "red_tint": Heavy red color wash on boiling rage, danger, low HP, or combat hostility (duration ~1.0s)
  * "negate": Inverted colors for cursed moments, sudden terrifying jumpscares, or dark plot twists (duration ~0.4s)
  * "focus_blur": Heavy gaussian blur for brain freeze, dizzy/stunned, silence, or awkward contemplation (duration ~1.2s)
  * "sepia": Warm nostalgic sepia tone for melancholic reflection, sad memories, or flashback defeat (duration ~2.5s)
  * "rainbow_hue": Rapid psychedelic rainbow hue cycling for celebration, victory hype, GG, or wheezing laughter (duration ~1.8s)

MULTIMODAL TIMELINE:
{}

OUTPUT FORMAT:
Return ONLY a valid raw JSON object. Do not include markdown or backticks if possible:
{{
  "segments": [
    {{
      "start": 0.0,
      "end": 2.0,
      "emotion": "Fear",
      "score": 0.85,
      "reasoning": "Speaker pleads 'udah bang' indicating surrender, grimace is panic not anger"
    }}
  ],
  "dominant_emotion": "Fear",
  "vfx_schedule": [
    {{
      "start_time": 1.0,
      "effect_name": "gta_ah shit here we go again",
      "reason": "Player surrenders in defeat"
    }}
  ],
  "builtin_schedule": [
    {{
      "start_time": 1.0,
      "effect_type": "dramatic_bw",
      "duration": 2.2
    }}
  ]
}}"#,
            effects_desc, timeline_text
        );

        let raw_res = provider.generate(&prompt, None).await?;
        let cleaned = extract_json_object(&raw_res);

        let parsed: AiArbiterResponse = serde_json::from_str(cleaned).map_err(|e| {
            crate::error::CliptzyError::AIProvider(format!(
                "Failed to parse AI Director JSON response: {}. Raw was: {}",
                e, raw_res
            ))
        })?;

        // 1. Map segments
        let mut final_segments = Vec::new();
        let mut distribution_scores: HashMap<String, f32> = HashMap::new();

        for seg_resp in parsed.segments {
            let emo = parse_emotion_label(&seg_resp.emotion);
            let emo_str = format!("{:?}", emo);
            *distribution_scores.entry(emo_str).or_insert(0.0) += seg_resp.score;

            // Cari bounding box visual terdekat pada interval ini
            let bbox = buckets
                .iter()
                .find(|b| {
                    b.start_time <= seg_resp.end
                        && b.end_time >= seg_resp.start
                        && b.visual_bbox.is_some()
                })
                .and_then(|b| b.visual_bbox.clone());

            final_segments.push(AnalysisSegment {
                start_time: seg_resp.start,
                end_time: seg_resp.end,
                emotion: emo,
                score: seg_resp.score.clamp(0.0, 1.0),
                bounding_box: bbox,
            });
        }

        // Dominant emotion
        let dominant_emotion = if let Some(ref dom_str) = parsed.dominant_emotion {
            parse_emotion_label(dom_str)
        } else {
            distribution_scores
                .iter()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(k, _)| parse_emotion_label(k))
                .unwrap_or(EmotionLabel::Neutral)
        };

        // Normalize distribution
        let total_dist: f32 = distribution_scores.values().sum();
        if total_dist > 0.0 {
            for v in distribution_scores.values_mut() {
                *v /= total_dist;
            }
        }

        // 2. Map VFX schedule (mendukung penumpukan/stacking hingga 2 efek simultan)
        let mut scheduled_vfx = Vec::new();

        for vfx_item in parsed.vfx_schedule {
            let start_t = vfx_item.start_time;

            // Hitung efek yang aktif pada start_t (izinkan hingga 2 efek bersamaan)
            let active_count = scheduled_vfx
                .iter()
                .filter(|e: &&ScheduledEffect| e.start_time <= start_t && e.end_time > start_t)
                .count();

            if active_count < 2 {
                // Hindari efek yang sama persis diulang dalam 3.5 detik
                let same_recently = scheduled_vfx
                    .iter()
                    .any(|e| e.effect.name == vfx_item.effect_name && (e.start_time - start_t).abs() < 3.5);

                // Jeda mulai antar efek setidaknya 0.8 detik
                let too_close_start = scheduled_vfx
                    .iter()
                    .any(|e| (e.start_time - start_t).abs() < 0.8);

                if !same_recently && !too_close_start {
                    if let Some(effect) = self.effects_mgr.get_effect_by_name(&vfx_item.effect_name) {
                        if effect.resolve_path().exists() {
                            let end_t = start_t + effect.get_duration();
                            scheduled_vfx.push(ScheduledEffect {
                                effect: effect.clone(),
                                start_time: start_t,
                                end_time: end_t,
                            });
                        }
                    }
                }
            }
        }

        scheduled_vfx.sort_by(|a, b| {
            a.start_time
                .partial_cmp(&b.start_time)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // 3. Map Built-in Visual Effects (FFmpeg Filters)
        let mut scheduled_builtin = Vec::new();
        for b_item in parsed.builtin_schedule {
            let b_type = match b_item.effect_type.to_lowercase().as_str() {
                "screen_shake" | "shake" => Some(BuiltinEffectType::ScreenShake),
                "white_flash" | "flash" => Some(BuiltinEffectType::WhiteFlash),
                "dramatic_bw" | "black_and_white" | "bw" => Some(BuiltinEffectType::DramaticBW),
                "deep_fried" | "deepfried" => Some(BuiltinEffectType::DeepFried),
                "punch_zoom" | "zoom" => Some(BuiltinEffectType::PunchZoom),
                "red_tint" | "red" => Some(BuiltinEffectType::RedTint),
                "negate" | "invert" => Some(BuiltinEffectType::Negate),
                "focus_blur" | "blur" => Some(BuiltinEffectType::FocusBlur),
                "sepia" => Some(BuiltinEffectType::Sepia),
                "rainbow_hue" | "rainbow" => Some(BuiltinEffectType::RainbowHue),
                _ => None,
            };

            if let Some(t) = b_type {
                let default_dur = match t {
                    BuiltinEffectType::WhiteFlash => 0.25,
                    BuiltinEffectType::ScreenShake => 0.65,
                    BuiltinEffectType::DramaticBW => 2.20,
                    BuiltinEffectType::DeepFried => 1.20,
                    BuiltinEffectType::PunchZoom => 0.80,
                    BuiltinEffectType::RedTint => 1.00,
                    BuiltinEffectType::Negate => 0.40,
                    BuiltinEffectType::FocusBlur => 1.20,
                    BuiltinEffectType::Sepia => 2.50,
                    BuiltinEffectType::RainbowHue => 1.80,
                };
                let dur = b_item.duration.unwrap_or(default_dur).clamp(0.15, 3.0);
                let start_t = b_item.start_time;

                let recently_used = scheduled_builtin.iter().any(|b: &ScheduledBuiltinEffect| {
                    b.effect_type == t && (b.start_time - start_t).abs() < 3.0
                });

                if !recently_used && scheduled_builtin.len() < 6 {
                    scheduled_builtin.push(ScheduledBuiltinEffect {
                        effect_type: t,
                        start_time: start_t,
                        end_time: start_t + dur,
                    });
                }
            }
        }

        // Jika AI tidak mengembalikan builtin_schedule, fallback ke heuristik bawaan
        if scheduled_builtin.is_empty() {
            let (_, _, _, _, fallback_builtin) = self.heuristics_arbitrate(buckets);
            scheduled_builtin = fallback_builtin;
        }

        scheduled_builtin.sort_by(|a, b| {
            a.start_time
                .partial_cmp(&b.start_time)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok((
            final_segments,
            dominant_emotion,
            distribution_scores,
            scheduled_vfx,
            scheduled_builtin,
        ))
    }

    /// Smart Contextual Heuristics Arbiter (Offline Engine)
    /// Menyelesaikan kontradiksi multimodal secara cerdas tanpa koneksi internet/LLM
    pub fn heuristics_arbitrate(
        &self,
        buckets: &[MultimodalBucket],
    ) -> (
        Vec<AnalysisSegment>,
        EmotionLabel,
        HashMap<String, f32>,
        Vec<ScheduledEffect>,
        Vec<ScheduledBuiltinEffect>,
    ) {
        let mut final_segments = Vec::new();
        let mut distribution_scores: HashMap<String, f32> = HashMap::new();

        // 1. Analisis kontekstual per bucket
        for b in buckets {
            let txt = b.spoken_text.to_lowercase();
            let mut resolved_emotion = EmotionLabel::Neutral;
            let mut score = 0.5f32;

            // A. Surrender / Pleading / Defeat phrases ("udah bang", "ampun", dsb)
            let is_surrender = txt.contains("udah bang")
                || txt.contains("uda bang")
                || txt.contains("ampun")
                || txt.contains("jangan bang")
                || txt.contains("jgn bang")
                || txt.contains("plis")
                || txt.contains("mati gua")
                || txt.contains("mati gue")
                || txt.contains("mati gw")
                || txt.contains("mati kan")
                || txt.contains("kabur")
                || txt.contains("serem")
                || txt.contains("takut")
                || txt.contains("tolong")
                || txt.contains("tolongin");

            // B. Confusion / Bewilderment phrases
            let is_confused = txt.contains("lah kok")
                || txt.contains("kok gitu")
                || txt.contains("hah")
                || txt.contains("gimana dah")
                || txt.contains("apaan")
                || txt.contains("kenapa")
                || txt.contains("kok bisa")
                || txt.contains("bingung")
                || txt.contains("lah?");

            // C. Shock / Jump-scare phrases
            let is_shock = txt.contains("kaget")
                || txt.contains("anjir")
                || txt.contains("astaga")
                || txt.contains("waduh")
                || txt.contains("woi")
                || txt.contains("astagfirullah")
                || (b.audio_emotion == Some(EmotionLabel::Shock) && b.audio_score > 0.6);

            // D. Happy / Laugh
            let is_happy = txt.contains("wkwk")
                || txt.contains("haha")
                || txt.contains("lucu")
                || txt.contains("mantap")
                || txt.contains("asik")
                || txt.contains("gg")
                || txt.contains("ez")
                || (b.audio_emotion == Some(EmotionLabel::Happy) && b.audio_score > 0.6);

            // E. Rage / Swearing
            let is_rage = txt.contains("anjing")
                || txt.contains("babi")
                || txt.contains("bangsat")
                || txt.contains("kontol")
                || txt.contains("rusak")
                || txt.contains("sialan");

            if is_surrender {
                // Walaupun visual terdeteksi 'Angry' (meringis ketakutan), konteks ucapan menyerah => FEAR
                resolved_emotion = EmotionLabel::Fear;
                score = 0.85f32.max(b.text_score).max(b.voice_score).min(1.0);
            } else if is_confused {
                resolved_emotion = EmotionLabel::Confused;
                score = 0.80f32.max(b.visual_score).max(b.text_score).min(1.0);
            } else if is_shock {
                resolved_emotion = EmotionLabel::Shock;
                score = 0.85f32.max(b.audio_score).max(b.visual_score).min(1.0);
            } else if is_happy {
                resolved_emotion = EmotionLabel::Happy;
                score = 0.85f32.max(b.audio_score).max(b.text_score).min(1.0);
            } else if is_rage {
                resolved_emotion = EmotionLabel::Angry;
                score = 0.90f32.max(b.audio_score).max(b.voice_score).min(1.0);
            } else {
                // Evaluasi cross-modal tertimbang cerdas
                let mut votes: HashMap<EmotionLabel, f32> = HashMap::new();
                if let Some(ref emo) = b.visual_emotion {
                    // Wajah scowling tanpa kata marah tidak boleh mendominasi penuh
                    let w = if *emo == EmotionLabel::Angry && b.voice_emotion == Some(EmotionLabel::Neutral) {
                        0.15
                    } else {
                        0.30
                    };
                    *votes.entry(emo.clone()).or_insert(0.0) += b.visual_score * w;
                }
                if let Some(ref emo) = b.voice_emotion {
                    *votes.entry(emo.clone()).or_insert(0.0) += b.voice_score * 0.35;
                }
                if let Some(ref emo) = b.audio_emotion {
                    *votes.entry(emo.clone()).or_insert(0.0) += b.audio_score * 0.25;
                }
                if let Some(ref emo) = b.text_emotion {
                    *votes.entry(emo.clone()).or_insert(0.0) += b.text_score * 0.10;
                }

                if let Some((top_emo, top_score)) = votes
                    .into_iter()
                    .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
                {
                    if top_emo == EmotionLabel::Angry {
                        // Anti-False Angry: Kemarahan sejati WAJIB memiliki bukti suara (audio shouting/angry tone)
                        // atau teks kata-kata kasar/makian. Jika hanya wajah saja, ubah menjadi Neutral (wajah fokus).
                        let has_audio_corroboration = b.audio_emotion == Some(EmotionLabel::Angry)
                            || (b.audio_emotion == Some(EmotionLabel::Shock) && b.audio_score > 0.6)
                            || (b.voice_emotion == Some(EmotionLabel::Angry) && b.voice_score > 0.5);

                        if has_audio_corroboration || is_rage {
                            resolved_emotion = EmotionLabel::Angry;
                            score = top_score.clamp(0.0, 1.0);
                        } else {
                            resolved_emotion = EmotionLabel::Neutral;
                            score = 0.5;
                        }
                    } else {
                        resolved_emotion = top_emo;
                        score = top_score.clamp(0.0, 1.0);
                    }
                }
            }

            let emo_str = format!("{:?}", resolved_emotion);
            *distribution_scores.entry(emo_str).or_insert(0.0) += score;

            final_segments.push(AnalysisSegment {
                start_time: b.start_time,
                end_time: b.end_time,
                emotion: resolved_emotion,
                score,
                bounding_box: b.visual_bbox.clone(),
            });
        }

        // 2. Dominant emotion
        let mut dominant_emotion = EmotionLabel::Neutral;
        let mut max_dist = 0.0f32;
        for (emo_str, sc) in &distribution_scores {
            if *sc > max_dist {
                max_dist = *sc;
                dominant_emotion = parse_emotion_label(emo_str);
            }
        }

        let total_dist: f32 = distribution_scores.values().sum();
        if total_dist > 0.0 {
            for v in distribution_scores.values_mut() {
                *v /= total_dist;
            }
        }

        // 3. VFX Meme Selection (Mendukung Penumpukan/Stacking hingga 2 Efek Bersamaan)
        let mut scheduled_vfx: Vec<ScheduledEffect> = Vec::new();

        // Cari segmen dengan emosi kuat non-neutral
        let mut candidates: Vec<&AnalysisSegment> = final_segments
            .iter()
            .filter(|s| s.emotion != EmotionLabel::Neutral && s.score >= 0.60)
            .collect();
        candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        for cand in &candidates {
            if scheduled_vfx.len() >= 6 {
                break;
            }

            let start_t = cand.start_time;

            // Hitung efek yang sedang aktif di detik start_t (izinkan hingga 2 efek simultan)
            let active_count = scheduled_vfx
                .iter()
                .filter(|e: &&ScheduledEffect| e.start_time <= start_t && e.end_time > start_t)
                .count();

            if active_count < 2 {
                let too_close_start = scheduled_vfx
                    .iter()
                    .any(|e| (e.start_time - start_t).abs() < 1.0);

                if !too_close_start {
                    let effects_to_consider: &[&str] = match cand.emotion {
                        EmotionLabel::Fear => {
                            let is_surrender = buckets
                                .iter()
                                .find(|b| b.start_time <= cand.end_time && b.end_time >= cand.start_time)
                                .map(|b| b.spoken_text.to_lowercase())
                                .map(|t| t.contains("udah") || t.contains("ampun") || t.contains("mati"))
                                .unwrap_or(false);

                            if is_surrender {
                                &["gta_ah shit here we go again", "ksi_nononono"]
                            } else {
                                &["ishowspeed_scream", "dog_scared"]
                            }
                        }
                        EmotionLabel::Confused => &["the rock_sus", "sfx_huh", "jhon travolta_confused"],
                        EmotionLabel::Shock => &["sfx_vineboom", "flightreact_woah woah hey hey", "man_shocked"],
                        EmotionLabel::Angry => &["cat_slamming table", "tyler1_scream"],
                        EmotionLabel::Happy => &["penguiz0_wooo yeah babyyy", "risitas_laugh", "sfx_kerja bagus", "ainsley harriott_hehe buoi"],
                        EmotionLabel::Sad => &["sfx_spongebob disappoint", "man_crying"],
                        _ => &[],
                    };

                    // Jadwalkan efek pertama yang belum aktif baru-baru ini
                    for &eff_name in effects_to_consider {
                        let same_recently = scheduled_vfx
                            .iter()
                            .any(|e| e.effect.name == eff_name && (e.start_time - start_t).abs() < 4.0);

                        if !same_recently {
                            if let Some(effect) = self.effects_mgr.get_effect_by_name(eff_name) {
                                if effect.resolve_path().exists() {
                                    let first_dur = effect.get_duration();
                                    scheduled_vfx.push(ScheduledEffect {
                                        effect: effect.clone(),
                                        start_time: start_t,
                                        end_time: start_t + first_dur,
                                    });

                                    // Jika momen sangat klimaks (score >= 0.85) dan layar belum ada efek lain,
                                    // izinkan penumpukan efek sekunder / combo punchline yang tumpang tindih!
                                    if cand.score >= 0.85 && effects_to_consider.len() > 1 && scheduled_vfx.len() < 6 {
                                        let second_eff_name = effects_to_consider[1];
                                        if let Some(second_effect) = self.effects_mgr.get_effect_by_name(second_eff_name) {
                                            if second_effect.resolve_path().exists() {
                                                let combo_offset = (first_dur * 0.5).clamp(0.4, 1.0);
                                                let second_start = start_t + combo_offset;
                                                let second_dur = second_effect.get_duration();
                                                scheduled_vfx.push(ScheduledEffect {
                                                    effect: second_effect.clone(),
                                                    start_time: second_start,
                                                    end_time: second_start + second_dur,
                                                });
                                            }
                                        }
                                    }
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Urutkan jadwal VFX berdasarkan waktu mulai
        scheduled_vfx.sort_by(|a, b| {
            a.start_time
                .partial_cmp(&b.start_time)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // 4. Built-in FFmpeg Visual Effects Scheduling (Shake, Flash, B&W, DeepFried, PunchZoom, RedTint, Negate, FocusBlur, Sepia, RainbowHue)
        let mut scheduled_builtin: Vec<ScheduledBuiltinEffect> = Vec::new();
        for cand in &candidates {
            if scheduled_builtin.len() >= 6 {
                break;
            }
            let start_t = cand.start_time;

            let builtin_to_try: Option<(BuiltinEffectType, f64)> = match cand.emotion {
                EmotionLabel::Shock => Some((BuiltinEffectType::WhiteFlash, 0.25)),
                EmotionLabel::Angry => Some((BuiltinEffectType::ScreenShake, 0.65)),
                EmotionLabel::Fear => {
                    let is_surrender = buckets
                        .iter()
                        .find(|b| b.start_time <= cand.end_time && b.end_time >= cand.start_time)
                        .map(|b| b.spoken_text.to_lowercase())
                        .map(|t| t.contains("udah") || t.contains("ampun") || t.contains("mati"))
                        .unwrap_or(false);
                    if is_surrender {
                        Some((BuiltinEffectType::DramaticBW, 2.20))
                    } else {
                        Some((BuiltinEffectType::WhiteFlash, 0.25))
                    }
                }
                EmotionLabel::Sad => Some((BuiltinEffectType::DramaticBW, 2.20)),
                EmotionLabel::Confused => Some((BuiltinEffectType::PunchZoom, 0.80)),
                EmotionLabel::Happy => Some((BuiltinEffectType::RainbowHue, 1.80)),
                _ => None,
            };

            if let Some((b_type, dur)) = builtin_to_try {
                // Cooldown: minimal 3.0s antar efek bertipe sama
                let recently_used = scheduled_builtin.iter().any(|b| {
                    b.effect_type == b_type && (b.start_time - start_t).abs() < 3.0
                });

                if !recently_used {
                    scheduled_builtin.push(ScheduledBuiltinEffect {
                        effect_type: b_type,
                        start_time: start_t,
                        end_time: start_t + dur,
                    });

                    // Combos & Punchlines
                    if b_type == BuiltinEffectType::ScreenShake && scheduled_builtin.len() < 6 {
                        // Jika amarah cukup tinggi (score >= 0.80), kombinasikan dengan RedTint (nuansa merah darah)
                        if cand.score >= 0.80 {
                            scheduled_builtin.push(ScheduledBuiltinEffect {
                                effect_type: BuiltinEffectType::RedTint,
                                start_time: start_t,
                                end_time: start_t + 1.00,
                            });
                        }
                        // Jika amarah luar biasa klimaks (score >= 0.88), combo juga dengan DeepFried!
                        if cand.score >= 0.88 && scheduled_builtin.len() < 6 {
                            scheduled_builtin.push(ScheduledBuiltinEffect {
                                effect_type: BuiltinEffectType::DeepFried,
                                start_time: start_t,
                                end_time: start_t + 1.20,
                            });
                        }
                    } else if b_type == BuiltinEffectType::WhiteFlash && cand.score >= 0.88 && scheduled_builtin.len() < 6 {
                        // Jumpscare / shock ekstrem -> combo dengan Negate (inversi warna horor)
                        scheduled_builtin.push(ScheduledBuiltinEffect {
                            effect_type: BuiltinEffectType::Negate,
                            start_time: start_t + 0.25,
                            end_time: start_t + 0.65,
                        });
                    } else if b_type == BuiltinEffectType::PunchZoom && cand.score >= 0.85 && scheduled_builtin.len() < 6 {
                        // Kebingungan mendalam / freeze otak -> FocusBlur
                        scheduled_builtin.push(ScheduledBuiltinEffect {
                            effect_type: BuiltinEffectType::FocusBlur,
                            start_time: start_t + 0.50,
                            end_time: start_t + 1.70,
                        });
                    } else if b_type == BuiltinEffectType::DramaticBW && cand.emotion == EmotionLabel::Sad && scheduled_builtin.len() < 6 {
                        // Momen kesedihan / refleksi flashback -> Sepia tone hangat
                        scheduled_builtin.push(ScheduledBuiltinEffect {
                            effect_type: BuiltinEffectType::Sepia,
                            start_time: start_t,
                            end_time: start_t + 2.50,
                        });
                    }
                }
            }
        }

        scheduled_builtin.sort_by(|a, b| {
            a.start_time
                .partial_cmp(&b.start_time)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        (
            final_segments,
            dominant_emotion,
            distribution_scores,
            scheduled_vfx,
            scheduled_builtin,
        )
    }
}

/// Helper untuk membersihkan string response dan mengambil blok JSON
fn extract_json_object(raw: &str) -> &str {
    let trimmed = raw.trim();

    // Cek format markdown code block ```json ... ```
    let regex = RE_JSON_FENCE.get_or_init(|| Regex::new(r"(?s)```(?:json)?\s*(\{.*?\})\s*```").unwrap());
    if let Some(caps) = regex.captures(trimmed) {
        if let Some(m) = caps.get(1) {
            return m.as_str();
        }
    }

    // Ambil string di antara kurung kurawal terluar { ... }
    if let (Some(start), Some(end)) = (trimmed.find('{'), trimmed.rfind('}')) {
        if start <= end {
            return &trimmed[start..=end];
        }
    }

    trimmed
}

fn parse_emotion_label(s: &str) -> EmotionLabel {
    match s.trim().to_lowercase().as_str() {
        "happy" => EmotionLabel::Happy,
        "angry" => EmotionLabel::Angry,
        "shock" => EmotionLabel::Shock,
        "fear" => EmotionLabel::Fear,
        "sad" => EmotionLabel::Sad,
        "confused" => EmotionLabel::Confused,
        _ => EmotionLabel::Neutral,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surrender_overrides_angry_face() {
        let arbiter = ContextArbiter::new();
        let buckets = vec![MultimodalBucket {
            start_time: 0.0,
            end_time: 1.0,
            spoken_text: "udah bang ampun bang".to_string(),
            visual_emotion: Some(EmotionLabel::Angry),
            visual_score: 0.90,
            visual_bbox: None,
            voice_emotion: Some(EmotionLabel::Neutral),
            voice_score: 0.60,
            audio_emotion: Some(EmotionLabel::Neutral),
            audio_score: 0.20,
            text_emotion: Some(EmotionLabel::Fear),
            text_score: 0.70,
        }];

        let (segments, dominant, _, vfx, builtin) = arbiter.heuristics_arbitrate(&buckets);

        assert_eq!(segments.len(), 1);
        // "udah bang" context MUST override visual angry to Fear!
        assert_eq!(segments[0].emotion, EmotionLabel::Fear);
        assert_eq!(dominant, EmotionLabel::Fear);
        assert!(!vfx.is_empty(), "Harus ada VFX surrender yang dijadwalkan");
        assert_eq!(vfx[0].effect.name, "gta_ah shit here we go again");
        // Momen menyerah / wasted harus memicu efek builtin DramaticBW (grayscale + vignette)
        assert!(
            builtin.iter().any(|b| b.effect_type == BuiltinEffectType::DramaticBW),
            "Harus menjadwalkan DramaticBW pada momen menyerah"
        );
    }

    #[test]
    fn test_confusion_phrase_resolution() {
        let arbiter = ContextArbiter::new();
        let buckets = vec![MultimodalBucket {
            start_time: 2.0,
            end_time: 3.0,
            spoken_text: "lah kok gitu apaan dah".to_string(),
            visual_emotion: Some(EmotionLabel::Neutral),
            visual_score: 0.50,
            visual_bbox: None,
            voice_emotion: Some(EmotionLabel::Neutral),
            voice_score: 0.50,
            audio_emotion: Some(EmotionLabel::Neutral),
            audio_score: 0.20,
            text_emotion: Some(EmotionLabel::Neutral),
            text_score: 0.30,
        }];

        let (segments, dominant, _, vfx, builtin) = arbiter.heuristics_arbitrate(&buckets);

        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].emotion, EmotionLabel::Confused);
        assert_eq!(dominant, EmotionLabel::Confused);
        assert!(!vfx.is_empty());
        assert_eq!(vfx[0].effect.name, "the rock_sus");
        // Momen bingung harus menjadwalkan PunchZoom
        assert!(
            builtin.iter().any(|b| b.effect_type == BuiltinEffectType::PunchZoom),
            "Harus menjadwalkan PunchZoom pada momen bingung"
        );
    }

    #[test]
    fn test_uncorroborated_angry_face_suppression() {
        let arbiter = ContextArbiter::new();
        // Wajah terdeteksi Angry (misal fokus/mengernyit), tapi suara tenang, audio ambient, teks normal
        let buckets = vec![MultimodalBucket {
            start_time: 0.0,
            end_time: 1.0,
            spoken_text: "ini gimana cara mainnya ya".to_string(),
            visual_emotion: Some(EmotionLabel::Angry),
            visual_score: 0.85,
            visual_bbox: None,
            voice_emotion: Some(EmotionLabel::Neutral),
            voice_score: 0.60,
            audio_emotion: Some(EmotionLabel::Neutral),
            audio_score: 0.20,
            text_emotion: Some(EmotionLabel::Neutral),
            text_score: 0.30,
        }];

        let (segments, dominant, _, vfx, builtin) = arbiter.heuristics_arbitrate(&buckets);

        assert_eq!(segments.len(), 1);
        // Harus ditekan menjadi Neutral karena tidak ada bukti suara/kata marah!
        assert_eq!(segments[0].emotion, EmotionLabel::Neutral);
        assert_eq!(dominant, EmotionLabel::Neutral);
        // Tidak boleh menjadwalkan efek banting meja atau guncang layar pada wajah fokus normal
        assert!(vfx.is_empty(), "Tidak boleh ada rage meme pada wajah fokus normal");
        assert!(builtin.is_empty(), "Tidak boleh ada camera shake pada wajah fokus normal");
    }

    #[test]
    fn test_vfx_stacking_concurrency() {
        let arbiter = ContextArbiter::new();
        // Momen kejutan klimaks tinggi (score 0.90) yang memicu combo stacking
        let buckets = vec![MultimodalBucket {
            start_time: 5.0,
            end_time: 6.0,
            spoken_text: "astaga kaget anjir".to_string(),
            visual_emotion: Some(EmotionLabel::Shock),
            visual_score: 0.90,
            visual_bbox: None,
            voice_emotion: Some(EmotionLabel::Shock),
            voice_score: 0.85,
            audio_emotion: Some(EmotionLabel::Shock),
            audio_score: 0.85,
            text_emotion: Some(EmotionLabel::Shock),
            text_score: 0.80,
        }];

        let (_, _, _, vfx, builtin) = arbiter.heuristics_arbitrate(&buckets);

        // Harus ada setidaknya 2 efek bertumpuk (combo: vineboom + reaction)
        assert!(vfx.len() >= 2, "Harus menjadwalkan penumpukan efek (stacking) pada momen klimaks");
        assert_eq!(vfx[0].effect.name, "sfx_vineboom");
        assert!(vfx[1].start_time < vfx[0].end_time, "Efek harus tumpang tindih (stacking)");

        // Shock klimaks harus memicu WhiteFlash impact
        assert!(
            builtin.iter().any(|b| b.effect_type == BuiltinEffectType::WhiteFlash),
            "Harus menjadwalkan WhiteFlash pada momen shock klimaks"
        );
    }

    #[test]
    fn test_rage_triggers_shake_and_deepfried() {
        let arbiter = ContextArbiter::new();
        // Kemarahan sejati dengan umpatan dan audio screaming
        let buckets = vec![MultimodalBucket {
            start_time: 10.0,
            end_time: 11.0,
            spoken_text: "anjing bangsat rusak".to_string(),
            visual_emotion: Some(EmotionLabel::Angry),
            visual_score: 0.92,
            visual_bbox: None,
            voice_emotion: Some(EmotionLabel::Angry),
            voice_score: 0.90,
            audio_emotion: Some(EmotionLabel::Angry),
            audio_score: 0.88,
            text_emotion: Some(EmotionLabel::Angry),
            text_score: 0.95,
        }];

        let (segments, dominant, _, _, builtin) = arbiter.heuristics_arbitrate(&buckets);

        assert_eq!(segments[0].emotion, EmotionLabel::Angry);
        assert_eq!(dominant, EmotionLabel::Angry);
        // Rage ekstrem (score >= 0.88) harus memicu ScreenShake + DeepFried combo
        assert!(
            builtin.iter().any(|b| b.effect_type == BuiltinEffectType::ScreenShake),
            "Harus menjadwalkan ScreenShake pada amarah sejati"
        );
        assert!(
            builtin.iter().any(|b| b.effect_type == BuiltinEffectType::DeepFried),
            "Harus menjadwalkan DeepFried pada amarah ekstrem"
        );
        assert!(
            builtin.iter().any(|b| b.effect_type == BuiltinEffectType::RedTint),
            "Harus menjadwalkan RedTint pada amarah ekstrem"
        );
    }

    #[test]
    fn test_happy_triggers_rainbow_hue() {
        let arbiter = ContextArbiter::new();
        let buckets = vec![MultimodalBucket {
            start_time: 1.0,
            end_time: 2.0,
            spoken_text: "wkwk mantap gg ez".to_string(),
            visual_emotion: Some(EmotionLabel::Happy),
            visual_score: 0.85,
            visual_bbox: None,
            voice_emotion: Some(EmotionLabel::Happy),
            voice_score: 0.80,
            audio_emotion: Some(EmotionLabel::Happy),
            audio_score: 0.75,
            text_emotion: Some(EmotionLabel::Happy),
            text_score: 0.90,
        }];

        let (segments, dominant, _, _, builtin) = arbiter.heuristics_arbitrate(&buckets);

        assert_eq!(segments[0].emotion, EmotionLabel::Happy);
        assert_eq!(dominant, EmotionLabel::Happy);
        assert!(
            builtin.iter().any(|b| b.effect_type == BuiltinEffectType::RainbowHue),
            "Harus menjadwalkan RainbowHue pada momen kemenangan/tawa Happy"
        );
    }

    #[test]
    fn test_builtin_combos_negate_focusblur_sepia() {
        let arbiter = ContextArbiter::new();

        // 1. Extreme shock -> Negate combo
        let shock_bucket = vec![MultimodalBucket {
            start_time: 0.0,
            end_time: 1.0,
            spoken_text: "astaga anjir kaget".to_string(),
            visual_emotion: Some(EmotionLabel::Shock),
            visual_score: 0.95,
            visual_bbox: None,
            voice_emotion: Some(EmotionLabel::Shock),
            voice_score: 0.90,
            audio_emotion: Some(EmotionLabel::Shock),
            audio_score: 0.90,
            text_emotion: Some(EmotionLabel::Shock),
            text_score: 0.85,
        }];
        let (_, _, _, _, builtin_shock) = arbiter.heuristics_arbitrate(&shock_bucket);
        assert!(
            builtin_shock.iter().any(|b| b.effect_type == BuiltinEffectType::Negate),
            "Harus menjadwalkan Negate pada shock ekstrem"
        );

        // 2. High confusion -> FocusBlur combo
        let conf_bucket = vec![MultimodalBucket {
            start_time: 0.0,
            end_time: 1.0,
            spoken_text: "lah kok bingung gimana dah apaan".to_string(),
            visual_emotion: Some(EmotionLabel::Confused),
            visual_score: 0.90,
            visual_bbox: None,
            voice_emotion: Some(EmotionLabel::Confused),
            voice_score: 0.85,
            audio_emotion: Some(EmotionLabel::Neutral),
            audio_score: 0.20,
            text_emotion: Some(EmotionLabel::Confused),
            text_score: 0.85,
        }];
        let (_, _, _, _, builtin_conf) = arbiter.heuristics_arbitrate(&conf_bucket);
        assert!(
            builtin_conf.iter().any(|b| b.effect_type == BuiltinEffectType::FocusBlur),
            "Harus menjadwalkan FocusBlur pada kebingungan mendalam"
        );

        // 3. Sadness -> Sepia combo
        let sad_bucket = vec![MultimodalBucket {
            start_time: 0.0,
            end_time: 1.0,
            spoken_text: "sedih banget ya".to_string(),
            visual_emotion: Some(EmotionLabel::Sad),
            visual_score: 0.90,
            visual_bbox: None,
            voice_emotion: Some(EmotionLabel::Sad),
            voice_score: 0.85,
            audio_emotion: Some(EmotionLabel::Sad),
            audio_score: 0.80,
            text_emotion: Some(EmotionLabel::Sad),
            text_score: 0.90,
        }];
        let (_, _, _, _, builtin_sad) = arbiter.heuristics_arbitrate(&sad_bucket);
        assert!(
            builtin_sad.iter().any(|b| b.effect_type == BuiltinEffectType::Sepia),
            "Harus menjadwalkan Sepia pada momen kesedihan mendalam"
        );
    }
}
