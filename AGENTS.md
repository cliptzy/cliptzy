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

## 🛠️ BAGIAN 4: LOG IMPLEMENTASI & PENYELESAIAN MASALAH (WORK LOG)

### 4.1. YouTube Bypass (Error 429 & PO Token)
- **Problem**: YouTube secara agresif memblokir *request* dari `yt-dlp` menggunakan deteksi Bot, Error 429, dan kewajiban menyertakan PO Token / Visitor Data untuk *client* Web standar.
- **Solusi**: 
  1. Melakukan *spoofing client* menggunakan kombinasi `android,web,default` via argumen `--extractor-args`.
  2. Mengaktifkan *External JavaScript* (`--remote-components ejs:github`) yang secara otomatis menggunakan `Node.js` / `Deno` lokal pengguna untuk memecahkan *challenge* enkripsi YouTube (mengadopsi kesuksesan *engine* Python lama).
- **Catatan Penting pada `rusty_ytdl`**: Crate `rusty_ytdl` memiliki *bug* dimana argumen khusus (`self.args`) diabaikan pada fase `fetch_video_infos`. Karenanya, fungsi `analyze_youtube_video` di-refactor menggunakan *raw* `tokio::process::Command` untuk memanggil `yt-dlp -J` secara independen, memastikan argumen bypass 100% tereksekusi dengan benar dan heatmap JSON bisa diekstrak sempurna.

### 4.2. Manajemen Cookies YouTube Terintegrasi
- **Problem**: Kebutuhan mem-bypass otentikasi YouTube mengharuskan penggunaan file `cookies.txt` yang valid.
- **Solusi**:
  1. Menambahkan tombol "Pilih File Cookies" dan "Test yt-dlp" di halaman `SettingsView.vue` (Profile Section).
  2. Implementasi Tauri command `copy_cookies_file` untuk mengimpor file *cookies* ke dalam `app_data_dir()/cred/cookies.txt`.
  3. Implementasi command `validate_cookies_file` (Cek format Netscape & kedaluwarsa) serta `test_youtube_cookies` (Membuktikan *cookies* tersebut valid untuk *fetch* yt-dlp secara nyata, bukan sekadar tes format).

### 4.3. Optimasi Kecepatan Face Detector (Downscaling)
- **Problem**: Ekstraksi *face keyframes* menggunakan OpenCV (`rustface`) pada video 1080p memakan waktu sangat lambat (bermenit-menit) karena ukuran matriks gambar yang masif.
- **Solusi**: Menyuntikkan perintah `-vf scale=-1:360` ke dalam *spawn* FFmpeg saat mengekstrak *frame* berformat JPEG ke *tempdir*. Proses dekompresi, *I/O storage*, dan kalkulasi wajah menjadi ~9x lipat lebih cepat.
- **Justifikasi**: Algoritma perhitungan koordinat wajah kita sudah ternormalisasi (`cx` dan `cy` berada di interval `0.0 - 1.0`). Hasil deteksi wajah di resolusi 360p bisa di- *mapping* sempurna kembali ke video *source* 1080p saat proses *cropping* tanpa kehilangan keakuratan.

### 4.4. Manajemen Pembatalan Proses (Aggressive Process Killing)
- **Problem**: Ketika *user* menekan tombol "Batal" di Global Status Bar, proses anak (*child process*) yang di-*spawn* oleh `tokio` (seperti `ffmpeg` dan `yt-dlp`) kerap menjadi *orphan* (zombie process) dan terus membebani RAM/CPU.
- **Solusi**: Mengimplementasikan `killall ffmpeg` dan `killall yt-dlp` (`taskkill /F /IM ffmpeg.exe` di Windows) via Command saat proses dibatalkan di `cancel_processing()`. Hal ini menjamin pembatalan bekerja bersih dan tuntas secara OS-level.

### 4.5. Pelaporan Progres Latar Belakang (UI Updates)
- Menambahkan emisi `ProgressEvent` via Tauri `app_handle.emit()` di dalam *looping* Face Tracker dan modul lainnya. *Frontend* sekarang menampilkan persentase progres pemrosesan secara *real-time* di `GlobalStatusBar.vue`.

### 4.6. Deteksi GPU Fisik & Sinkronisasi Hardware Acceleration
- **Problem**: Pengaturan *Hardware Acceleration* sebelumnya bergantung pada `ffmpeg -hwaccels` yang hanya mengembalikan *flag* saat di-kompilasi (misal `nvenc` atau `qsv` selalu muncul jika *binary* mendukungnya, walau GPU fisiknya tidak ada). Hal ini menyebabkan *error* rendering atau opsi palsu di UI.
- **Solusi**: Mengimplementasi `crate::utils::get_system_gpus()` untuk melakukan *polling* akurat ke OS level (`powershell WMI` di Windows, `system_profiler` di Mac). Menyamakan validasi OS dengan kapabilitas *binary* `ffmpeg`, lalu membuat antarmuka Vue (Pinia) agar otomatis melakukan *fallback* ke mode `CPU` bila GPU yang dikonfigurasi tidak terdeteksi.

### 4.7. Background Monitor Utilisasi GPU Tanpa Blocking (Zero-Cost Sharing)
- **Problem**: Penggunaan *resource* GPU (`gpu_usage`) di dashboard (*monitor.rs*) selalu nol karena `sysinfo` belum memiliki kapabilitas membaca GPU bawaan.
- **Solusi**: Membuat `std::thread::spawn` yang akan menjalankan perintah `typeperf` secara asinkron di belakang layar (khusus Windows), membaca metrik secara konstan dan menyimpan nilai `max_val` persentase penggunaannya. Agar pengiriman UI tidak menjadi lambat, hasil tersebut disimpan ke `std::sync::atomic::AtomicU32` sehingga `get_system_metrics()` bisa menarik angka secara kilat tanpa *lock* (zero-cost).

### 4.8. Optimasi Performa I/O dengan Refaktor DRY (Dependency Injection)
- **Problem**: File `AppConfig::load()` dibaca berkali-kali dari disk pada saat *rendering* panas (seperti saat spawn FFmpeg di `cropper.rs` dan `burner.rs`) yang melanggar prinsip *Don't Repeat Yourself* (DRY) dan menyebabkan *bottleneck I/O*.
- **Solusi**: Memperbaiki arsitektur aplikasi sehingga *Config* ditarik satu kali di level orkestrator (`clip.rs`) dan kemudian nilai konfigurasi yang diperlukan (`HwAccel`) dipassing ke bawah secara beruntun (*pass-by-value/reference*) via `OutputConfig` dan `VideoBurnerConfig`. Selain itu, merombak deteksi utilitas murni agar seluruh pemanggilan `tokio::process::Command::new("ffmpeg")` difilter secara ketat melalui implementasi pembaca `find_executable` (*which* crate).
