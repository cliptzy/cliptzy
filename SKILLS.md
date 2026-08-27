# 🛠️ SKILLS.md — Panduan Teknis Implementasi Arsitektur Rust Cliptzy

Dokumen ini berisi **panduan teknis mendalam** untuk ekosistem Rust native Cliptzy. Seluruh peninggalan *engine* Python telah dihapus. Gunakan sebagai referensi arsitektur saat mengerjakan `TODO_CORE.md`.

---

## 📑 DAFTAR ISI

1. [Skill 1: FFmpeg Pipeline Architecture](#skill-1-ffmpeg-pipeline-architecture)
2. [Skill 2: Sistem Strategi Crop (Strategy Pattern)](#skill-2-sistem-strategi-crop)
3. [Skill 3: Whisper & Subtitle Generator](#skill-3-whisper--subtitle-generator)
4. [Skill 4: Deteksi Wajah & Keyframe Tracking](#skill-4-deteksi-wajah--keyframe-tracking)
5. [Skill 5: Abstraksi AI Providers](#skill-5-abstraksi-ai-providers)
6. [Skill 6: Arsitektur Uploaders](#skill-6-arsitektur-uploaders)
7. [Skill 7: Integrasi Sistem & Konfigurasi SSOT](#skill-7-integrasi-sistem--konfigurasi-ssot)

---

## Skill 1: FFmpeg Pipeline Architecture

**Folder Utama**: `src-tauri/src/processing/ffmpeg/`

### Konteks
Aplikasi tidak lagi menggunakan penyatuan (concatenation) string raw untuk perintah FFmpeg. Kita menggunakan pendekatan pembangun berbasis graf (Graph-based builder) untuk filter kompleks.

### Konsep Inti
1. **`FilterGraph` (`filters.rs`)**:
   Struktur ini membangun rantai `-filter_complex`. Setiap filter memiliki input dan output "pads" (contoh: `[0:v]`, `[v1]`).
   - Gunakan `add_input` untuk memetakan sumber.
   - Panggil fungsi utilitas seperti `FilterGraph::scale`, `FilterGraph::crop`, `FilterGraph::overlay` untuk membuat node filter.
   - Panggil `graph.to_string()` untuk merendernya secara aman untuk shell.

2. **`HwAccel` (`hwaccel.rs`)**:
   Mendeteksi akselerasi GPU secara dinamis di level OS.
   - Mac: VideoToolbox
   - Windows/Linux: NVENC (Nvidia), QSV (Intel), AMF (AMD)
   - Fungsi `encode_args()` menyuntikkan argumen CRF/Bitrate yang sudah dioptimalkan (membatasi *bloat* ukuran render, contoh: `-b:v 3000k` untuk encoder HW).

3. **`Runner`**:
   Pembungkus proses *spawn* FFmpeg (atau langsung menggunakan `rust_ffmpeg`) yang akan membaca `stderr` secara *asynchronous* untuk menerbitkan (emit) progress ke antarmuka Tauri.

---

## Skill 2: Sistem Strategi Crop

**Folder Utama**: `src-tauri/src/processing/cropper.rs`

### Pola Desain (Design Pattern)
Kita menggunakan **Strategy Pattern** agar mode pemotongan (crop) baru dapat ditambahkan tanpa memodifikasi kode pemotongan yang sudah ada (*Open-Closed Principle*).

### Implementasi
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
        hwaccel: &HwAccel,
    ) -> Result<FFmpegCommand, CliptzyError>;
}
```

### Cara Menambah Mode Crop Baru:
1. Buat struct baru, misal `pub struct SplitBrollCrop;`
2. Implementasikan trait `CropStrategy`.
3. Dalam fungsi `build_command`, rakit algoritma `filter_complex` FFmpeg kamu.
4. Tambahkan mode ke *factory* `create_crop_strategy()` agar dikenali oleh GUI.

---

## Skill 3: Whisper & Subtitle Generator

**Folder Utama**: `src-tauri/src/transcription/`

### Mekanisme `whisper-rs`
- Kita menggunakan FFI binding `whisper-rs` (C++) untuk performa *in-process* ultra-cepat.
- Model `.bin` diunduh secara otomatis via `deps/manager.rs` saat dibutuhkan pertama kali.
- **Output Penting**: Audio diekstrak (16kHz WAV), lalu ditranskripsikan dengan fitur ekstraksi *Word-level timestamp* (`WordTimestamp`).

### Pembuat ASS (ASS Writer)
Modul `ass_writer.rs` mengubah `Vec<TranscriptSegment>` menjadi file subtitle `.ass`.
- **Styling**: `plain` vs `full_color` (Karaoke).
- **Animasi**: Pop-in `hormozi` memanfaatkan tag bawaan ASS `\fscx` dan `\fscy`.
- Penggandaan efek ASS di-render via FFmpeg filter `subtitles=...`.

---

## Skill 4: Deteksi Wajah & Keyframe Tracking

**Folder Utama**: `src-tauri/src/face/`

### Konteks
Fitur ini dibutuhkan untuk crop dinamis seperti `CenterFaceCrop` yang mengunci kamera ke wajah pembicara saat ia bergerak.

### Alur Pelacakan (Tracking Pipeline):
1. **Deteksi Dasar (`detector.rs`)**:
   Mengekstrak frame JPEG setiap `N` detik menggunakan FFmpeg ke folder lokal, lalu memindai wajah via `rustface` (atau `ort` untuk YuNet).
2. **Normalisasi**:
   Seluruh koordinat wajah (`x`, `y`) dinormalisasi menjadi skala `0.0 - 1.0` (Center `cx`, `cy`).
3. **Penyaringan (Filtering) & Keyframe (`tracker.rs`)**:
   - Algoritma membuang *jitter* mikroskopis (ambang batas perbedaan < 0.03).
   - Menentukan pergerakan: `glide` (gerakan halus interpolasi linear) vs `cut` (pemotongan instan apabila pembicara melompat ekstrem > 0.15).

---

## Skill 5: Abstraksi AI Providers

**Folder Utama**: `src-tauri/src/ai/`

### Konteks
Modul penghasil *hook*, metadata, dan *highlight* video (pemotong video panjang menjadi durasi pendek).

### Implementasi Trait
Sama halnya dengan *Cropper*, integrasi LLM menggunakan trait `AIProvider`:
- `OllamaProvider`: Untuk memproses secara lokal (via `http://localhost:11434/api/generate`).
- `GeminiProvider`: API berbasis REST milik Google.
- `OpenAIProvider`: Kompatibel untuk OpenAI, Groq, dll.

### Regex Parser yang Kebal Error
LLM acap kali membalas *prompt* JSON dengan markdown *codeblocks* (` ```json ... ``` `) yang tidak valid. Modul parsing menggunakan RegEx untuk mengekstrak hanya bagian JSON-nya (diimplementasikan di `detector.rs`).

---

## Skill 6: Arsitektur Uploaders

**Folder Utama**: `src-tauri/src/uploaders/`

### Konteks
Mekanisme pengunggahan otomatis paska-rendering. Dibangun untuk mudah diperluas (*extensible*).

### Desain Trait
```rust
#[async_trait]
pub trait Uploader: Send + Sync {
    fn platform_name(&self) -> &str;
    async fn upload(
        &self,
        file_path: &Path,
        metadata: &UploadMetadata,
        app: &AppHandle, // Untuk progress reporting
    ) -> Result<UploadResult, CliptzyError>;
}
```
Setiap platform (YouTube, TikTok, dll.) mengimplementasikan trait ini. Manajemen otentikasi (OAuth via Supabase atau Session Cookies) diatur langsung oleh pengunggah (*uploader*) spesifik atau lewat utilitas sesi bawaan Tauri.

---

## Skill 7: Integrasi Sistem & Konfigurasi SSOT

### Konfigurasi (Single Source of Truth)
- Konfigurasi diletakkan di `src/config/`.
- Frontend Vue (`settings.ts`) memanggil Tauri `invoke('load_config_file')` saat `App.vue` ter- *mount*.
- Rust menangani *disk writing* (penulisan I/O) yang memastikan sinkronisasi yang valid jika backend perlu membaca konfigurasi secara simultan (*Thread-Safe*).

### Pelaporan Progres ke Antarmuka
Karena Rust memiliki arsitektur *thread/task* yang bekerja secara asinkron di belakang layar (`tokio::spawn`), *progress updates* tidak boleh menyita siklus utama aplikasi.
- **Event Bus Tauri**: Kita membungkus event menggunakan `ProgressEvent` (label, current, total, persentase) dan memancarkannya (`app_handle.emit()`).
- Vue UI mendengarkannya via `listen('clip-progress')` untuk ditampilkan di komponen `GlobalStatusBar.vue`.

---
*Diperbarui dan dirombak ulang pada 27 Agustus 2026. Sinkron dengan implementasi Native Rust & Tauri terkini.*
