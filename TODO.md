# 📋 TODO.md — Cliptzy Desktop (Native Rust Architecture Roadmap)

> **Arsitektur Baru**: Tauri (Rust Native Backend) + Vue 3 (Frontend)
>
> **Catatan Penting**: Arsitektur Hybrid Python (FastAPI) telah **dibatalkan**. Seluruh pemrosesan (Auth, Subs, Video, AI) akan ditulis ulang atau dibungkus menggunakan Rust dan Tauri Commands. Submodule `engine/` (Python) akan dihapus sepenuhnya.

---

## Fase 1: Migrasi Inti & Pengaturan Ulang Proyek

- [x] Inisialisasi Tauri v2 + Vue 3.
- [x] Konfigurasi Tauri (`tauri.conf.json`) dengan identifier dan command dev yang tepat.
- [x] Migrasikan sistem Autentikasi dan Sync (Supabase) dari Python ke Rust (`supabase.rs`).
- [x] Implementasi endpoint `login_with_google`, `logout`, dan Storage API di Rust via `reqwest`.
- [x] Perbarui `AGENTS.md` untuk merefleksikan perubahan arsitektur Murni Rust.

## Fase 2: Pembangunan Kembali Fitur Inti di Rust

Seluruh pustaka Python lama harus dicarikan padanannya di ekosistem Rust, atau menggunakan pemanggilan CLI eksternal yang diorkestrasi oleh Rust (`std::process::Command`).

- [x] **Modul Download Video (`yt-dlp`)**:
  - Gunakan crate `youtube_dl` atau jalankan binari `yt-dlp` langsung dari Rust (`src/video/youtube.rs`).
- [ ] **Modul Transkripsi (Whisper)**:
  - Gunakan crate `whisper-rs` (binding ke whisper.cpp) atau panggil binari mandiri `whisper.cpp` untuk transkripsi audio.
- [ ] **Pemrosesan Video (FFmpeg)**:
  - Buat helper/modul Rust yang membungkus panggilan ke binari `ffmpeg` (membuat subtitle, crop, padding).
- [ ] **Deteksi AI & Sorotan (Highlight)**:
  - Buat client LLM (Ollama/Gemini/OpenAI) langsung di Rust menggunakan `reqwest` untuk logika _content highlight_.

## Fase 3: Integrasi Frontend Vue 3 & State Management

- [x] Konfigurasi Pinia & TailwindCSS.
- [x] Perbarui Auth Store Vue (`auth.ts`) untuk menggunakan `invoke` alih-alih `proxyRequest`.
- [x] Buat UI Halaman Login, Dashboard Utama (URL Input), Settings.
- [x] Rancang ulang komponen _Progress Bar_ & _Log Viewer_ agar mendengarkan event yang di-_emit_ oleh proses Rust (`app.emit("log", ...)`).

## Fase 4: Optimasi dan Distribusi

- [ ] Manajemen Dependensi Eksternal (FFmpeg, yt-dlp, model AI):
  - Rust harus memiliki modul "Bootstrapper/Downloader" yang akan mengecek keberadaan binari `ffmpeg`, `yt-dlp`, dan file model `.bin` di direktori `AppData`, lalu mengunduhnya secara otomatis jika belum ada saat aplikasi dibuka.
- [ ] Hapus modul lawas yang berkaitan dengan _Health Check_ / Port Proxying FastAPI (`health.rs`, `engine.rs`), karena semua logic kini internal.
- [ ] Buat alur GitHub Actions untuk _cross-platform compilation_ Tauri (Windows, MacOS, Linux).
