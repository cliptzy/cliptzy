# 📋 TODO_CORE.md — Rencana Migrasi Engine Python → Rust Native

> **Dokumen ini adalah peta jalan (roadmap) lengkap** untuk migrasi seluruh logic dari `src-tauri/engine/core/` (Python)
> ke backend Rust native di `src-tauri/src/`. Setiap fase dirancang agar bisa di-deliver secara inkremental
> tanpa mengganggu fitur yang sudah berjalan.

---

## 📑 DAFTAR ISI

- [Bagian 0: Status Migrasi Saat Ini](#bagian-0-status-migrasi-saat-ini)
- [Bagian 1: Arsitektur Target & Struktur Folder](#bagian-1-arsitektur-target--struktur-folder)
- [Bagian 2: Design Patterns & Prinsip Arsitektur](#bagian-2-design-patterns--prinsip-arsitektur)
- [Bagian 3: Phase 0 — Foundation & Shared Infrastructure](#bagian-3-phase-0--foundation--shared-infrastructure)
- [Bagian 4: Phase 1 — Video Acquisition & Analysis Pipeline](#bagian-4-phase-1--video-acquisition--analysis-pipeline)
- [Bagian 5: Phase 2 — Video Processing Core (FFmpeg Pipeline)](#bagian-5-phase-2--video-processing-core-ffmpeg-pipeline)
- [Bagian 6: Phase 3 — Audio & Transcription Pipeline](#bagian-6-phase-3--audio--transcription-pipeline)
- [Bagian 7: Phase 4 — AI Integration Layer](#bagian-7-phase-4--ai-integration-layer)
- [Bagian 8: Phase 5 — Uploaders & Platform Abstraction](#bagian-8-phase-5--uploaders--platform-abstraction)
- [Bagian 9: Phase 6 — Orchestration & Use Cases](#bagian-9-phase-6--orchestration--use-cases)
- [Bagian 10: Phase 7 — Polish, TTS, & Compilation](#bagian-10-phase-7--polish-tts--compilation)
- [Bagian 11: Strategi Bundle & Distribusi Binary](#bagian-11-strategi-bundle--distribusi-binary)
- [Bagian 12: Prioritas & Urutan Eksekusi](#bagian-12-prioritas--urutan-eksekusi)
- [Bagian 13: Keputusan Teknis Kunci (ADR)](#bagian-13-keputusan-teknis-kunci-adr)

---

## Bagian 0: Status Migrasi Saat Ini

### ✅ Sudah Selesai di Rust

| Modul Python | File Rust | Catatan |
|---|---|---|
| `supabase_sync.py` | `supabase.rs` | Google OAuth PKCE, session, config sync, storage |
| `config.py` (save/load) | `commands.rs` | `save_config_file`, `copy_asset_file` |
| `youtube.py` (metadata) | `video/youtube.rs` | Heatmap analysis, video info, download via `yt-dlp` crate |
| `utils.py` (paths) | `paths.rs` | `app_data_dir()` |
| Cookies management | `commands.rs` | `copy_cookies_file`, `validate_cookies_file` |
| System monitor | `monitor.rs` | CPU & RAM via `sysinfo` |
| `security.py` | Tidak perlu | Obfuscation env vars sudah via `dotenvy` di `build.rs` |

### ❌ Belum Dimigrasi (Scope Dokumen Ini)

| Modul Python | Prioritas | Kompleksitas |
|---|---|---|
| `processor.py` (crop, burn subtitle, stack) | 🔴 Kritis | Sangat Tinggi |
| `processing/cropper.py` (9 crop modes) | 🔴 Kritis | Tinggi |
| `processing/subtitle.py` (burn + VFX overlay) | 🔴 Kritis | Tinggi |
| `processing/stacker.py` (intro/outro/watermark) | 🟡 Penting | Sedang |
| `subtitle.py` (Whisper transcription + ASS) | 🟡 Penting | Tinggi |
| `face_tracker.py` (YuNet + keyframes) | 🟡 Penting | Tinggi |
| `ai/detector.py` (highlight + metadata) | 🟡 Penting | Sedang |
| `ai/providers (ollama, gemini, openai)` | 🟡 Penting | Rendah |
| `uploaders/* (YouTube, TikTok, Instagram)` | 🟠 Nanti | Sedang |
| `processing/emotion_analyzer.py` (DeepFace) | 🔵 Opsional | Sangat Tinggi |
| `processing/audio_analyzer.py` (AST model) | 🔵 Opsional | Sangat Tinggi |
| `processing/voice_analyzer.py` (Wav2Vec2) | 🔵 Opsional | Tinggi |
| `processing/text_analyzer.py` (Roberta) | 🔵 Opsional | Tinggi |
| `processing/tts_engine.py` (Kokoro TTS) | 🔵 Opsional | Tinggi |
| `processing/brainrot_processor.py` | 🔵 Opsional | Sedang |
| `channel_manager.py` | 🟢 Rendah | Rendah |
| `dependency_manager.py` | 🟢 Rendah | Rendah |
| `video_effects.py` | 🟡 Penting | Rendah |

---

## Bagian 1: Arsitektur Target & Struktur Folder

### 1.1 Prinsip Penataan Folder

```
Prinsip: "Screaming Architecture" — struktur folder HARUS menjelaskan DOMAIN bisnis,
bukan detail teknis. Setiap folder adalah bounded context yang bisa berkembang independen.
```

### 1.2 Struktur Folder Rust yang Direkomendasikan

```
src-tauri/src/
├── main.rs                          # Binary entrypoint
├── lib.rs                           # Tauri app builder, plugin init, command registry
│
├── commands/                        # 🎯 Tauri IPC command handlers (thin layer)
│   ├── mod.rs                       #    Re-exports semua command modules
│   ├── system.rs                    #    get_system_metrics, check_dependencies
│   ├── config.rs                    #    save_config, load_config, copy_asset
│   ├── auth.rs                      #    login, logout, get_user_info
│   ├── sync.rs                      #    sync_config_up/down, upload/download_file
│   ├── cookies.rs                   #    copy_cookies, validate_cookies
│   ├── video.rs                     #    analyze_video, scan_segments, clip_video
│   └── upload.rs                    #    upload_to_youtube, upload_to_tiktok, etc.
│
├── config/                          # ⚙️ Configuration management
│   ├── mod.rs                       #    AppConfig struct utama
│   ├── models.rs                    #    SubtitleConfig, AIConfig, CompilationConfig, dll.
│   └── defaults.rs                  #    Default values & preset ratios
│
├── video/                           # 🎬 Video acquisition & metadata
│   ├── mod.rs
│   ├── youtube.rs                   #    yt-dlp: metadata, heatmap, download (SUDAH ADA)
│   ├── downloader.rs                #    Download manager dengan progress & cancellation
│   └── local.rs                     #    Local file probe (ffprobe wrapper)
│
├── processing/                      # 🔧 Video/Audio processing pipeline (CORE)
│   ├── mod.rs
│   ├── ffmpeg/                      #    FFmpeg abstraction layer
│   │   ├── mod.rs
│   │   ├── runner.rs                #    Command builder & subprocess executor
│   │   ├── filters.rs               #    Filter graph builder (scale, crop, overlay, etc.)
│   │   ├── probe.rs                 #    ffprobe wrapper (duration, codec, resolution)
│   │   └── hwaccel.rs               #    Hardware acceleration detection & codec selection
│   │
│   ├── cropper.rs                   #    9 crop modes (center, face, split, full, etc.)
│   ├── subtitle_burner.rs           #    Burn ASS subtitle + chromakey VFX overlay
│   ├── stacker.rs                   #    Intro/outro concat + watermark overlay
│   ├── thumbnail.rs                 #    Frame extraction + meme overlay + collage grid
│   └── effects.rs                   #    Video effects catalog & asset manager
│
├── transcription/                   # 🎙️ Speech-to-text
│   ├── mod.rs
│   ├── whisper.rs                   #    Whisper binding (whisper-rs atau CLI wrapper)
│   ├── ass_writer.rs                #    ASS subtitle file generator (karaoke, brutalist)
│   └── models.rs                    #    WordTimestamp, TranscriptSegment, dll.
│
├── face/                            # 👤 Face detection & tracking
│   ├── mod.rs
│   ├── detector.rs                  #    YuNet ONNX face detection
│   ├── tracker.rs                   #    Keyframe extraction + jitter filtering
│   └── models.rs                    #    FaceKeyframe, NormalizedCenter
│
├── ai/                              # 🧠 AI provider abstraction
│   ├── mod.rs
│   ├── provider.rs                  #    AIProvider trait + factory
│   ├── ollama.rs                    #    Ollama REST client
│   ├── gemini.rs                    #    Google Gemini REST client
│   ├── openai.rs                    #    OpenAI-compatible REST client
│   ├── detector.rs                  #    Highlight detection from transcript
│   ├── metadata.rs                  #    Generate title, tags, hook, emotion enrichment
│   └── prompts.rs                   #    Prompt templates (centralized, easy to iterate)
│
├── analysis/                        # 📊 Emotion & audio analysis (OPSIONAL, Phase Lanjut)
│   ├── mod.rs
│   ├── emotion.rs                   #    Facial emotion (via ONNX Runtime atau CLI)
│   ├── audio.rs                     #    Audio event detection (scream, laugh, etc.)
│   ├── voice.rs                     #    Voice emotion SER
│   └── text.rs                      #    Text sentiment classification
│
├── tts/                             # 🗣️ Text-to-Speech
│   ├── mod.rs
│   ├── engine.rs                    #    TTS trait + Kokoro/gTTS/edge-tts wrapper
│   └── voice_clone.rs               #    Voice conversion (Kanade, opsional)
│
├── uploaders/                       # 📤 Social media upload abstraction
│   ├── mod.rs
│   ├── traits.rs                    #    Uploader trait, UploadResult, UploadMetadata
│   ├── youtube.rs                   #    YouTube Data API v3
│   ├── tiktok.rs                    #    TikTok uploader
│   ├── instagram.rs                 #    Instagram Reels uploader
│   └── facebook.rs                  #    🔮 Facebook Reels (future-ready placeholder)
│
├── orchestrator/                    # 🎼 Use cases & workflow coordination
│   ├── mod.rs
│   ├── scan.rs                      #    ScanVideoUseCase (heatmap/AI/sequential)
│   ├── clip.rs                      #    ClipVideoUseCase (Phase 1: download + crop)
│   ├── render.rs                    #    RenderClipUseCase (Phase 2: effects + subtitle)
│   ├── compile.rs                   #    CompileVideoUseCase (Top N compilation)
│   ├── batch_upload.rs              #    BatchUploadUseCase
│   └── pipeline.rs                  #    Pipeline trait & shared cancellation/progress
│
├── channels/                        # 📺 Channel management (kurasi kreator)
│   ├── mod.rs
│   └── manager.rs                   #    CRUD channel catalog + video filtering
│
├── deps/                            # 📦 Dependency & binary manager
│   ├── mod.rs
│   └── manager.rs                   #    Download & verify FFmpeg/yt-dlp binaries
│
├── error.rs                         # ❌ Unified error types (thiserror)
├── paths.rs                         # 📁 Centralized path resolution (SUDAH ADA)
├── monitor.rs                       # 📈 System metrics (SUDAH ADA)
├── supabase.rs                      # 🔐 Auth & cloud sync (SUDAH ADA)
│
└── constants/                       # 📎 Shared constants
    ├── mod.rs
    ├── emotions.rs                  #    VALID_EMOTIONS, EMOTION_DESCRIPTIONS
    └── effects.json                 #    Video effects catalog (embed via include_str!)
```

### 1.3 Mengapa Struktur Ini?

| Keputusan | Alasan |
|---|---|
| `commands/` terpisah dari logic | Commands hanyalah **thin adapter** yang memanggil orchestrator/service. Memudahkan testing tanpa Tauri context. |
| `processing/ffmpeg/` sebagai sub-modul | FFmpeg adalah dependensi terbesar. Abstraksi di sini memungkinkan **swap ke library FFI** di masa depan tanpa mengubah consumer code. |
| `uploaders/` dengan trait | Menambah platform baru (Facebook, X/Twitter) cukup implementasi trait tanpa menyentuh orchestrator. |
| `orchestrator/` terpisah dari processing | Clean Architecture: use case **mengorkestrasi** modul processing, bukan sebaliknya. |
| `analysis/` terpisah & opsional | Module ML berat ini bisa di-feature-gate (`#[cfg(feature = "analysis")]`) sehingga bundle ringan tanpa AI. |
| `constants/` dengan `include_str!` | Embed JSON statis ke binary → zero runtime file I/O, satu binary portable. |

---

## Bagian 2: Design Patterns & Prinsip Arsitektur

### 2.1 Pola Desain Utama

#### A. **Pipeline Pattern** (untuk Video Processing)

```
                                     ┌─────────────────────────┐
                                     │    PipelineContext       │
                                     │  - job_dir: PathBuf     │
                                     │  - config: AppConfig    │
                                     │  - cancel: CancelToken  │
                                     │  - progress: ProgressTx │
                                     └────────────┬────────────┘
                                                  │
          ┌───────────┬───────────┬───────────┬───┴───┬───────────┬──────────┐
          ▼           ▼           ▼           ▼       ▼           ▼          ▼
     ┌─────────┐ ┌─────────┐ ┌────────┐ ┌────────┐ ┌──────┐ ┌────────┐ ┌──────┐
     │Download │ │  Crop   │ │Transcr.│ │AI Meta │ │ ASS  │ │ Burn   │ │Stack │
     │ Stage   │ │ Stage   │ │ Stage  │ │ Stage  │ │Writer│ │Subtitle│ │Concat│
     └─────────┘ └─────────┘ └────────┘ └────────┘ └──────┘ └────────┘ └──────┘
```

```rust
// Setiap stage adalah unit independen yang bisa di-skip, retry, atau paralelkan
#[async_trait]
pub trait PipelineStage: Send + Sync {
    fn name(&self) -> &str;
    async fn execute(&self, ctx: &mut PipelineContext) -> Result<(), PipelineError>;
    fn can_skip(&self, ctx: &PipelineContext) -> bool { false }
}
```

**Mengapa Pipeline?**
- Python `process_single_clip()` adalah **God Function** 600+ baris. Pipeline memecahnya menjadi stage atomik.
- Setiap stage bisa di-test independen.
- Stage bisa di-reorder atau di-skip berdasarkan config (e.g., skip transcription jika `use_subtitle = false`).
- Natural fit untuk progress reporting (`stage 3/7: Transcribing...`).

#### B. **Strategy Pattern** (untuk Crop Mode & AI Provider)

```rust
pub trait CropStrategy: Send + Sync {
    fn build_ffmpeg_filter(
        &self,
        input: &VideoInfo,
        output: &OutputConfig,
        face_data: Option<&FaceData>,
    ) -> Result<FilterGraph, ProcessingError>;
}

// 9 implementasi: DefaultCrop, CenterFaceCrop, SplitLeftCrop, SplitRightCrop,
//                 SplitFaceCrop, FullCrop, FullFaceCrop, MultiFaceCrop, SplitBrollCrop
```

**Mengapa Strategy?**
- Python `build_crop_command()` adalah switch-case 400+ baris. Strategy memecahnya menjadi struct terpisah.
- Menambah crop mode baru = tambah struct baru, **zero perubahan** di existing code.

#### C. **Abstract Factory** (untuk Uploaders)

```rust
pub trait Uploader: Send + Sync {
    fn platform_name(&self) -> &str;
    async fn upload(
        &self,
        file_path: &Path,
        metadata: &UploadMetadata,
        progress: &ProgressTx,
    ) -> Result<UploadResult, UploadError>;
}

pub fn create_uploader(platform: &str, config: &AppConfig) -> Box<dyn Uploader> {
    match platform {
        "youtube"   => Box::new(YouTubeUploader::new(config)),
        "tiktok"    => Box::new(TikTokUploader::new(config)),
        "instagram" => Box::new(InstagramUploader::new(config)),
        "facebook"  => Box::new(FacebookUploader::new(config)),  // Future
        _           => Box::new(DummyUploader),
    }
}
```

#### D. **Observer/Event Pattern** (untuk Progress & Cancellation)

```rust
use tokio::sync::{broadcast, watch};

/// Cancellation token — shared across all stages
pub type CancelToken = tokio_util::sync::CancellationToken;

/// Progress channel — dari processing stages ke frontend
#[derive(Serialize, Clone, Debug)]
pub struct ProgressEvent {
    pub stage: String,
    pub label: String,
    pub current: u32,
    pub total: u32,
}

pub type ProgressTx = broadcast::Sender<ProgressEvent>;
```

**Frontend** menerima progress via `tauri::Emitter::emit()` dan `listen()` di Vue.

```rust
// Di dalam Tauri command:
app_handle.emit("clip-progress", &progress_event)?;
```

### 2.2 Error Handling Strategy

```rust
// error.rs — gunakan `thiserror` untuk error types yang terstruktur
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliptzyError {
    #[error("Video download gagal: {0}")]
    Download(String),

    #[error("FFmpeg error (exit code {code}): {message}")]
    FFmpeg { code: i32, message: String },

    #[error("Transcription error: {0}")]
    Transcription(String),

    #[error("AI provider error: {0}")]
    AIProvider(String),

    #[error("Upload error ({platform}): {message}")]
    Upload { platform: String, message: String },

    #[error("Config error: {0}")]
    Config(String),

    #[error("Operasi dibatalkan oleh pengguna")]
    Cancelled,

    #[error("File tidak ditemukan: {0}")]
    FileNotFound(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

// Untuk Tauri commands, convert ke String:
impl From<CliptzyError> for String {
    fn from(e: CliptzyError) -> String {
        e.to_string()
    }
}
```

### 2.3 Concurrency Model

| Aspek | Python (Lama) | Rust (Baru) |
|---|---|---|
| Threading | `ThreadPoolExecutor` + GIL bottleneck | `tokio` async + `tokio::task::spawn_blocking` untuk FFmpeg subprocess |
| Cancellation | `is_cancelled` flag polling | `CancellationToken` dari `tokio-util` — propagasi otomatis ke child tasks |
| Progress | `event_hook` callback | `broadcast::Sender<ProgressEvent>` + Tauri `emit()` |
| Subprocess | `subprocess.Popen` + line reader | `tokio::process::Command` + `BufReader` async line streaming |

---

## Bagian 3: Phase 0 — Foundation & Shared Infrastructure

> **Tujuan:** Bangun fondasi yang dipakai semua modul lain.

### 3.0.1 Unified Error Types

- [x] Buat `src/error.rs` dengan `CliptzyError` enum (lihat §2.2)
- [x] Tambah `thiserror` ke `Cargo.toml`
- [x] Refactor semua `Result<T, String>` di `commands.rs` → `Result<T, CliptzyError>`
- [x] Implement `From<CliptzyError> for String` agar kompatibel dengan Tauri

### 3.0.2 Configuration Models (Strongly-Typed)

- [x] Buat `src/config/mod.rs` dengan `AppConfig` struct
- [x] Buat `src/config/models.rs`:
  ```rust
  #[derive(Serialize, Deserialize, Clone, Debug)]
  pub struct SubtitleConfig {
      pub whisper_model: String,        // "small", "medium", "large-v3"
      pub font: String,
      pub fonts_dir: String,
      pub location: String,             // "bottom", "center", "top"
      pub delay: f64,
      pub font_size: u32,
      pub color: String,                // "&H00FFFFFF"
      pub bg_color: String,             // "&H80000000"
      pub border_style: u32,            // 1 atau 4
      pub animation: String,            // "hormozi", "scale", "none"
      pub style: String,                // "plain", "full_color"
      pub max_words: u32,
  }

  #[derive(Serialize, Deserialize, Clone, Debug)]
  pub struct AIConfig {
      pub provider: String,             // "ollama" | "gemini" | "openai"
      pub ollama_host: String,
      pub ollama_model: String,
      pub gemini_key: String,
      pub gemini_model: String,
      pub openai_key: String,
      pub openai_model: String,
      pub openai_base_url: String,
      pub use_highlight: bool,
      pub use_generate_intro: bool,
      // ... (sesuaikan dengan config.json Python)
  }

  #[derive(Serialize, Deserialize, Clone, Debug)]
  pub struct PlatformConfig {
      pub session: String,
      pub auto_upload: bool,
      pub visibility: String,           // "public", "private", "unlisted"
      // platform-specific fields...
  }

  #[derive(Serialize, Deserialize, Clone, Debug)]
  pub struct CompilationConfig {
      pub ordering: String,             // "countdown" | "countup"
      pub numbering_duration: f64,
      pub use_tts: bool,
      pub tts_template: String,
      pub use_subtitle: bool,
      pub crop_mode: String,
  }
  ```
- [x] Buat `src/config/defaults.rs` dengan implementasi `Default` untuk setiap struct
- [x] Migrasi `save_config_file` dan `load_config` dari `commands.rs` ke `config/mod.rs`

### 3.0.3 Progress & Cancellation Infrastructure

- [x] Tambah `tokio-util` ke `Cargo.toml` (untuk `CancellationToken`)
- [x] Buat progress event types di `src/orchestrator/pipeline.rs`:
  ```rust
  #[derive(Serialize, Clone, Debug)]
  pub struct ProgressEvent {
      pub stage: String,
      pub label: String,
      pub current: u32,
      pub total: u32,
      pub detail: Option<String>,
  }
  ```
- [x] Implement emit helper yang mengirim ke Tauri window:
  ```rust
  pub fn emit_progress(handle: &tauri::AppHandle, event: &ProgressEvent) {
      let _ = handle.emit("clip-progress", event);
  }
  ```

### 3.0.4 Constants & Embedded Assets

- [x] Buat `src/constants/mod.rs`:
  ```rust
  pub const VALID_EMOTIONS: &[&str] = &[
      "neutral", "happy", "angry", "shock", "fear", "sad", "confused"
  ];

  pub static EFFECTS_CATALOG: &str = include_str!("../../assets/video_effects.json");
  ```
- [x] Copy `core/constant/video_effects.json` → `src-tauri/assets/video_effects.json`
- [x] Embed via `include_str!` — zero file I/O at runtime

### 3.0.5 Refactor commands.rs → commands/

- [x] Split `commands.rs` monolith menjadi per-domain modules:
  - `commands/system.rs` → `get_system_metrics`
  - `commands/config.rs` → `save_config_file`, `copy_asset_file`
  - `commands/cookies.rs` → `copy_cookies_file`, `validate_cookies_file`
  - `commands/auth.rs` → `login_with_google`, `logout`, `get_user_id`, `get_user_info`
  - `commands/sync.rs` → `sync_config_up`, `sync_config_down`, `upload_file`, `download_file`
  - `commands/video.rs` → `analyze_video` + commands baru nanti
- [x] Update `lib.rs` imports

---

## Bagian 4: Phase 1 — Video Acquisition & Analysis Pipeline

> **Tujuan:** Lengkapi kemampuan download & analisis video.
> **Mapping Python:** `youtube.py`, `use_cases/scan_video.py`, `use_cases/preview_clip.py`

### 4.1 FFmpeg Probe Wrapper

- [x] Buat `src/processing/ffmpeg/probe.rs`: *(Catatan: Diubah kebijakannya, sekarang langsung menggunakan crate `rust_ffprobe` dari ekosistem)*
  ```rust
  pub struct VideoProbeResult {
      pub duration: f64,
      pub width: u32,
      pub height: u32,
      pub has_audio: bool,
      pub codec: String,
      pub fps: f64,
  }

  pub async fn probe_video(path: &Path) -> Result<VideoProbeResult, CliptzyError> {
      // Jalankan: ffprobe -v quiet -print_format json -show_format -show_streams <path>
      // Parse JSON output
  }
  ```
- [x] Ini dipakai oleh hampir semua modul lain → **harus selesai pertama**

### 4.2 FFmpeg Command Runner

- [x] Buat `src/processing/ffmpeg/runner.rs`: *(Catatan: Diubah kebijakannya, sekarang langsung menggunakan crate `rust_ffmpeg` dari ekosistem)*
  ```rust
  pub struct FFmpegCommand {
      args: Vec<String>,
      cancel_token: CancelToken,
  }

  impl FFmpegCommand {
      pub fn new() -> Self;
      pub fn arg(mut self, arg: &str) -> Self;
      pub fn args(mut self, args: &[&str]) -> Self;
      pub fn input(mut self, path: &Path) -> Self;
      pub fn output(mut self, path: &Path) -> Self;
      pub fn filter_complex(mut self, filter: &str) -> Self;
      pub fn with_cancel(mut self, token: CancelToken) -> Self;

      /// Jalankan FFmpeg, stream stderr line-by-line, emit progress
      pub async fn execute(
          &self,
          progress: Option<&ProgressTx>,
      ) -> Result<(), CliptzyError>;
  }
  ```
- [x] Implementasi `execute()`:
  - Spawn `tokio::process::Command`
  - Baca stderr async via `BufReader::new(child.stderr)`
  - Parse progress (`frame=`, `time=`, `speed=`) untuk emit ke frontend
  - Check `cancel_token.is_cancelled()` setiap iterasi → `child.kill()` jika cancelled
  - Return `CliptzyError::FFmpeg` jika exit code != 0

### 4.3 Hardware Acceleration Detection

- [x] Buat `src/processing/ffmpeg/hwaccel.rs`:
  ```rust
  pub enum HwAccel {
      VideoToolbox,  // macOS
      Nvenc,         // NVIDIA
      Amf,           // AMD
      Qsv,           // Intel
      Cpu,           // Fallback
  }

  impl HwAccel {
      pub fn detect(config_override: Option<&str>) -> Self;
      pub fn encoder(&self) -> &str;       // "h264_videotoolbox", "libx264", etc.
      pub fn encode_args(&self) -> Vec<String>;
  }
  ```
- [x] Port logic dari Python `processing/utils.py → get_video_codec_args()`

### 4.4 Video Downloader dengan Progress

- [x] Buat `src/video/downloader.rs`:
  - Wrapper di atas `yt-dlp` crate yang sudah ada
  - Tambah support download **range** (`--download-sections *start-end`)
  - Progress callback → emit ke Tauri
  - Cancellation support
  - Concurrent fragment download (`concurrent_fragment_downloads = 16`)

### 4.5 Local Video Handler

- [x] Buat `src/video/local.rs`:
  - `probe_local_video(path)` → gunakan `ffprobe`
  - `cut_local_segment(path, start, end, output)` → FFmpeg `-ss -to -c copy`

### 4.6 Scan Video Use Case

- [x] Buat `src/orchestrator/scan.rs`:
  - Port `ScanVideoUseCase` dari Python
  - Cache segments ke `clips/<video_id>/segments.json`
  - Support local video (langsung probe duration, generate sequential segments)
  - Tauri command: `scan_video(url) -> ScanResult`

### 4.7 Preview Video Use Case

- [x] Extend `src/video/youtube.rs` atau buat terpisah:
  - Ekstrak: title, thumbnail, uploader, duration, language
  - In-memory cache (`dashmap` atau `HashMap` + `Mutex`)
  - Tauri command: `preview_video(url) -> PreviewResult`

---

## Bagian 5: Phase 2 — Video Processing Core (FFmpeg Pipeline)

> **Tujuan:** Implementasi semua crop mode dan video manipulation.
> **Mapping Python:** `processing/cropper.py`, `ffmpeg.py`
> **Ini adalah JANTUNG aplikasi.**

### 5.1 FFmpeg Filter Graph Builder

- [x] Buat `src/processing/ffmpeg/filters.rs`:
  ```rust
  pub struct FilterGraph {
      inputs: Vec<String>,
      filters: Vec<FilterNode>,
      outputs: Vec<String>,
  }

  pub struct FilterNode {
      filter_name: String,     // "scale", "crop", "overlay", "chromakey", etc.
      params: Vec<(String, String)>,
      input_pads: Vec<String>,
      output_pads: Vec<String>,
  }

  impl FilterGraph {
      pub fn new() -> Self;
      pub fn add_input(&mut self, label: &str) -> &mut Self;
      pub fn scale(w: &str, h: &str) -> FilterNode;
      pub fn crop(w: &str, h: &str, x: &str, y: &str) -> FilterNode;
      pub fn overlay(x: &str, y: &str) -> FilterNode;
      pub fn concat(segments: u32, has_video: bool, has_audio: bool) -> FilterNode;
      pub fn chromakey(color: &str, similarity: f32) -> FilterNode;
      pub fn subtitles(path: &str, force_style: &str) -> FilterNode;
      pub fn loudnorm(i: f32, lra: f32, tp: f32) -> FilterNode;

      /// Render ke string `-filter_complex "..."`
      pub fn to_string(&self) -> String;
  }
  ```
- [x] Ini meng-**abstract** pembuatan filter FFmpeg yang sebelumnya berupa string concatenation di Python

### 5.2 Crop Strategies (9 Mode)

- [x] Buat `src/processing/cropper.rs` dengan trait `CropStrategy`:
  ```rust
  pub trait CropStrategy: Send + Sync {
      fn name(&self) -> &str;
      fn build_command(
          &self,
          input: &Path,
          output: &Path,
          video_info: &VideoProbeResult,
          output_config: &OutputConfig,
          face_data: Option<&FaceData>,
      ) -> Result<FFmpegCommand, CliptzyError>;
  }
  ```

- [x] Implementasi per crop mode (dalam urutan prioritas): *(Catatan: Semua mode utama telah diimplementasikan dalam bentuk strategi modular di folder `cropper/`)*

  1. **`DefaultCrop`** — Scale to cover + center crop
     - [x] Paling simpel, test pertama kali
     - [x] Port dari: `build_cover_scale_crop_vf()`

  2. **`FullCrop`** — Letterbox gameplay + blurred background padding
     - [x] Port dari: `crop_mode == "full"` di Python

  3. **`SplitLeftCrop` / `SplitRightCrop`** — Top center + bottom left/right
     - [ ] Port dari: `crop_mode == "split_left"` / `"split_right"`

  4. **`CenterFaceCrop`** - Dynamic keyframe face tracking crop
     - [x] **Paling kompleks:** Membutuhkan face detection (Phase terpisah)
     - [x] Port dynamic FFmpeg expression builder (`if(lt(t,...), ...)`)
     - [x] Keyframe simplification (max 85 terms untuk AST limit FFmpeg)

  5. **`SplitFaceCrop`** — Top center + bottom dynamic face
     - [x] Kombinasi split + face tracking

  6. **`FullFaceCrop`** — Top gameplay + bottom face + blurred bg
     - [x] Paling advanced split mode

  7. **`MultiFaceCrop`** — Podcast layout (2 faces + full)
     - [x] Butuh `get_two_faces_normalized_centers()` (diimplementasikan di `tracker.rs`)

  8. **`SplitBrollCrop`** — Top main + bottom random B-roll
     - [x] Butuh asset manager untuk B-roll files (diimplementasikan di `broll_manager.rs`)

  9. **`PassthroughCrop`** — No crop
     - [x] Mempertahankan resolusi asli video tanpa crop.

- [x] Factory function: *(Catatan: Sudah diperbarui untuk mendukung seluruh mode secara dinamis)*
  ```rust
  pub fn create_crop_strategy(mode: &str) -> Box<dyn CropStrategy> {
      match mode {
          "none" => Box::new(PassthroughCrop),
          "default" => Box::new(DefaultCrop),
          "center_face" => Box::new(CenterFaceCrop),
          "split_face" => Box::new(SplitFaceCrop),
          "multi_face" => Box::new(MultiFaceCrop),
          "split_broll" => Box::new(SplitBrollCrop),
          "full" => Box::new(FullCrop),
          "full_face" => Box::new(FullFaceCrop),
          _ => Box::new(DefaultCrop),
      }
  }
  ```

### 5.3 Subtitle Burner & VFX Overlay

- [x] Buat `src/processing/subtitle_burner.rs`:
  - Port `burn_subtitle_and_highlight()` dari Python
  - Fungsi utama:
    - Inject 3-second hook text ke ASS
    - Scan enriched transcript untuk scheduled VFX (green-screen overlay)
    - Build `filter_complex` dengan `subtitles`, `chromakey`, `overlay`, `adelay`, `amix`
    - Apply `loudnorm` audio normalization
  - **Catatan:** Ini adalah filter FFmpeg paling kompleks di seluruh app

### 5.4 Stacker (Intro/Outro/Watermark)

- [x] Buat `src/processing/stacker.rs`:
  - Port `stack_and_concat()` dari Python
  - Concat via FFmpeg concat demuxer (`-f concat -safe 0`)
  - Overlay watermark PNG
  - Handle kasus: ada/tidak ada intro, ada/tidak ada outro
  - Sequential mode: tambah "Lanjut Part Berikutnya" card

### 5.5 Thumbnail Generator

- [x] Buat `src/processing/thumbnail.rs`:
  - `generate_thumbnail()`: Extract frame + overlay meme VFX
  - `generate_compilation_thumbnail()`: 2x2 grid collage dari multiple clips
  - Port dari Python `processing/thumbnail.py`

### 5.6 Video Effects Manager

- [x] Buat `src/processing/effects.rs`:
  - Load catalog dari embedded `include_str!("effects.json")`
  - `get_effect(emotion, exclude)` → random matching effect
  - `get_effect_by_name(name)` → lookup by name
  - Asset path resolution ke `assets/video_effects/`

---

## Bagian 6: Phase 3 — Audio & Transcription Pipeline

> **Tujuan:** Speech-to-text dengan Whisper untuk subtitle otomatis.
> **Mapping Python:** `subtitle.py`

### 6.1 Pendekatan Whisper: CLI vs Binding

**Rekomendasi: Gunakan `whisper-rs` (binding ke `whisper.cpp`)**

| Aspek | `whisper-rs` (C++ binding) | CLI (`whisper-cli`) |
|---|---|---|
| Bundle size | +5-10MB (library) | +50-100MB (standalone binary) |
| Latency | Rendah (in-process) | Tinggi (spawn process + load model) |
| Word timestamps | ✅ Didukung | ✅ Didukung |
| GPU support | CUDA, Metal, Vulkan | Tergantung build |
| Maintenance | Aktif, ikut whisper.cpp | Manual binary management |
| **Rekomendasi** | ✅ **Pilihan utama** | Fallback jika build gagal |

### 6.2 Implementasi Whisper

- [x] Tambah `whisper-rs` ke `Cargo.toml`:
  ```toml
  [dependencies]
  whisper-rs = { version = "0.13", features = ["metal"] }  # macOS Metal
  # Untuk cross-platform: features = ["cuda"] di build CI Linux/Windows
  ```
- [x] Buat `src/transcription/whisper.rs`:
  ```rust
  pub struct WhisperTranscriber {
      model: WhisperContext,
  }

  impl WhisperTranscriber {
      pub fn new(model_path: &Path) -> Result<Self, CliptzyError>;

      /// Transcribe audio file, return word-level timestamps
      pub fn transcribe(
          &self,
          audio_path: &Path,
          language: Option<&str>,
          cancel: &CancelToken,
          progress: &ProgressTx,
      ) -> Result<Vec<TranscriptSegment>, CliptzyError>;
  }

  #[derive(Serialize, Clone, Debug)]
  pub struct TranscriptSegment {
      pub start: f64,
      pub end: f64,
      pub text: String,
      pub words: Vec<WordTimestamp>,
  }

  #[derive(Serialize, Clone, Debug)]
  pub struct WordTimestamp {
      pub word: String,
      pub start: f64,
      pub end: f64,
  }
  ```
- [x] Model management:
  - Download model on first use ke `<app_data>/models/ggml-<size>.bin`
  - Progress reporting saat download
  - Support sizes: tiny, small, medium, large-v3

### 6.3 ASS Subtitle Writer

- [x] Buat `src/transcription/ass_writer.rs`:
  - Port `write_enriched_ass_file()` dari Python
  - Support styles:
    - `plain`: Kata per kata, satu warna
    - `full_color`: Karaoke, warna per emosi
  - Support animations:
    - `hormozi`: Pop-in scale effect (`\fscx\fscy\t(...)`)
    - `scale`: Smooth scale
    - `none`: Statis
  - `format_ass_time(seconds) -> String` (format `H:MM:SS.cs`)
  - Handle subtitle delay offset

### 6.4 Audio Extraction

- [x] Implementasi di `src/processing/ffmpeg/runner.rs` atau helper terpisah:
  ```rust
  /// Extract audio dari video untuk transcription
  pub async fn extract_audio(
      video_path: &Path,
      output_path: &Path,
      sample_rate: u32,  // 16000 untuk Whisper
  ) -> Result<(), CliptzyError> {
      // ffmpeg -i input -vn -acodec pcm_s16le -ar 16000 -ac 1 output.wav
  }
  ```

---

## Bagian 7: Phase 4 — AI Integration Layer

> **Tujuan:** Port AI providers dan highlight detection.
> **Mapping Python:** `ai/detector.py`, `ai/ollama_provider.py`, `ai/gemini_provider.py`, `ai/openai_provider.py`

### 7.1 AI Provider Trait

- [x] Buat `src/ai/provider.rs`:
  ```rust
  #[async_trait]
  pub trait AIProvider: Send + Sync {
      fn name(&self) -> &str;

      /// Generate text response dari prompt
      async fn generate(
          &self,
          prompt: &str,
          progress: Option<&ProgressTx>,
      ) -> Result<String, CliptzyError>;
  }
  ```

### 7.2 Implementasi Providers

- [x] **`src/ai/ollama.rs`** — Ollama REST client:
  - `POST {host}/api/generate` dengan streaming
  - `reqwest` sudah ada di dependencies
  - Params: `temperature: 0.3`, `num_predict: 8192`, `num_ctx: 16384`

- [x] **`src/ai/gemini.rs`** — Google Gemini REST client:
  - SSE streaming via `https://generativelanguage.googleapis.com/v1beta/models/{model}:streamGenerateContent`
  - JSON response format
  - Fallback non-streaming

- [x] **`src/ai/openai.rs`** — OpenAI-compatible REST client:
  - `POST {base_url}/v1/chat/completions` dengan streaming
  - Compatible dengan: OpenAI, Groq, OpenRouter, LM Studio, vLLM
  - `response_format: { type: "json_object" }`

- [x] **Factory function** di `src/ai/mod.rs`:
  ```rust
  pub fn create_provider(config: &AIConfig) -> Box<dyn AIProvider> {
      match config.provider.as_str() {
          "ollama" => Box::new(OllamaProvider::new(&config.ollama_host, &config.ollama_model)),
          "gemini" => Box::new(GeminiProvider::new(&config.gemini_key, &config.gemini_model)),
          "openai" => Box::new(OpenAIProvider::new(
              &config.openai_key,
              &config.openai_model,
              &config.openai_base_url,
          )),
          _ => Box::new(GeminiProvider::new(&config.gemini_key, &config.gemini_model)),
      }
  }
  ```

### 7.3 Highlight Detector

- [x] Buat `src/ai/detector.rs`:
  - Port `detect_highlights()`:
    - Chunk transcript (12K chars local, 250K chars cloud)
    - Send prompt dengan kriteria (hook & payoff, 15-60s, no intro/outro)
    - Parse JSON response: `[{start, duration, title, reason, score}]`
  - Port `_parse_json_highlights()` — regex + JSON parser untuk handle LLM yang sering output JSON tidak sempurna

### 7.4 Metadata Generator

- [x] Buat `src/ai/metadata.rs`:
  - Port `generate_metadata()`:
    - Input: clip text, YouTube title, channel name, URL, visual/audio emotions
    - Output: title, tags, 3-second hook, emotion-enriched words, VFX assignments
  - Centralize prompt templates di `src/ai/prompts.rs`

---

## Bagian 8: Phase 5 — Uploaders & Platform Abstraction

> **Tujuan:** Upload otomatis ke social media.
> **Mapping Python:** `uploaders/*.py`
> **Desain untuk extensibility** — Facebook, X/Twitter bisa ditambah kapan saja.

### 8.1 Uploader Trait & Models

- [ ] Buat `src/uploaders/traits.rs`:
  ```rust
  #[derive(Serialize, Deserialize, Clone, Debug)]
  pub struct UploadMetadata {
      pub title: String,
      pub description: String,
      pub tags: Vec<String>,
      pub visibility: String,        // "public", "private", "unlisted"
      pub thumbnail_path: Option<PathBuf>,
      pub scheduled_time: Option<String>,
  }

  #[derive(Serialize, Clone, Debug)]
  pub struct UploadResult {
      pub success: bool,
      pub platform: String,
      pub url: Option<String>,
      pub error_msg: Option<String>,
  }

  #[async_trait]
  pub trait Uploader: Send + Sync {
      fn platform_name(&self) -> &str;
      async fn upload(
          &self,
          file_path: &Path,
          metadata: &UploadMetadata,
          progress: &ProgressTx,
      ) -> Result<UploadResult, CliptzyError>;
  }
  ```

### 8.2 YouTube Uploader

- [ ] Buat `src/uploaders/youtube.rs`:
  - **Pendekatan:** Gunakan Google OAuth2 tokens dari Supabase session
  - Chunked upload via YouTube Data API v3 (`videos.insert`)
  - Set thumbnail via `thumbnails.set`
  - Auto-add `#shorts` tag
  - Resumable upload untuk file besar
  - **Dependensi:** `reqwest` (sudah ada)

### 8.3 TikTok Uploader

- [ ] Buat `src/uploaders/tiktok.rs`:
  - **Pendekatan:** Direct API jika tersedia, atau browser automation via `tauri-plugin-shell` spawn
  - Parse cookies dari session file
  - Scheduled publish support

### 8.4 Instagram Uploader

- [ ] Buat `src/uploaders/instagram.rs`:
  - **Pendekatan:** Instagram Graph API atau session-based upload
  - Parse sessionid dari cookies

### 8.5 Facebook Uploader (Future-Ready)

- [ ] Buat `src/uploaders/facebook.rs` sebagai placeholder:
  ```rust
  pub struct FacebookUploader;

  #[async_trait]
  impl Uploader for FacebookUploader {
      fn platform_name(&self) -> &str { "facebook" }

      async fn upload(&self, ...) -> Result<UploadResult, CliptzyError> {
          Err(CliptzyError::Upload {
              platform: "facebook".into(),
              message: "Facebook Reels upload belum diimplementasi".into(),
          })
      }
  }
  ```
- [ ] Struktur sudah siap — tinggal isi implementasi saat waktunya tiba

### 8.6 Batch Upload Orchestrator

- [ ] Buat `src/orchestrator/batch_upload.rs`:
  - Upload multiple clips ke multiple platforms
  - Interval antar upload (konfigurabel)
  - Retry logic dengan backoff
  - Progress per-clip per-platform

---

## Bagian 9: Phase 6 — Orchestration & Use Cases

> **Tujuan:** Satukan semua modul menjadi workflow end-to-end.
> **Mapping Python:** `controller.py`, `use_cases/*.py`

### 9.1 Pipeline Context

- [x] Buat `src/orchestrator/pipeline.rs`:
  ```rust
  pub struct PipelineContext {
      pub job_dir: PathBuf,
      pub video_id: String,
      pub config: AppConfig,
      pub cancel_token: CancelToken,
      pub progress_tx: ProgressTx,
      pub app_handle: tauri::AppHandle,
      pub metadata: HashMap<String, serde_json::Value>,  // shared state antar stage
  }
  ```

### 9.2 Clip Video Use Case (Phase 1)

- [x] Buat `src/orchestrator/clip.rs`:
  - Port `ClipVideoUseCase.execute()`:
    1. Resolve target segments (heatmap / AI / custom)
    2. Pre-download semua segments **secara paralel** (`tokio::task::JoinSet`)
    3. Untuk setiap segment:
       - Cut video (download range atau ffmpeg copy)
       - Detect face (jika crop mode membutuhkan)
       - Crop video via `CropStrategy`
       - Generate transcript (jika use_subtitle)
       - Generate AI metadata (jika use_highlight)
       - Write ASS subtitle
       - Burn subtitle + VFX overlay
       - Stack intro/outro/watermark
    4. Merge clips jika `merge_clips = true`
    5. Generate thumbnail
    6. Return output paths

### 9.3 Render Clip Use Case (Phase 2)

- [x] Buat `src/orchestrator/render.rs`:
  - Port `RenderClipUseCase.execute()`:
    - Re-render existing clips dengan settings baru
    - Concurrent processing via `JoinSet`

### 9.4 Compilation Use Case

- [x] Buat `src/orchestrator/compile.rs`:
  - Port `CompileVideoUseCase`:
    - Generate numbering cards
    - Process each clip
    - Concat semua segments (cards + clips)
    - Generate compilation thumbnail (2x2 grid)

### 9.5 Tauri Commands Integration

- [x] Buat/extend `src/commands/video.rs`:
  ```rust
  #[tauri::command]
  pub async fn clip_video(
      app: tauri::AppHandle,
      payload: ClipPayload,
  ) -> Result<ClipResult, String> {
      let cancel_token = CancelToken::new();
      // Store cancel_token agar bisa di-cancel dari frontend
      // ...
      let result = ClipVideoUseCase::new(app.clone(), cancel_token)
          .execute(payload)
          .await?;
      Ok(result)
  }

  #[tauri::command]
  pub async fn cancel_current_job(app: tauri::AppHandle) -> Result<(), String> {
      // Cancel via stored CancelToken
  }
  ```

---

## Bagian 10: Phase 7 — Polish, TTS, & Compilation

> **Modul-modul ini bersifat enhancement dan bisa ditunda.**

### 10.1 TTS Engine

- [ ] Buat `src/tts/engine.rs`:
  - **Rekomendasi:** Gunakan `edge-tts` CLI (Microsoft Edge TTS) sebagai primary
    - Ringan, tidak butuh model lokal, kualitas bagus
    - Spawn via `tokio::process::Command`
  - Fallback: `gtts-cli` (Google TTS)
  - **Kokoro TTS** → terlalu berat untuk bundle Rust (butuh PyTorch). Simpan sebagai opsional.
  - Voice clone (Kanade) → **tunda** sampai ada Rust binding yang memadai

### 10.2 Face Detection

- [x] Buat `src/face/detector.rs`:
  - **Implementasi:** Menggunakan `rustface` (SeetaFace) murni di CPU.
  - Alasan: Lebih ringan, tidak butuh dependensi `ort`/ONNX.
  - Threshold ditingkatkan ke 3.5 (untuk memfilter patung/kartun).

- [x] Buat `src/face/tracker.rs`:
  - Port `get_face_keyframes()`:
    - Extract JPEG dari FFmpeg di resolusi 360p (9x lebih cepat).
    - Detect face di setiap frame via `rustface`.
    - Jitter filtering (threshold 0.03) & Exponential Moving Average (EMA).
    - Extreme movement detection (threshold 0.15).
    - Classify: `glide` vs `cut`
    - Keyframe deduplication

### 10.3 Emotion Analysis (OPSIONAL — Feature Gate)

> **Rekomendasi:** Tunda modul ini. Gunakan AI provider (Gemini/OpenAI) untuk analisis emosi
> berbasis teks, bukan model ML lokal. Ini mengurangi bundle size **~2GB+**.

- [ ] Jika tetap ingin implementasi:
  - Gunakan `ort` (ONNX Runtime) untuk semua model:
    - Face emotion: Convert HuggingFace model ke ONNX
    - Audio event: Convert AST model ke ONNX
    - Voice emotion: Convert Wav2Vec2 ke ONNX
    - Text emotion: Convert Roberta ke ONNX
  - Feature gate: `#[cfg(feature = "local-ml")]`

### 10.4 Channel Manager

- [ ] Buat `src/channels/manager.rs`:
  - Port `ChannelManager`:
    - CRUD operations pada `channels/channels.json`
    - Scrape channel metadata via yt-dlp
    - Video catalog filtering & pagination

---

## Bagian 11: Strategi Bundle & Distribusi Binary

### 11.1 Estimasi Ukuran Bundle

| Komponen | Ukuran Estimasi | Catatan |
|---|---|---|
| Tauri + Vue + Webview | ~15-20 MB | Base app |
| Rust binary (compiled) | ~5-10 MB | Semua logic native |
| `whisper.cpp` model (small) | ~466 MB | Download on demand, BUKAN bundled |
| FFmpeg binary | ~80-120 MB | Download on demand atau expect system install |
| yt-dlp binary | ~10-15 MB | Managed oleh `yt-dlp` crate |
| ONNX models (face/emotion) | ~50-200 MB | Download on demand |
| **Total installer** | **~25-35 MB** | Tanpa models, models di-download saat pertama kali dipakai |

### 11.2 Strategi "Download on Demand"

```rust
// src/deps/manager.rs
pub struct DependencyManager;

impl DependencyManager {
    /// Check & download FFmpeg jika belum ada
    pub async fn ensure_ffmpeg(progress: &ProgressTx) -> Result<PathBuf, CliptzyError>;

    /// Check & download Whisper model jika belum ada
    pub async fn ensure_whisper_model(
        model_name: &str,
        progress: &ProgressTx,
    ) -> Result<PathBuf, CliptzyError>;

    /// Check & download YuNet ONNX model
    pub async fn ensure_face_model(progress: &ProgressTx) -> Result<PathBuf, CliptzyError>;
}
```

### 11.3 Feature Gates untuk Komponen Opsional

```toml
# Cargo.toml
[features]
default = ["whisper", "face-detection"]
whisper = ["whisper-rs"]
face-detection = ["ort"]
local-ml = ["ort"]  # emotion analysis, voice analysis, etc.
```

Ini memungkinkan build **minimal** untuk development cepat dan build **full** untuk release.

---

## Bagian 12: Prioritas & Urutan Eksekusi

```
Phase 0 ──→ Phase 1 ──→ Phase 2 ──→ Phase 3 ──→ Phase 4 ──→ Phase 5 ──→ Phase 6 ──→ Phase 7
Foundation   Acquire    FFmpeg       Whisper      AI          Upload     Orchestrate  Polish
  (1 minggu)  (1 minggu) (2-3 minggu) (1-2 minggu) (1 minggu) (1-2 minggu) (2 minggu) (ongoing)
```

### Milestone Deliverables

| Milestone | Deskripsi | Apa yang Bisa Dilakukan User |
|---|---|---|
| **M1: Foundation** | Phase 0 selesai | Tidak ada perubahan visible, tapi codebase lebih bersih |
| **M2: Scan & Preview** | Phase 1 selesai | User bisa scan video YouTube, lihat heatmap, preview metadata |
| **M3: Basic Clip** | Phase 2 (crop mode default + full) | User bisa **crop video** dengan mode default dan full |
| **M4: Subtitle** | Phase 3 selesai | User bisa crop + **subtitle otomatis** |
| **M5: AI Highlights** | Phase 4 selesai | User bisa **scan highlight via AI** dan generate metadata |
| **M6: Auto Upload** | Phase 5 selesai | User bisa upload otomatis ke YouTube |
| **M7: End-to-End** | Phase 6 selesai | **Full workflow** dari scan → clip → upload |
| **M8: All Crop Modes** | Phase 2 lanjutan (face tracking) | Semua 9 crop mode berfungsi |
| **M9: Compilation** | Phase 7 | Mode kompilasi Top N |

### Minimum Viable Product (MVP)

> **MVP = M3 (Basic Clip)**
>
> User bisa: Paste URL YouTube → Scan heatmap → Pilih segments → Crop ke 9:16 (default mode) → Download clip final.
>
> Ini sudah memberikan **value utama** aplikasi tanpa menunggu semua fitur selesai.

---

## Bagian 13: Keputusan Teknis Kunci (ADR)

### ADR-001: FFmpeg via CLI vs FFI Binding

**Keputusan: CLI (`std::process::Command` / `tokio::process::Command`)**

| Pro CLI | Contra FFI |
|---|---|
| Tidak perlu compile FFmpeg dari source | `ffmpeg-sys` butuh ~30 menit compile |
| Mudah debug (lihat command yang dijalankan) | FFI error sulit di-debug |
| User bisa pakai FFmpeg versi mereka sendiri | Binding sering out-of-date |
| Filter complex string sudah battle-tested | FFI API berbeda dari CLI API |
| Bundle size lebih kecil (FFmpeg terpisah) | Static link menambah ~50MB+ ke binary |

**Mitigasi risiko CLI:**
- Abstraksi di `FFmpegCommand` builder — jika suatu hari mau switch ke FFI, hanya ganti implementasi internal
- Proper error handling dari exit code + stderr parsing

### ADR-002: Whisper via whisper-rs vs CLI

**Keputusan: `whisper-rs` (in-process binding)**

- Lebih cepat (tanpa overhead spawn process)
- Word-level timestamps langsung sebagai struct
- Metal support di macOS, CUDA di Linux/Windows
- Fallback ke CLI jika build gagal di platform tertentu

### ADR-003: Emotion Analysis — Lokal vs Cloud

**Keputusan: Cloud-first (via AI provider), lokal opsional**

- Python version butuh PyTorch + 4 model ML (~2-3 GB RAM, ~1-2 GB disk)
- Untuk Rust, delegate ke AI provider (Gemini/OpenAI) via prompt engineering
  - Kirim transcript text → minta analisis emosi per kata
  - Jauh lebih ringan, tidak perlu model lokal
- Jika user mau lokal: feature gate `local-ml` dengan ONNX Runtime

### ADR-004: TTS Engine

**Keputusan: `edge-tts` CLI sebagai primary**

- Microsoft Edge TTS: gratis, kualitas tinggi, banyak bahasa
- Spawn sebagai subprocess, bukan library dependency
- Kokoro TTS butuh PyTorch → terlalu berat untuk Rust native
- Voice clone (Kanade) → tunda sampai ada solusi Rust native

### ADR-005: Social Media Uploaders — Native vs Browser Automation

**Keputusan: Native API untuk YouTube, hybrid untuk TikTok/Instagram**

- YouTube: Google API v3 + OAuth2 (sudah punya tokens dari Supabase)
- TikTok: Official Creator API jika tersedia, atau delegate ke CLI tool
- Instagram: Graph API untuk business accounts
- Semua di-abstract via `Uploader` trait → mudah swap implementasi

### ADR-006: Shared State Management di Tauri

**Keputusan: `tauri::Manager::state()` dengan `Arc<Mutex<>>` untuk mutable state**

```rust
// Di lib.rs setup:
.manage(Arc::new(Mutex::new(AppConfig::default())))
.manage(Arc::new(Mutex::new(JobManager::new())))

// Di commands:
#[tauri::command]
async fn get_config(config: State<'_, Arc<Mutex<AppConfig>>>) -> Result<AppConfig, String> {
    Ok(config.lock().unwrap().clone())
}
```

Untuk job cancellation, gunakan `CancellationToken` yang disimpan di `JobManager`.

---

## 📎 Lampiran: Crate Rust yang Direkomendasikan

| Kebutuhan | Crate | Catatan |
|---|---|---|
| Error handling | `thiserror` | Derive macro untuk error types |
| Async runtime | `tokio` | ✅ Sudah ada |
| HTTP client | `reqwest` | ✅ Sudah ada |
| JSON | `serde`, `serde_json` | ✅ Sudah ada |
| Cancellation | `tokio-util` | `CancellationToken` |
| Whisper STT | `whisper-rs` | Binding ke whisper.cpp |
| ONNX inference | `ort` | Untuk face detection + ML models |
| YouTube download | `yt-dlp` | ✅ Sudah ada (crate) |
| Regex | `regex` | Untuk parse AI JSON responses |
| Date/Time | `chrono` | Untuk scheduled uploads |
| URL parsing | `url` | ✅ Sudah ada |
| Concurrent map | `dashmap` | Untuk in-memory cache |
| UUID | `uuid` | Untuk job IDs |
| Temp files | `tempfile` | Untuk FFmpeg intermediate files |
| File watcher | `notify` | Opsional: watch output directory |
| Async trait | `async-trait` | Untuk trait dengan async methods |

---

> **Dokumen ini harus di-update setiap kali ada phase yang selesai atau keputusan arsitektur baru.**
> Tandai item checklist dengan `[x]` saat selesai.
