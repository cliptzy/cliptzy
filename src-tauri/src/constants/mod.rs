pub mod emotions;

pub const VALID_EMOTIONS: &[&str] = &[
    "neutral", "happy", "angry", "shock", "fear", "sad", "confused",
];

pub static EFFECTS_CATALOG: &str = include_str!("../../assets/video_effects.json");
