# 📜 AGENTS.md — Peraturan & Panduan Proyek Cliptzy (Tauri Desktop App)

Dokumen ini adalah **sumber kebenaran utama (single source of truth)** untuk seluruh AI Model dan pengembang yang bekerja pada proyek **Cliptzy Desktop** — sebuah aplikasi YouTube Clipper & Auto Uploader yang dibangun menggunakan arsitektur **Murni Rust & Tauri (Native)**.

> ⚠️ **BACA SELURUH DOKUMEN INI SEBELUM MENULIS SATU BARIS KODE PUN.**
> Setiap asumsi yang tidak diverifikasi dari dokumen ini dianggap **halusinasi** dan harus dihindari.

---

## 📐 BAGIAN 0: GROUND TRUTH — STATUS PROYEK SAAT INI

> **Bagian ini WAJIB dibaca terlebih dahulu.**

### 0.1 Apa yang SUDAH ADA (Fakta)

| Komponen | Status | Detail |
|----------|--------|--------|
| **Tauri + Vue 3 + TypeScript** | ✅ Scaffold dasar | Proyek Tauri v2 sudah diinisialisasi. |
| **Rust Backend (Tauri)** | ✅ Pengembangan Aktif | Fungsionalitas autentikasi, manajemen config, dan manajemen sesi (Supabase) sudah di-porting ke native Rust (`supabase.rs`). |
| **Video Engine** | ✅ Selesai | Ekstraksi Metadata & Stream URL menggunakan native Rust via binding CLI `yt-dlp` (di `youtube.rs`), dengan pengiriman data *frontend* via IPC Tauri commands. |
| **Studio Editor UI** | ✅ Fungsional | Sinkronisasi mutakhir antara YouTube IFrame Player (Center Cropped 9:16) dan Timeline Editor berbasis durasi klip, Multi-track rendering untuk Teks (Subtitle) & Video, serta Auto-Looping batas pemutaran segmen. |
| **Python Engine (Legacy)** | 🗑️ Akan Dihapus | Sebelumnya aplikasi menggunakan Python FastAPI server di-spawn sebagai child process. **Pendekatan ini dibatalkan** demi performa dan kemudahan distribusi. Seluruh logic akan ditulis ulang dalam Rust murni. |

### 0.2 Apa yang BELUM ADA (Belum Diimplementasi)

Seluruh logika pemrosesan AI (Whisper, FFmpeg, yt-dlp, DeepFace) dari kode Python lama akan diterjemahkan dan diimplementasikan secara native di Rust (atau menggunakan binding native / crate Rust yang ekuivalen).
- Modul Rust untuk memanggil FFmpeg (`ffmpeg-cli` atau `std::process::Command`).
- Modul Rust untuk transkripsi audio (`whisper-rs` atau ekuivalen).
- Modul Rust untuk integrasi YouTube (`rusty_ytdl` atau binding `yt-dlp` CLI).
- Migrasi sisa antarmuka Vue untuk memanggil Tauri Commands langsung (tanpa proxy HTTP ke Python).

---

## 🏗️ BAGIAN 1: ARSITEKTUR SISTEM (BARU)

Seluruh sistem sekarang terdiri dari dua lapisan saja (Frontend dan Native Backend).

```
┌─────────────────────────────────────────────────────────────┐
│                    Tauri Window (Webview)                    │
│  ┌───────────────────────────────────────────────────────┐  │
│  │              Vue 3 + Pinia + Tailwind               │  │
│  │  ┌──────────┐ ┌──────────┐ ┌────────────────────┐   │  │
│  │  │Dashboard │ │Compiler  │ │ Settings/Upload    │   │  │
│  │  └──────────┘ └──────────┘ └────────────────────┘   │  │
│  └───────────────────┬─────────────────────────────────┘  │
│                      │ invoke() / listen()                  │
│  ┌───────────────────▼─────────────────────────────────┐  │
│  │             Native Rust Backend (Tauri)              │  │
│  │  ┌────────────┐ ┌────────┐ ┌──────────────────┐    │  │
│  │  │supabase.rs │ │video.rs│ │  audio/ai.rs     │    │  │
│  │  │(Auth & DB) │ │(FFmpeg)│ │ (Whisper / dll)  │    │  │
│  │  └────────────┘ └────────┘ └──────────────────┘    │  │
│  └─────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

---

## 🚫 BAGIAN 2: LARANGAN UTAMA (STRICT PROHIBITIONS)

### 2.1 Larangan Umum

1. **DILARANG MENGGUNAKAN PYTHON** — Kita tidak lagi menggunakan server Python (FastAPI). Jangan pernah mengusulkan atau menambahkan kembali kode Python untuk fungsi backend. Semuanya harus native Rust.
2. **DILARANG hardcode path absolut** — Gunakan modul `crate::paths` di Rust untuk mendapatkan lokasi AppData dan direktori kerja.
3. **DILARANG menambah dependensi besar tanpa alasan yang kuat** — Cari implementasi crate Rust terkecil yang bisa menyelesaikan tugas, hindari crate monolithic jika tidak perlu.

### 2.2 Larangan Frontend (Vue + TypeScript)

1. **DILARANG menggunakan fetch() ke localhost** — Semua komunikasi dengan logic AI/backend **HARUS** melalui `invoke()` dari `@tauri-apps/api/core`.
2. **State Management** — Hanya gunakan Pinia.

### 2.3 Larangan Rust (Tauri Backend)

1. **Jangan block main thread Tauri** — Gunakan `async` / `await` (`tokio`) untuk semua I/O dan operasi berat (transkripsi, render video).
2. **Error Handling** — Gunakan `Result<T, String>` (via `.map_err(|e| e.to_string())`) untuk diteruskan ke Frontend Tauri. Jangan gunakan `unwrap()` di production path.

---

## 📋 BAGIAN 3: CHECKLIST AI SEBELUM MULAI BEKERJA

Sebelum menulis kode, AI Model WAJIB menjawab pertanyaan berikut:

1. ✅ Apakah saya sadar bahwa aplikasi ini **100% Native Rust + Vue** dan kita telah membuang Python?
2. ✅ Apakah saya menggunakan Crate Rust (seperti `reqwest`, `tokio`, dsb) atau CLI (seperti memanggil `ffmpeg.exe` via `std::process::Command`) alih-alih server Python?
3. ✅ Apakah semua perintah di-*expose* ke Frontend melalui `#[tauri::command]`?
