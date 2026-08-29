# 📜 AGENTS.md — Peraturan & Panduan Proyek Cliptzy (Tauri Desktop App)

Dokumen ini adalah **sumber kebenaran utama (single source of truth)** untuk seluruh AI Model dan pengembang yang bekerja pada proyek **Cliptzy Desktop** — sebuah aplikasi YouTube Clipper & Auto Uploader yang dibangun menggunakan arsitektur **Murni Rust & Tauri (Native)**.

> ⚠️ **BACA SELURUH DOKUMEN INI SEBELUM MENULIS SATU BARIS KODE PUN.**
> Setiap asumsi yang tidak diverifikasi dari dokumen ini dianggap **halusinasi** dan harus dihindari.

---

## 📐 BAGIAN 0: GROUND TRUTH — STATUS PROYEK SAAT INI

> **Bagian ini WAJIB dibaca terlebih dahulu.**

### 0.1 Apa yang SUDAH ADA (Fakta)

| Komponen                       | Status                | Detail                                                                                                                                                                                                             |
| ------------------------------ | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Tauri + Vue 3 + TypeScript** | ✅ Scaffold dasar     | Proyek Tauri v2 sudah diinisialisasi.                                                                                                                                                                              |
| **Rust Backend (Tauri)**       | ✅ Pengembangan Aktif | Fungsionalitas autentikasi, manajemen config, dan manajemen sesi (Supabase) sudah di-porting ke native Rust (`supabase.rs`).                                                                                       |
| **Video Engine**               | ✅ Selesai            | Ekstraksi Metadata & Stream URL menggunakan native Rust via binding CLI `yt-dlp` (di `youtube.rs`), dengan pengiriman data _frontend_ via IPC Tauri commands.                                                      |
| **Studio Editor UI**           | ✅ Fungsional         | Sinkronisasi mutakhir antara YouTube IFrame Player (Center Cropped 9:16) dan Timeline Editor berbasis durasi klip, Multi-track rendering untuk Teks (Subtitle) & Video, serta Auto-Looping batas pemutaran segmen. |
| **Python Engine (Legacy)**     | 🗑️ Akan Dihapus       | Sebelumnya aplikasi menggunakan Python FastAPI server di-spawn sebagai child process. **Pendekatan ini dibatalkan** demi performa dan kemudahan distribusi. Seluruh logic akan ditulis ulang dalam Rust murni.     |

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
4. **SELALU cek ekosistem sebelum membangun dari awal** — Sebelum menulis modul logika kompleks secara manual, selalu telusuri _Tauri Plugins_ resmi atau _Crates_ Rust di internet. Gunakan solusi komunitas jika memungkinkan untuk mempermudah implementasi.

### 2.2 Larangan Frontend (Vue + TypeScript)

1. **DILARANG menggunakan fetch() ke localhost** — Semua komunikasi dengan logic AI/backend **HARUS** melalui `invoke()` dari `@tauri-apps/api/core`.
2. **State Management** — Hanya gunakan Pinia.

### 2.3 Larangan Rust (Tauri Backend)

1. **Jangan block main thread Tauri** — Gunakan `async` / `await` (`tokio`) untuk semua I/O dan operasi berat (transkripsi, render video).
2. **Error Handling** — Gunakan `Result<T, String>` (via `.map_err(|e| e.to_string())`) untuk diteruskan ke Frontend Tauri. Jangan gunakan `unwrap()` di production path.
3. **Pemisahan Modul (Separation of Concerns)** — DILARANG keras mencampur fungsi (_dirty code_) ke dalam file yang tidak sesuai dengan ranahnya. Contoh: Fungsi untuk membuat file ASS (subtitle/debug) harus diletakkan di `transcription/ass_writer.rs`, jangan dicampur di file orkestrator seperti `clip.rs`. Pastikan struktur kode selalu rapi.

---

## 📋 BAGIAN 3: CHECKLIST AI SEBELUM MULAI BEKERJA

Sebelum menulis kode, AI Model WAJIB menjawab pertanyaan berikut:

1. ✅ Apakah saya sadar bahwa aplikasi ini **100% Native Rust + Vue** dan kita telah membuang Python?
2. ✅ Apakah saya menggunakan Crate Rust (seperti `reqwest`, `tokio`, dsb) atau CLI (seperti memanggil `ffmpeg.exe` via `std::process::Command`) alih-alih server Python?
3. ✅ Apakah semua perintah di-_expose_ ke Frontend melalui `#[tauri::command]`?
4. ✅ Apakah saya sudah menelusuri **Tauri Plugin** resmi atau **Rust Crate** yang relevan di internet sebelum memutuskan untuk menulis ulang fungsionalitas secara manual?

---

## 🛠️ BAGIAN 4: LOG IMPLEMENTASI & PENYELESAIAN MASALAH (WORK LOG)

### 4.1. YouTube Bypass (Error 429 & PO Token)

- **Problem**: YouTube secara agresif memblokir _request_ dari `yt-dlp` menggunakan deteksi Bot, Error 429, dan kewajiban menyertakan PO Token / Visitor Data untuk _client_ Web standar.
- **Solusi**:
  1. Melakukan _spoofing client_ menggunakan kombinasi `android,web,default` via argumen `--extractor-args`.
  2. Mengaktifkan _External JavaScript_ (`--remote-components ejs:github`) yang secara otomatis menggunakan `Node.js` / `Deno` lokal pengguna untuk memecahkan _challenge_ enkripsi YouTube (mengadopsi kesuksesan _engine_ Python lama).
- **Catatan Penting pada `rusty_ytdl`**: Crate `rusty_ytdl` memiliki _bug_ dimana argumen khusus (`self.args`) diabaikan pada fase `fetch_video_infos`. Karenanya, fungsi `analyze_youtube_video` di-refactor menggunakan _raw_ `tokio::process::Command` untuk memanggil `yt-dlp -J` secara independen, memastikan argumen bypass 100% tereksekusi dengan benar dan heatmap JSON bisa diekstrak sempurna.

### 4.2. Manajemen Cookies YouTube Terintegrasi

- **Problem**: Kebutuhan mem-bypass otentikasi YouTube mengharuskan penggunaan file `cookies.txt` yang valid.
- **Solusi**:
  1. Menambahkan tombol "Pilih File Cookies" dan "Test yt-dlp" di halaman `SettingsView.vue` (Profile Section).
  2. Implementasi Tauri command `copy_cookies_file` untuk mengimpor file _cookies_ ke dalam `app_data_dir()/cred/cookies.txt`.
  3. Implementasi command `validate_cookies_file` (Cek format Netscape & kedaluwarsa) serta `test_youtube_cookies` (Membuktikan _cookies_ tersebut valid untuk _fetch_ yt-dlp secara nyata, bukan sekadar tes format).

### 4.3. Optimasi Kecepatan Face Detector (Downscaling)

- **Problem**: Ekstraksi _face keyframes_ menggunakan OpenCV (`rustface`) pada video 1080p memakan waktu sangat lambat (bermenit-menit) karena ukuran matriks gambar yang masif.
- **Solusi**: Menyuntikkan perintah `-vf scale=-1:360` ke dalam _spawn_ FFmpeg saat mengekstrak _frame_ berformat JPEG ke _tempdir_. Proses dekompresi, _I/O storage_, dan kalkulasi wajah menjadi ~9x lipat lebih cepat.
- **Justifikasi**: Algoritma perhitungan koordinat wajah kita sudah ternormalisasi (`cx` dan `cy` berada di interval `0.0 - 1.0`). Hasil deteksi wajah di resolusi 360p bisa di- _mapping_ sempurna kembali ke video _source_ 1080p saat proses _cropping_ tanpa kehilangan keakuratan.

### 4.4. Manajemen Pembatalan Proses (Aggressive Process Killing)

- **Problem**: Ketika _user_ menekan tombol "Batal" di Global Status Bar, proses anak (_child process_) yang di-_spawn_ oleh `tokio` (seperti `ffmpeg` dan `yt-dlp`) kerap menjadi _orphan_ (zombie process) dan terus membebani RAM/CPU.
- **Solusi**: Mengimplementasikan `killall ffmpeg` dan `killall yt-dlp` (`taskkill /F /IM ffmpeg.exe` di Windows) via Command saat proses dibatalkan di `cancel_processing()`. Hal ini menjamin pembatalan bekerja bersih dan tuntas secara OS-level.

### 4.5. Pelaporan Progres Latar Belakang (UI Updates)

- Menambahkan emisi `ProgressEvent` via Tauri `app_handle.emit()` di dalam _looping_ Face Tracker dan modul lainnya. _Frontend_ sekarang menampilkan persentase progres pemrosesan secara _real-time_ di `GlobalStatusBar.vue`.

### 4.6. Deteksi GPU Fisik & Sinkronisasi Hardware Acceleration

- **Problem**: Pengaturan _Hardware Acceleration_ sebelumnya bergantung pada `ffmpeg -hwaccels` yang hanya mengembalikan _flag_ saat di-kompilasi (misal `nvenc` atau `qsv` selalu muncul jika _binary_ mendukungnya, walau GPU fisiknya tidak ada). Hal ini menyebabkan _error_ rendering atau opsi palsu di UI.
- **Solusi**: Mengimplementasi `crate::utils::get_system_gpus()` untuk melakukan _polling_ akurat ke OS level (`powershell WMI` di Windows, `system_profiler` di Mac). Menyamakan validasi OS dengan kapabilitas _binary_ `ffmpeg`, lalu membuat antarmuka Vue (Pinia) agar otomatis melakukan _fallback_ ke mode `CPU` bila GPU yang dikonfigurasi tidak terdeteksi.

### 4.7. Background Monitor Utilisasi GPU Tanpa Blocking (Zero-Cost Sharing)

- **Problem**: Penggunaan _resource_ GPU (`gpu_usage`) di dashboard (_monitor.rs_) selalu nol karena `sysinfo` belum memiliki kapabilitas membaca GPU bawaan.
- **Solusi**: Membuat `std::thread::spawn` yang akan menjalankan perintah `typeperf` secara asinkron di belakang layar (khusus Windows), membaca metrik secara konstan dan menyimpan nilai `max_val` persentase penggunaannya. Agar pengiriman UI tidak menjadi lambat, hasil tersebut disimpan ke `std::sync::atomic::AtomicU32` sehingga `get_system_metrics()` bisa menarik angka secara kilat tanpa _lock_ (zero-cost).

### 4.8. Optimasi Performa I/O dengan Refaktor DRY (Dependency Injection)

- **Problem**: File `AppConfig::load()` dibaca berkali-kali dari disk pada saat _rendering_ panas (seperti saat spawn FFmpeg di `cropper.rs` dan `burner.rs`) yang melanggar prinsip _Don't Repeat Yourself_ (DRY) dan menyebabkan _bottleneck I/O_.
- **Solusi**: Memperbaiki arsitektur aplikasi sehingga _Config_ ditarik satu kali di level orkestrator (`clip.rs`) dan kemudian nilai konfigurasi yang diperlukan (`HwAccel`) dipassing ke bawah secara beruntun (_pass-by-value/reference_) via `OutputConfig` dan `VideoBurnerConfig`. Selain itu, merombak deteksi utilitas murni agar seluruh pemanggilan `tokio::process::Command::new("ffmpeg")` difilter secara ketat melalui implementasi pembaca `find_executable` (_which_ crate).

### 4.9. Sistem Job Management berbasis Video ID & Caching Segmen

- **Problem**: Penggunaan `uuidv4` untuk _job directory_ menyebabkan video mentah (`source.mp4`) didownload ulang setiap kali _user_ merender segmen dari video YouTube yang sama, sehingga sangat boros bandwidth dan waktu.
- **Solusi**: Mengganti _job ID_ menggunakan format `video_id` dari metadata YouTube. File-file intermediet (seperti video sumber, audio WAV, dan _subtitles_) kini dinamai berdasarkan urutan/indeks segmennya (misal `source_1.mp4`, `subtitles_2.ass`). Ini memungkinkan aplikasi menggunakan mekanisme _cache_ sehingga video yang sama tidak diunduh ulang di render klip berikutnya.

### 4.10. Implementasi Multi-Mode Pelacakan Wajah (Face Tracking Modes)

- **Problem**: Proses _face tracking_ menggunakan _Optical Flow_ sangat presisi namun berat, sementara _user_ mungkin menginginkan opsi yang lebih cepat atau statis.
- **Solusi**: Menambahkan opsi dropdown "Metode Pelacakan Wajah" di UI Inspector yang tersinkronisasi permanen ke file konfigurasi `AppConfig` Rust:
  1. **Sinematik (Mulus & Lambat)**: Memakai `optical-flow-lk` dengan _Exponential Moving Average_ untuk gerakan kamera yang halus.
  2. **Dinamis (Standard AI)**: Memakai `rustface` murni untuk deteksi presisi namun statis yang diekstrak per detik (_fast_).
  3. **Statis (Kunci Posisi Awal)**: Mengekstrak eksklusif _1 frame_ pertama saja (`-vframes 1`) untuk mencari wajah dan menguncinya selamanya guna kecepatan render ultra-tinggi.

### 4.11. Optimasi Ukuran File Output FFmpeg (Bitrate & CRF Control)

- **Problem**: FFmpeg melakukan _re-encoding_ (crop, burn subtitle) tanpa batas _bitrate_ yang jelas, menyebabkan output membengkak ekstrem (contoh: _source_ 20MB menjadi _output_ 204MB dengan _bitrate_ di atas 20.000 kbps).
- **Solusi**: Menyuntikkan limitasi ukuran secara proaktif di `HwAccel::encode_args()` pada `hwaccel.rs`.
  - Untuk _CPU Encoder_ (`libx264`): Menggunakan `-crf 26` dipadukan dengan limitasi maksimum `-maxrate 4000k` dan `-bufsize 8000k`.
  - Untuk _Hardware Encoder_ (NVENC, QSV, AMF): Secara paksa menggunakan target `-b:v 3000k` dengan `-maxrate 4000k`. Ini akan menekan ukuran file tetap kecil (cocok untuk distribusi Shorts/Reels) tanpa kompromi kualitas yang tampak.

### 4.12. Penyatuan Sistem Warna Teks (Dual Theme)

- **Problem**: Warna teks sering menyatu dengan _background_ (tidak kontras) karena penggunaan kelas Tailwind yang di-_hardcode_ secara statis (seperti `text-gray-900` dan `dark:text-white`) di puluhan komponen Vue.
- **Solusi**: Mengganti seluruh kelas _hardcoded_ dengan CSS variable semantik (`text-[var(--color-text-main)]` dan `text-[var(--color-text-muted)]`) menggunakan _script_ Node otomatis. Ini menjamin kontras warna yang nyaman untuk dilihat (_Spatial Bento Box_) baik di mode Light Pastel maupun Slate Dark secara dinamis.

### 4.13. Standarisasi Komponen Primitif UI (Menolak Nuxt UI)

- **Problem**: Ketidakkonsistenan elemen antarmuka mentah (seperti _hover scale_ pada tombol yang berbeda-beda) memicu wacana untuk bermigrasi secara masif ke _framework_ eksternal seperti Nuxt UI yang akan mencederai _ground rules_ (tanpa _library_ tambahan).
- **Solusi**:
  1. Membuat komponen **`BaseButton.vue`** yang kokoh dengan dukungan _loading state_, _variants_ (primary, secondary, danger, ghost), dan transisi _scale hover_ yang seragam.
  2. Mendaftarkan `BaseButton`, `SpatialInput`, dan `RangeSlider` secara global di `main.ts` agar bisa dipakai tanpa instruksi impor berulang.
  3. Memperbaiki _hardcoded background_ pada `SpatialInput.vue` dan `RangeSlider.vue` agar mematuhi aturan _Dual Theme_.

### 4.14. Ekstraksi Komponen ScanResultCard & Perbaikan peer-checked

- **Problem**:
  1. Kode iterasi daftar hasil _scan_ (AI & Heatmap) berulang (_duplicate_) secara panjang lebar di `SourceSegmentsPanel.vue`.
  2. Implementasi gaya Tailwind `peer-checked:opacity-100` gagal menargetkan elemen `<IconCheck>` karena spesifikasi hierarki penyeleksi turunan saudara (_descendant of a sibling_) di Tailwind yang kaku.
- **Solusi**: Mengabstraksi keseluruhan baris menjadi komponen `ScanResultCard.vue` dan menggunakan reaktivitas _state_ Vue (`:class`) ketimbang CSS _pseudo-selectors_, membuat panel sumber segmen jauh lebih pendek dan elegan.

### 4.15. Perbaikan Dinamis Padding & Minimal Durasi Klip

- **Problem**: Pengaturan padding dan durasi minimum di UI tidak mengubah hasil akhir klip karena segmen dipotong secara statis mengikuti data analisis awal (heatmap). Terjadi bug pada padding bernilai negatif akibat definisi variabel Rust bertipe `u32`.
- **Solusi**: Mengubah tipe `padding` menjadi `i32` di `models.rs`, dan mengimplementasikan injeksi ulang perhitungan `start`/`end` langsung pada `ClipVideoUseCase::execute`. Sistem sekarang secara otomatis memekarkan klip (_deficit allocation_) beserta penjagaan batas (_underflow shifting_) ke ujung lawannya jika titik potong awal melampaui detik `0.0`.

### 4.16. Penyelamatan Kompilasi GitHub Actions & Pemendekan Path Windows

- **Problem**: Path kompilasi `whisper-rs-sys` pada OS Windows melampaui limit _MAX_PATH_ (260 karakter), dan setelan _default feature_ Vulkan menyebabkan kegagalan build otomatis pada _runner_ Mac/Ubuntu yang tidak menginstal Vulkan SDK.
- **Solusi**:
  1. Menerapkan pengalihan `target-dir = "../t"` pada _local cache_ khusus lingkungan Windows.
  2. Modifikasi `release.yml` GitHub Actions untuk menghilangkan _default features_ secara global, lalu mem-_passing_ argumen spesifik `--features gpu-metal` khusus macOS dan menginjeksi dependensi GitHub Action `humbletim/install-vulkan-sdk` agar _runner_ Windows/Ubuntu dapat meng- _compile_ _shader_ Vulkan secara mulus demi 100% stabilitas rilis distribusi.

### 4.17. Standarisasi Logging Terpusat via Tauri Plugin

- **Problem**: Implementasi logging sebelumnya dilakukan secara manual menggunakan kombinasi `tracing-subscriber` dan `tracing-appender`, serta kurang terintegrasi secara praktis dengan konsol webview frontend.
- **Solusi**:
  1. Menghapus manajemen log manual dan menggantinya dengan pustaka resmi `tauri-plugin-log` (v2).
  2. Mengganti seluruh pemanggilan macro `tracing::info!` dan kawan-kawannya di seluruh _codebase_ Rust menjadi `log::info!` (pustaka standar `log`), serta mencabut dependensi `tracing` sepenuhnya. Hal ini dikarenakan `tauri-plugin-log` berintegrasi secara _native_ dan menangkap event dari _facade_ `log` untuk ditulis ke file dan diteruskan ke _webview_.
  3. Memanggil fungsi `attachConsole()` di dalam `main.ts` Vue untuk meneruskan seluruh riwayat logging backend ke antarmuka _Developer Tools_ browser tanpa merusak integrasi pelaporan progres `GlobalStatusBar.vue` yang berjalan murni via Tauri Event Emitter.

### 4.18. Peningkatan Akurasi Deteksi Emosi (ViT) & Modularisasi ONNX

- **Problem**: Model ONNX FERPlus (30MB) yang sebelumnya digunakan untuk mendeteksi emosi wajah sangat tidak akurat dalam menangani variasi mikro-ekspresi _Youtuber Gaming_ karena arsitektur lawas dan limitasi input gambar yang sangat kecil (64x64 Grayscale). Selain itu, logika pengunduhan dan manajemen sesi ort::session::Session terkunci keras di dalam visual.rs, menyulitkan ekspansi fitur AI di masa depan.
- **Solusi**:
  1. Melakukan migrasi model ke **Vision Transformer (ViT)** (Xenova/facial_emotions_image_detection berukuran ~330MB) yang dilatih menggunakan dataset modern.
  2. Menyesuaikan ulang pipa pra-pemrosesan (_preprocessing_) gambar di Rust agar mengubah _frame_ menjadi ukuran 224x224 RGB, menerapkan normalisasi mean/std ImageNet, dan me-_mapping_ output label secara akurat.
  3. Mengabstraksi logika _download_, _hardware acceleration_ (DirectML), dan _caching session_ ke dalam _struct_ mandiri OnnxModelManager pada crate::ai::onnx. Hal ini membuat seluruh model AI berformat .onnx di masa depan (seperti _SpeechBrain_ atau _Voice Cloning_) dapat di-_spawn_ hanya dengan satu baris instansiasi global.
