use super::audio::AudioEventAnalyzer;
use super::text::TextSentimentAnalyzer;
use super::visual::VisualEmotionAnalyzer;
use super::voice::VoiceEmotionAnalyzer;
use super::{AnalysisSegment, EmotionAnalyzer, EmotionLabel};
use crate::config::models::AIConfig;
use crate::error::CliptzyError;
use crate::orchestrator::pipeline::ProgressEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;

pub struct EmotionFusion {
    pub visual: VisualEmotionAnalyzer,
    pub audio: AudioEventAnalyzer,
    pub voice: VoiceEmotionAnalyzer,
    pub text: TextSentimentAnalyzer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionTimeline {
    pub segments: Vec<AnalysisSegment>,
    pub dominant_emotion: EmotionLabel,
    pub emotion_distribution: HashMap<String, f32>, // Using String for EmotionLabel for simpler serialization
    #[serde(default)]
    pub visual: Vec<AnalysisSegment>,
    #[serde(default)]
    pub audio: Vec<AnalysisSegment>,
    #[serde(default)]
    pub voice: Vec<AnalysisSegment>,
    #[serde(default)]
    pub text: Vec<AnalysisSegment>,
    #[serde(default)]
    pub scheduled_effects: Vec<crate::processing::effects::ScheduledEffect>,
    #[serde(default)]
    pub scheduled_builtin_effects: Vec<crate::processing::burner::builtin::ScheduledBuiltinEffect>,
}

impl EmotionFusion {
    pub fn new() -> Self {
        Self {
            visual: VisualEmotionAnalyzer::new(),
            audio: AudioEventAnalyzer::new(),
            voice: VoiceEmotionAnalyzer::new(),
            text: TextSentimentAnalyzer::new(),
        }
    }

    /// Run all analyzers and fuse their outputs using weighted voting
    pub async fn analyze_fusion(
        &self,
        video_path: &Path, // Used for Visual and Audio (we might need to extract audio for wav/voice)
        audio_path: &Path, // Extracted WAV path for Audio/Voice analyzers
        transcript_path: &Path, // Transcript JSON for Text Analyzer
        config: &AIConfig,
        cancel: &CancellationToken,
        progress: &Sender<ProgressEvent>,
    ) -> Result<EmotionTimeline, CliptzyError> {
        let _ = progress.send(ProgressEvent {
            stage: "fusion".into(),
            label: "Starting Multi-Modal Emotion Fusion".into(),
            current: 0,
            total: 100,
            detail: None,
        });

        let visual_fut = async {
            if config.use_emotion_detection {
                self.visual.analyze(video_path, cancel, progress).await
            } else {
                Ok(vec![])
            }
        };

        let audio_fut = async {
            if config.use_audio_analysis {
                self.audio.analyze(audio_path, cancel, progress).await
            } else {
                Ok(vec![])
            }
        };

        let voice_fut = async {
            if config.use_voice_analysis {
                self.voice.analyze(audio_path, cancel, progress).await
            } else {
                Ok(vec![])
            }
        };

        let text_fut = async {
            if config.use_text_analysis {
                self.text.analyze(transcript_path, cancel, progress).await
            } else {
                Ok(vec![])
            }
        };

        let (visual_res, audio_res, voice_res, text_res) =
            tokio::join!(visual_fut, audio_fut, voice_fut, text_fut);

        let visual_segments = visual_res.unwrap_or_else(|e| {
            log::warn!("Visual analyzer failed: {}", e);
            vec![]
        });
        let audio_segments = audio_res.unwrap_or_else(|e| {
            log::warn!("Audio analyzer failed: {}", e);
            vec![]
        });
        let voice_segments = voice_res.unwrap_or_else(|e| {
            log::warn!("Voice analyzer failed: {}", e);
            vec![]
        });
        let text_segments = text_res.unwrap_or_else(|e| {
            log::warn!("Text analyzer failed: {}", e);
            vec![]
        });
        let transcript_snippets = super::arbiter::load_transcript_snippets(transcript_path);
        let arbiter = super::arbiter::ContextArbiter::new();
        let (
            fused_segments,
            dominant_emotion,
            distribution,
            scheduled_effects,
            scheduled_builtin_effects,
        ) = arbiter
            .arbitrate(
                &visual_segments,
                &audio_segments,
                &voice_segments,
                &text_segments,
                &transcript_snippets,
                config,
                progress,
            )
            .await;

        let timeline = EmotionTimeline {
            segments: fused_segments,
            dominant_emotion,
            emotion_distribution: distribution,
            visual: visual_segments,
            audio: audio_segments,
            voice: voice_segments,
            text: text_segments,
            scheduled_effects,
            scheduled_builtin_effects,
        };

        let _ = progress.send(ProgressEvent {
            stage: "fusion".into(),
            label: "Emotion Fusion Complete".into(),
            current: 100,
            total: 100,
            detail: None,
        });

        Ok(timeline)
    }

    #[allow(dead_code)]
    fn merge_timelines(
        &self,
        visual: Vec<AnalysisSegment>,
        audio: Vec<AnalysisSegment>,
        voice: Vec<AnalysisSegment>,
        text: Vec<AnalysisSegment>,
    ) -> EmotionTimeline {
        // Weighted voting: visual=0.4, audio=0.2, voice=0.3, text=0.1
        let w_visual = 0.4;
        let w_audio = 0.2;
        let w_voice = 0.3;
        let w_text = 0.1;

        let mut fused_segments = Vec::new();
        let mut distribution_scores = HashMap::new();

        // 1. Determine timeline buckets (e.g., every 1.0 second)
        let mut max_end_time = 0.0;
        let all_segments = [&visual, &audio, &voice, &text];
        for segments in all_segments.iter() {
            for seg in segments.iter() {
                if seg.end_time > max_end_time {
                    max_end_time = seg.end_time;
                }
            }
        }

        let bucket_duration = 1.0;
        let num_buckets = (max_end_time / bucket_duration).ceil() as usize;

        for b in 0..num_buckets {
            let start = b as f64 * bucket_duration;
            let end = start + bucket_duration;

            let mut votes = HashMap::new();

            // Helper to accumulate weighted score
            let mut add_vote = |seg: &AnalysisSegment, weight: f32| {
                if seg.start_time <= end && seg.end_time >= start {
                    let score = votes.entry(seg.emotion.clone()).or_insert(0.0);
                    *score += seg.score * weight;
                }
            };

            for seg in &visual {
                add_vote(seg, w_visual);
            }
            for seg in &audio {
                add_vote(seg, w_audio);
            }
            for seg in &voice {
                add_vote(seg, w_voice);
            }
            for seg in &text {
                add_vote(seg, w_text);
            }

            // Find best emotion for this bucket
            if !votes.is_empty() {
                let mut best_emotion = EmotionLabel::Neutral;
                let mut max_score = 0.0;

                for (emotion, score) in votes.iter() {
                    // Update global distribution
                    let emotion_name = format!("{:?}", emotion);
                    let dist = distribution_scores.entry(emotion_name).or_insert(0.0);
                    *dist += score;

                    if *score > max_score {
                        max_score = *score;
                        best_emotion = emotion.clone();
                    }
                }

                // Threshold filtering
                if max_score > 0.15 {
                    // Try to get bounding box if visual was present
                    let bb = visual
                        .iter()
                        .find(|v| {
                            v.start_time <= end && v.end_time >= start && v.bounding_box.is_some()
                        })
                        .and_then(|v| v.bounding_box.clone());

                    fused_segments.push(AnalysisSegment {
                        start_time: start,
                        end_time: end,
                        emotion: best_emotion,
                        score: max_score,
                        bounding_box: bb,
                    });
                }
            }
        }

        // Calculate dominant emotion
        let mut dominant_emotion = EmotionLabel::Neutral;
        let mut max_dist = 0.0;
        for (emotion_str, score) in distribution_scores.iter() {
            if *score > max_dist {
                max_dist = *score;
                // Reverse map string to enum
                dominant_emotion = match emotion_str.as_str() {
                    "Happy" => EmotionLabel::Happy,
                    "Angry" => EmotionLabel::Angry,
                    "Shock" => EmotionLabel::Shock,
                    "Fear" => EmotionLabel::Fear,
                    "Sad" => EmotionLabel::Sad,
                    "Confused" => EmotionLabel::Confused,
                    _ => EmotionLabel::Neutral,
                };
            }
        }

        // Normalize distribution to 100%
        let total_dist_score: f32 = distribution_scores.values().sum();
        if total_dist_score > 0.0 {
            for score in distribution_scores.values_mut() {
                *score = *score / total_dist_score;
            }
        }

        EmotionTimeline {
            segments: fused_segments,
            dominant_emotion,
            emotion_distribution: distribution_scores,
            visual,
            audio,
            voice,
            text,
            scheduled_effects: vec![],
            scheduled_builtin_effects: vec![],
        }
    }
}
