/**
 * Registri model ONNX / ML lokal yang digunakan oleh backend (crate::ai::onnx).
 *
 * Single source of truth untuk halaman Manajemen Model di Settings.
 * Setiap entri memakai `module` yang sama persis dengan modul analyzer Rust
 * (src-tauri/src/analysis/*.rs) agar siap di-wire ke command backend
 * (mis. `list_onnx_models`, `download_onnx_model`, `delete_onnx_model`, dll.)
 * di tugas terpisah — tanpa mengubah UI.
 */

export type OnnxModelKind =
  | "visual"
  | "voice"
  | "audio"
  | "text"
  | "text_tokenizer"
  | "face";

export interface OnnxModelDef {
  /** Key unik yang identik dengan nama modul analyzer di Rust. */
  id: OnnxModelKind;
  /** Nama file yang disimpan di AppData/models/. */
  file: string;
  /** URL sumber (HuggingFace / GitHub) untuk download. */
  url: string;
  /** Nama tampilan yang ramah pengguna. */
  displayName: string;
  /** Kategori fungsional. */
  category: string;
  /** Deskripsi singkat fungsi model. */
  description: string;
  /** Estimasi ukuran untuk tampilan (sampai backend mengembalikan ukuran nyata). */
  approxSize: string;
  /** Emosi / daftar output yang relevan (opsional, untuk meta). */
  tags?: string[];
}

export const ONNX_MODELS: OnnxModelDef[] = [
  {
    id: "visual",
    file: "emotion_vit.onnx",
    url: "https://huggingface.co/Xenova/facial_emotions_image_detection/resolve/main/onnx/model.onnx",
    displayName: "Visual Emotion (ViT)",
    category: "Visual",
    description:
      "Vision Transformer untuk mendeteksi emosi dari wajah pada frame video (224×224 RGB).",
    approxSize: "~330 MB",
    tags: ["Happy", "Sad", "Angry", "Fear", "Shock", "Neutral"],
  },
  {
    id: "voice",
    file: "wav2vec2_superb_er.onnx",
    url: "https://huggingface.co/onnx-community/wav2vec2-base-superb-er-ONNX/resolve/main/onnx/model.onnx",
    displayName: "Voice Emotion (Wav2Vec2)",
    category: "Voice",
    description:
      "Wav2Vec2 SUPERB Emotion Recognition untuk mendeteksi emosi dari suara (16 kHz).",
    approxSize: "~380 MB",
    tags: ["Neutral", "Happy", "Angry", "Sad"],
  },
  {
    id: "audio",
    file: "ast_audioset.onnx",
    url: "https://huggingface.co/onnx-community/ast-finetuned-audioset-10-10-0.4593-ONNX/resolve/main/onnx/model.onnx",
    displayName: "Audio Event (AST)",
    category: "Audio",
    description:
      "Audio Spectrogram Transformer untuk deteksi event audio (tawa, teriakan, tembakan).",
    approxSize: "~350 MB",
    tags: ["Laughter", "Crying", "Scream", "Explosion"],
  },
  {
    id: "text",
    file: "twitter_roberta_emotion.onnx",
    url: "https://huggingface.co/onnx-community/twitter-roberta-base-emotion-ONNX/resolve/main/onnx/model.onnx",
    displayName: "Text Sentiment (RoBERTa)",
    category: "Text",
    description:
      "Twitter-RoBERTa untuk mendeteksi sentimen emosi dari transkrip subtitle.",
    approxSize: "~500 MB",
    tags: ["Angry", "Joy", "Optimism", "Sad"],
  },
  {
    id: "text_tokenizer",
    file: "twitter_roberta_tokenizer.onnx",
    url: "https://huggingface.co/onnx-community/twitter-roberta-base-emotion-ONNX/resolve/main/tokenizer.json",
    displayName: "RoBERTa Tokenizer",
    category: "Text",
    description:
      "Asset tokenizer pendukung model Text Sentiment (sidecar, diunduh otomatis).",
    approxSize: "~3.6 MB",
  },
  {
    id: "face",
    file: "seeta_fd_frontal_v1.0.bin",
    url: "https://github.com/atomashpolskiy/rustface/raw/master/model/seeta_fd_frontal_v1.0.bin",
    displayName: "Face Detector (SeetaFace)",
    category: "Visual",
    description:
      "Model rusface (SeetaFace) untuk deteksi bounding box wajah pada keyframes.",
    approxSize: "~2 MB",
    tags: ["Face detection"],
  },
];

export const ONNX_MODEL_CATEGORIES = [
  "Visual",
  "Voice",
  "Audio",
  "Text",
] as const;

export type OnnxModelCategory = (typeof ONNX_MODEL_CATEGORIES)[number];