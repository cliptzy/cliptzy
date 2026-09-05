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
| **Rust Backend (Tauri)**       | ✅ Pengembangan Aktif | Autentikasi, config, sesi Supabase (`supabase.rs`), pipeline render klip (`orchestrator/clip/`), kompilasi reaksi (`orchestrator/compilation/`), transkripsi Whisper, face tracking, dan crop/burn FFmpeg — semuanya native Rust. |
| **Video Engine**               | ✅ Selesai            | Ekstraksi Metadata & Stream URL menggunakan native Rust via binding CLI `yt-dlp` (di `youtube.rs`), dengan pengiriman data _frontend_ via IPC Tauri commands.                                                      |
| **Arsitektur Orkestrator**     | ✅ Modular            | File monolitik (`compilation.rs`, `clip.rs`, `cropper.rs`, `utils.rs`) sudah dipecah menjadi folder modul per _concern_ / fase pipeline.                                                                          |
| **Studio Editor UI**           | ✅ Fungsional         | Sinkronisasi mutakhir antara YouTube IFrame Player (Center Cropped 9:16) dan Timeline Editor berbasis durasi klip, Multi-track rendering untuk Teks (Subtitle) & Video, serta Auto-Looping batas pemutaran segmen. |
| **Python Engine (Legacy)**     | 🗑️ Akan Dihapus       | Sebelumnya aplikasi menggunakan Python FastAPI server di-spawn sebagai child process. **Pendekatan ini dibatalkan** demi performa dan kemudahan distribusi. Seluruh logic akan ditulis ulang dalam Rust murni.     |

### 0.2 Apa yang BELUM ADA (Belum Diimplementasi)

Logika Python lama sudah sebagian besar digantikan Rust native. Sisa pekerjaan utama:

- Auto-uploader ke YouTube/TikTok/Instagram (modul `uploaders/` masih dalam pengembangan).
- Integrasi penuh UI untuk semua command backend baru (mis. `is_supabase_available` belum dipanggil frontend).
- Penghapusan total _legacy Python engine_ (`src-tauri/engine/`) setelah audit final.

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
3. **Desain Sistem (STRICT)** — WAJIB mematuhi `UI_DESIGN.md`. **DILARANG** menggunakan radius melengkung (`rounded-md`, `rounded-lg`, `rounded-full`), semua komponen antarmuka harus tajam (`rounded-none`). **DILARANG** menggunakan palet warna sembarangan (seperti `bg-blue-500`, `text-red-400`); selalu gunakan semantic tokens dari daisyUI (`base-200`, `primary`, `secondary`, `accent`, `error`). **DILARANG** menggunakan animasi transisi ukuran berlebihan (`hover:scale-110`).

### 2.3 Larangan Rust (Tauri Backend)

1. **Jangan block main thread Tauri** — Gunakan `async` / `await` (`tokio`) untuk semua I/O dan operasi berat (transkripsi, render video).
2. **Error Handling** — Gunakan `Result<T, CliptzyError>` untuk command Tauri baru. `CliptzyError` mengimplementasikan `Serialize` sebagai string sehingga frontend tetap bisa menangkap error via `catch`. Jangan gunakan `unwrap()` di production path.
3. **Pemisahan Modul (Separation of Concerns)** — DILARANG keras mencampur fungsi (_dirty code_) ke dalam file yang tidak sesuai dengan ranahnya. Contoh: Fungsi untuk membuat file ASS (subtitle/debug) harus diletakkan di `transcription/ass_writer.rs`, jangan dicampur di file orkestrator seperti `orchestrator/clip/`. Pastikan struktur kode selalu rapi.

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
- **Solusi**: Mengimplementasi `crate::system::gpu::get_system_gpus()` (sebelumnya di `utils.rs`) untuk melakukan _polling_ akurat ke OS level (`powershell WMI` di Windows, `system_profiler` di Mac). Menyamakan validasi OS dengan kapabilitas _binary_ `ffmpeg`, lalu membuat antarmuka Vue (Pinia) agar otomatis melakukan _fallback_ ke mode `CPU` bila GPU yang dikonfigurasi tidak terdeteksi.

### 4.7. Background Monitor Utilisasi GPU Tanpa Blocking (Zero-Cost Sharing)

- **Problem**: Penggunaan _resource_ GPU (`gpu_usage`) di dashboard (_monitor.rs_) selalu nol karena `sysinfo` belum memiliki kapabilitas membaca GPU bawaan.
- **Solusi**: Membuat `std::thread::spawn` yang akan menjalankan perintah `typeperf` secara asinkron di belakang layar (khusus Windows), membaca metrik secara konstan dan menyimpan nilai `max_val` persentase penggunaannya. Agar pengiriman UI tidak menjadi lambat, hasil tersebut disimpan ke `std::sync::atomic::AtomicU32` sehingga `get_system_metrics()` bisa menarik angka secara kilat tanpa _lock_ (zero-cost).

### 4.8. Optimasi Performa I/O dengan Refaktor DRY (Dependency Injection)

- **Problem**: File `AppConfig::load()` dibaca berkali-kali dari disk pada saat _rendering_ panas (seperti saat spawn FFmpeg di `cropper.rs` dan `burner.rs`) yang melanggar prinsip _Don't Repeat Yourself_ (DRY) dan menyebabkan _bottleneck I/O_.
- **Solusi**: Memperbaiki arsitektur aplikasi sehingga _Config_ ditarik satu kali di level orkestrator (`orchestrator/clip/mod.rs`) dan kemudian nilai konfigurasi yang diperlukan (`HwAccel`) dipassing ke bawah secara beruntun (_pass-by-value/reference_) via `OutputConfig` dan `VideoBurnerConfig`. Selain itu, merombak deteksi utilitas murni agar seluruh pemanggilan `tokio::process::Command::new("ffmpeg")` difilter secara ketat melalui implementasi pembaca `find_executable` (_which_ crate).

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

### 4.12. Penyatuan Sistem Warna dan Estetika Terminal (Dark-First)

- **Problem**: Inkonsistensi warna dan gaya membulat (_rounded_ / _bento_) di banyak komponen sebelumnya menyulitkan perombakan desain secara merata, serta kurang mencerminkan nuansa aplikasi teknis profesional.
- **Solusi**: Mengadopsi pedoman desain `UI_DESIGN.md` yang ketat (Terminal-tool aesthetic). Mengganti seluruh kelas _hardcoded_ dengan sistem token bawaan daisyUI (`base-100`, `base-content`, `accent`, dll). Seluruh _primitive components_ kini mengimplementasikan ujung tajam absolut (`rounded-none` / 0px radius) dan menghilangkan warna _vivid_ agar tidak mendistraksi mata (kecuali pada elemen kritis).

### 4.13. Standarisasi Komponen Primitif UI (Pure Vue + Tailwind)

- **Problem**: Kebutuhan komponen yang solid sempat memicu wacana penggunaan _framework_ eksternal seperti Nuxt UI atau penggunaan variabel CSS manual yang berantakan.
- **Solusi**:
  1. Menolak penggunaan library komponen eksternal sepenuhnya.
  2. Merancang primitif kustom (`CButton.vue`, `CInput.vue`, `CCard.vue`, `CSlider.vue`, dsb.) yang membungkus token daisyUI secara efisien.
  3. Mendaftarkan komponen-komponen utama secara global di `main.ts` agar dapat diakses bersih tanpa impor berulang di seluruh _view_.

### 4.14. Ekstraksi Komponen ScanResultCard & Perbaikan peer-checked

- **Problem**:
  1. Kode iterasi daftar hasil _scan_ (AI & Heatmap) berulang (_duplicate_) secara panjang lebar di `SourceSegmentsPanel.vue`.
  2. Implementasi gaya Tailwind `peer-checked:opacity-100` gagal menargetkan elemen `<IconCheck>` karena spesifikasi hierarki penyeleksi turunan saudara (_descendant of a sibling_) di Tailwind yang kaku.
- **Solusi**: Mengabstraksi keseluruhan baris menjadi komponen `ScanResultCard.vue` dan menggunakan reaktivitas _state_ Vue (`:class`) ketimbang CSS _pseudo-selectors_, membuat panel sumber segmen jauh lebih pendek dan elegan.

### 4.15. Perbaikan Dinamis Padding & Minimal Durasi Klip

- **Problem**: Pengaturan padding dan durasi minimum di UI tidak mengubah hasil akhir klip karena segmen dipotong secara statis mengikuti data analisis awal (heatmap). Terjadi bug pada padding bernilai negatif akibat definisi variabel Rust bertipe `u32`.
- **Solusi**: Mengubah tipe `padding` menjadi `i32` di `models.rs`, dan mengimplementasikan injeksi ulang perhitungan `start`/`end` langsung pada `ClipVideoUseCase::execute` di `orchestrator/clip/mod.rs`. Sistem sekarang secara otomatis memekarkan klip (_deficit allocation_) beserta penjagaan batas (_underflow shifting_) ke ujung lawannya jika titik potong awal melampaui detik `0.0`.

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

### 4.19. Kurasi Konfigurasi Subtitle & Sinkronisasi Settings ↔ Studio Inspector

- **Problem**: Pengaturan subtitle di `SubtitleSection.vue` dan `InspectorPanel.vue` membingungkan karena tiga kontrol tumpang tindih (`style`, `animation`, `border_style`). Field `subtitle.style` ada di `config.json` tetapi **tidak dipakai** backend ASS (`ass_writer.rs` hanya membaca `animation` + `border_style`). Font masih berupa input teks bebas, warna masih format ASS mentah, dan UI kedua lokasi tidak selaras.
- **Solusi**:
  1. Membuat **single source of truth** frontend di `src/constants/subtitle.ts` (preset, font, lokasi, konversi warna ASS ↔ hex) dan komponen bersama `SubtitleStyleControls.vue` + `AssColorPicker.vue`.
  2. Mengganti tiga kontrol terpisah menjadi satu **Preset Gaya**: Plain (`animation=none`, `border_style=1`), Hormozi, Karaoke, Brutalist Box (`border_style=3`). Field `style` legacy tetap disinkronkan saat preset dipilih agar kompatibel dengan `config.json` lama.
  3. Font diubah menjadi **dropdown** (Arial, Impact, Bangers, Inter, TheBoldFont, Courier New). Warna teks & background memakai **color picker** native dengan konversi otomatis ke format ASS `&HAABBGGRR`; background mendukung slider opasitas.
  4. `PreviewPanel.vue` memakai utilitas warna yang sama agar preview real-time konsisten dengan output render.

### 4.20. Toggle Burn Watermark & Burn Subtitle (Preview + Pipeline Render)

- **Problem**: Watermark dan subtitle selalu ditampilkan di preview Studio dan selalu di-burn saat render, tanpa opsi mematikan salah satunya secara independen dari aset/transkripsi yang ada.
- **Solusi**:
  1. Menambahkan field config `burn_watermark: bool` dan `burn_subtitle: bool` (default `true`) di `models.rs`, `defaults.rs`, dan `settings.ts`.
  2. **Inspector Studio** — kartu Branding: toggle **Burn Watermark**; kartu Kustomisasi Subtitle: toggle **Burn Subtitle**. Kontrol terkait (posisi watermark / gaya subtitle) dinonaktifkan saat toggle off.
  3. **Preview** — overlay watermark/subtitle disembunyikan jika toggle masing-masing `false`.
  4. **Backend** — `clip/mod.rs` dan `clip/subtitle.rs` hanya memproses watermark/subtitle jika flag burn aktif. `TimelinePanel.vue` meneruskan `use_subtitle` dari `burn_subtitle`.
  5. **Pemisahan tanggung jawab**: `subtitle.enabled` = Whisper AI / generate transkripsi; `burn_subtitle` = preview + burn ASS ke video output.

### 4.21. Cache Global Restreamer (Fingerprint DB Lintas Match / Job)

- **Problem**: Setiap match/VOD utama memakai `job_dir` berbeda (`jobs/{video_id}/`). Saat meng-clip match 2 dari live stream yang sama, aplikasi mengunduh ulang audio restreamer 8 jam dan membangun ulang fingerprint database (`build_or_load_fingerprint_db`) — proses paling lambat — meskipun restreamer VOD identik.
- **Solusi**:
  1. Modul baru `orchestrator/restreamer_cache.rs` menyimpan aset restreamer secara **global per `restreamer_video_id`**:
     ```
     AppData/cache/restreamers/{restreamer_id}/
     ├── audio.m4a
     ├── audio_16k.wav
     ├── fingerprint.bin      ← generate_db sekali, dipakai ulang
     └── sync/{main_video_id}_{moments_hash}.json
     ```
  2. `audio_sync.rs` memakai path global tersebut, bukan `job_dir`. Hasil sinkronisasi per kombinasi video utama + momen epik tetap di-cache terpisah.
  3. **Migrasi otomatis** dari cache lama di `jobs/{video_id}/` ke cache global saat file legacy terdeteksi.

### 4.22. Pencarian Restreamer Multi-Query (Judul Clickbait / Non-Literal)

- **Problem**: Pencarian restreamer hanya memakai satu kueri `ytsearch` berdasarkan judul VOD resmi (mis. `AE vs RRQ`). Restreamer Indonesia sering memakai judul clickbait yang tidak menyebut kedua tim secara literal (mis. _"Nobar MPL ID S18"_, _"Akankah AE wangi hari ini?"_). Parameter `search_keywords` dari frontend juga diabaikan (`_custom_keywords`).
- **Solusi**:
  1. **Ekspansi heuristik** — dari judul match diekstrak tim (`vs`), liga (`MPL`, `MSC`, `MDL`), lalu dibuat hingga 8 kueri: `{timA} {timB} nobar`, `live reaction`, `reaksi`, `{liga} nobar`, dll.
  2. **Ekspansi AI opsional** — jika provider AI dikonfigurasi, AI menghasilkan ~6 kueri tambahan yang realistis untuk konteks nobar/reaksi MLBB Indonesia.
  3. Pencarian dijalankan **bertahap**; hasil digabung & deduplikasi per channel/video ID; berhenti setelah ~10 kandidat. Filter durasi (>1 jam) dan jendela tanggal upload tetap berlaku.
  4. Cache pencarian (`restreamer_search.json`) memakai `queries_hash` agar invalidasi akurat saat kueri berubah.
  5. **UI Studio** — field kata kunci restreamer ditampilkan juga di mode Reaksi (`AE RRQ MPL S18`), tidak hanya mode meme shorts.
  6. Verifikasi akhir tetap via **audio fingerprinting** saat sinkronisasi — judul clickbait tidak masalah selama audionya cocok.

### 4.23. Refactor Audit Backend — Modularisasi `orchestrator/compilation/`

- **Problem**: File `orchestrator/compilation.rs` (~1800 baris) menjadi _god file_ yang mencampur model data, helper, ekstraksi audio, deteksi momen epik, pencarian restreamer, sinkronisasi audio, dan clipping.
- **Solusi**: Memecah menjadi folder `orchestrator/compilation/`:
  ```
  orchestrator/compilation/
  ├── mod.rs              # PrepareCompilationUseCase + execute orchestration
  ├── models.rs           # EpicMoment, RestreamerInfo, cache entries
  ├── helpers.rs          # emit_stage, utilitas bersama
  ├── audio_extraction.rs
  ├── moment_detection.rs
  ├── restreamer_search.rs
  ├── audio_sync.rs
  └── clipping.rs
  ```
  API publik (`PrepareCompilationResult`, `EpicMoment`, dll.) di-_re-export_ dari `mod.rs`. Command `prepare_compilation` / `execute_compilation` di `commands/video.rs` tidak berubah.

### 4.24. Pemisahan `utils.rs` & Modularisasi Dependensi Sistem

- **Problem**: `utils.rs` menjadi tempat pembuangan kode yang tidak sesuai ranahnya (`AppDependencies`, deteksi GPU, browser, unduhan model ONNX).
- **Solusi**:
  1. `AppDependencies` → `deps/manager.rs`
  2. `get_system_gpus()` → `system/gpu.rs`
  3. `get_installed_browsers_list()` → `system/browser.rs`
  4. `ensure_model_downloaded()` → `ai/onnx.rs`
  5. `utils/` kini hanya berisi: `find_executable`, `kill_processes`, `date`
  6. Helper ASS (`build_render_config`, `apply_brutalist_box_style`, `try_generate_emotion_debug_ass`) dipindah ke `transcription/ass_writer.rs`

### 4.25. Refactor Face Tracker, Segment Audio & Perampingan Commands

- **Problem**: `face/tracker.rs` terlalu besar; logika analisis audio segmen bercampur di `commands/video.rs`; I/O WAV tersebar.
- **Solusi**:
  1. `face/tracker.rs` dipecah: `face/frame_extractor.rs`, `face/tracker_strategy.rs`, `face/tracker.rs` (orkestrasi tipis).
  2. `decode_wav` / `write_wav_segment` → `transcription/audio.rs`
  3. Logika `analyze_segment_audio` → `orchestrator/segment_audio.rs` (`AnalyzeSegmentAudioUseCase`)
  4. `commands/video.rs` menjadi _thin router_ dengan helper `build_pipeline_context()` — hanya meneruskan ke use case yang sesuai.

### 4.26. Modularisasi Cropper (Strategy Pattern)

- **Problem**: `processing/cropper.rs` monolitik dengan banyak mode crop dalam satu file.
- **Solusi**: Folder `processing/cropper/`:
  ```
  processing/cropper/
  ├── mod.rs           # create_crop_strategy() factory
  ├── default.rs
  ├── full.rs
  ├── full_face.rs
  ├── center_face.rs
  └── passthrough.rs
  ```
  Setiap mode crop adalah strategi terpisah yang mengimplementasikan trait `CropStrategy`.

### 4.27. Supabase Graceful Offline & Ekstraksi OAuth

- **Problem**: Aplikasi _panic_ saat startup jika env Supabase tidak dikonfigurasi; logika OAuth TCP listener bercampur di `supabase.rs`.
- **Solusi**:
  1. `SupabaseClient::offline()`, `is_available()`, `require_available()` — `lib.rs` tidak lagi panic; mode offline aman.
  2. OAuth TCP listener diekstrak ke `auth/oauth_server.rs` + `auth/mod.rs`
  3. Command baru `is_supabase_available` (belum dipakai UI, tersedia untuk deteksi offline di masa depan).

### 4.28. Standarisasi Error Supabase via `CliptzyError`

- **Problem**: Error Supabase masih berupa `String` atau `unwrap()` di beberapa path; tidak konsisten dengan error type terpusat.
- **Solusi**:
  1. Menambahkan variant `CliptzyError::Supabase(String)` di `error.rs`
  2. Seluruh method publik `supabase.rs` mengembalikan `Result<_, CliptzyError>`
  3. `commands/auth.rs` dan `commands/sync.rs` mengembalikan `CliptzyError` (bukan `String`)
  4. `CliptzyError` mengimplementasikan `serde::Serialize` sebagai string agar frontend tetap kompatibel via `catch (err)`

### 4.29. Modularisasi `orchestrator/clip/` (Folder per Fase Pipeline)

- **Problem**: Setelah refactor fase di `clip.rs`, file masih ~750 baris dan sulit dirawat — tidak konsisten dengan struktur `orchestrator/compilation/`.
- **Solusi**: Memecah `orchestrator/clip.rs` menjadi folder:
  ```
  orchestrator/clip/
  ├── mod.rs          # ClipVideoUseCase + execute() + re-export publik
  ├── models.rs       # ClipPayload, ClipResult, ClipPaths, cache entries
  ├── helpers.rs      # apply_segment_bounds, clip_paths, probe_output_dimensions
  ├── download.rs     # download_phase
  ├── emotion.rs      # emotion_phase
  ├── crop.rs         # crop_phase + face keyframes + debug ASS
  ├── subtitle.rs     # subtitle_phase + transkripsi/cache
  └── finalize.rs     # stack_phase + thumbnail_phase
  ```
  - `impl ClipVideoUseCase` tersebar per modul fase (pola Rust valid).
  - API publik tetap: `ClipPayload`, `ClipResult`, `ClipVideoUseCase` — `commands/video.rs` dan `TimelinePanel.vue` tidak perlu diubah.
  - Event progress `clip-progress` tetap dipancarkan via `orchestrator/pipeline.rs::emit_progress`.

### 4.30. Verifikasi Integritas IPC Frontend ↔ Backend

- **Konteks**: Setelah seluruh refactor audit, perlu dipastikan `src/` (Vue) masih berkomunikasi dengan backend.
- **Hasil Verifikasi** (Agustus 2026):
  1. `cargo check` (backend) dan `npm run build` (frontend) — **lulus tanpa error**.
  2. Semua `invoke()` di `src/` terdaftar di `lib.rs::invoke_handler` (25+ command).
  3. Payload `clip_video` dari `TimelinePanel.vue` (snake_case) cocok dengan `ClipPayload` Rust.
  4. Tauri v2 otomatis memetakan camelCase JS ↔ snake_case Rust untuk argumen command (`videoUrl` → `video_url`, dll.).
  5. Event `clip-progress` didengarkan `GlobalStatusBar.vue` — progres render real-time tetap berfungsi.

### 4.31. Implementasi Lanjutan Strategy Cropper & B-Roll UI

- **Konteks**: Telah dilakukan penambahan berbagai mode pemotongan video (*crop modes*) tingkat lanjut seperti `SplitFaceCrop`, `FullFaceCrop`, `MultiFaceCrop`, `SplitBrollCrop`, dan `PassthroughCrop` tanpa ada pembaharuan dokumentasi yang memadai sebelumnya.
- **Backend**:
  1. Penambahan file logika modul di `processing/cropper/` dan abstraksi factory `create_crop_strategy`.
  2. Implementasi `broll_manager.rs` untuk memilih klip stok latar secara acak dari direktori B-roll.
  3. Modifikasi fungsi `get_two_faces_normalized_centers` pada `face/tracker.rs` khusus untuk memfasilitasi kebutuhan podcast dua arah (`MultiFaceCrop`).
- **Frontend**:
  1. Pembuatan `BrollAssetsSection.vue` dalam layar *Settings* (disinkronisasi ke PINIA `broll_dir`).
  2. Ekstraksi spesifikasi metadata *crop mode* (`requiresFaces`, `requiresBroll`, icon, deskripsi) ke `src/constants/cropModes.ts`.
  3. Modifikasi `InspectorPanel.vue` guna memunculkan pesan peringatan UI dinamis tergantung syarat tiap mode.

### 4.32. Integrasi Cek & Instalasi Otomatis yt-dlp via Crate `yt-dlp`

- **Problem**: Panel "Dependensi Eksternal" di Settings (`EngineSection.vue`) sebelumnya hanya memeriksa dan memasang FFmpeg dan Deno. `yt-dlp` belum tercatat dalam UI maupun didukung instalasi otomatisnya, padahal `yt-dlp` adalah dependensi krusial untuk scraping video YouTube.
- **Solusi**:
  1. Memanfaatkan crate `yt-dlp` yang sudah ada di dependensi Rust (`yt_dlp::client::deps::LibraryInstaller`) untuk mengunduh binary resmi `yt-dlp` langsung dari GitHub release sesuai platform/arsitektur pengguna ke direktori `app_data_dir()/bin`.
  2. Memperluas struct `DependencyStatus` dengan field `ytdlp_installed: bool` dan `ytdlp_version: String`, serta mengoptimalkan fungsi `check_dependencies` di `deps/manager.rs` agar mendeteksi keberadaan binary di PATH maupun folder bin lokal aplikasi via `find_executable`.
  3. Mengekspos command baru `install_ytdlp` di `deps/manager.rs` dan mendaftarkannya di `lib.rs::invoke_handler` serta menyertakannya dalam pipeline `install_dependencies`.
  4. Menambahkan card `yt-dlp` di `EngineSection.vue` dengan tombol aksi responsif ("Pasang" / "Perbarui" / status checkmark) dan sinkronisasi progress instalasi secara real-time.

### 4.33. Integrasi Agen AI & Penanganan Bug rig-core (Custom Provider)

- **Problem**: Pengguna mendapati error `JsonError` ketika menjalankan test Agen AI di UI, model dropdown di settings sering hilang, serta command `ask_agent` mengalami panic pada state management.
- **Solusi**:
  1. **Perbaikan Tauri State**: Fungsi `ask_agent` memicu panic karena mencoba menarik `AppConfig` melalui `tauri::State`. Ini diperbaiki dengan memuat config langsung dari disk (`AppConfig::load()`), sejalan dengan arsitektur Rust kita.
  2. **Perbaikan UI Vue Lifecycle**: Model yang sedang aktif terhapus dari Vuex/Pinia state saat dropdown kosong. Diperbaiki di `AISection.vue` dengan menginjeksi opsi statis untuk nilai yang `tersimpan` (`<option :value="...">...</option>`) jika model belum di-fetch dari API.
  3. **Injeksi `stream: false` pada reqwest (rig-core)**: Proxy kustom milik pengguna memaksa *Server-Sent Events (SSE)* streaming karena rig-core mengomit atribut stream. Memperbaikinya dengan memodifikasi builder: `.additional_params(serde_json::json!({ "stream": false }))`, sehingga proxy mengembalikan format JSON biasa.
  4. **Migrasi API Legacy (rig-core 0.42.0)**: Update terbaru `rig-core` merubah endpoint default ke `Responses API` (mengharap `object: "response"`). Namun, mayoritas proxy masih memakai `Chat Completions` API (`chat.completion`). Diperbaiki dengan mengikat `client.completions_api()` saat inisialisasi Client builder.
  5. **Smart Fallback (Tanpa Tool)**: Menambahkan blok retry otomatis pada pemanggilan LLM. Jika AI Proxy gagal memproses parameter `tools` (biasanya terjadi karena model/proxy tidak support spesifikasi *Function Calling* milik OpenAI), aplikasi akan seketika mengulang request secara polos (hanya Prompt biasa) dan orchestrator `moment_detection.rs` akan fallback menggunakan `extract_json_array` untuk mengambil output.


### 4.34. Perbaikan Bias Pusat Pelacakan Wajah (Face Tracking Offset)

- **Problem**: Pengguna mendapati bahwa wajah di video akhir sedikit melenceng ke kanan, serta pada mode `split_face`, wajah terlalu kecil di paruh bawah kanvas.
- **Solusi**:
  1. **Pusat Geometris Wajah**: Daripada menerapkan offset statis yang bergantung pada dimensi kanvas, perbaikan diterapkan langsung di *engine* pelacakan wajah (`face/tracker.rs` & `face/tracker_strategy.rs`). Bounding box dari *SeetaFace/rustface* sering mencakup kuping/rambut. Oleh karena itu, kita menggeser _center_ X (`cx`) menjadi **55% dari lebar bounding box** (`bbox.x() + bbox.width() * 0.55`) untuk memastikan target terpusat presisi pada fitur tengah wajah (hidung).
  2. **Skala Mode `split_face`**: Memodifikasi _filter graph_ FFmpeg di `split_face.rs` untuk menerapkan _zoom_ ekstra (**1.3x**) khusus pada video `split_bottom`, dan **1.2x** pada `full_face.rs`, sehingga wajah terlihat lebih besar dan proporsional.

### 4.35. Perbaikan Kompilasi Lintas OS (Cross-Compilation macOS) & Dead Code

- **Problem**: Ketika pengguna melakukan _build_ untuk target macOS Apple Silicon (`aarch64-apple-darwin`) via GitHub Actions, _compiler_ Rust mengalami _error_ `file not found for module models` pada `src/ai/mod.rs` serta _warning_ `unreachable expression` di `hwaccel.rs`.
- **Solusi**:
  1. **Case-Sensitivity File System**: Menghapus deklarasi `pub mod models;` yang memanggil direktori kosong (`src/ai/models`) karena tidak digunakan. Hal ini menyelesaikan _error_ di OS dengan file system *case-sensitive* (seperti Linux/macOS runner) yang sensitif terhadap perubahan *case* folder Git.
  2. **Unreachable Code**: Memperbaiki makro kondisional kompilasi di `hwaccel.rs` menggunakan `#[cfg(not(target_os = "macos"))]` pada cabang *fallback* CPU, sehingga *compiler* macOS tidak membuang ekspresi tersebut sebagai kode mati (_dead code_).

### 4.36. Single Source of Truth (SSOT) Model ONNX & Command Manajemen UI

- **Problem**: Model ONNX (ViT, Wav2Vec2, AST, RoBERTa, SeetaFace) sebelumnya memiliki URL hardcoded tersebar di 5+ file analyzer (`visual.rs`, `voice.rs`, `audio.rs`, `text.rs`, `face/tracker.rs`). Tidak ada sinkronisasi metadata (nama, kategori, ukuran) antara backend dan frontend. UI Settings tidak memiliki halaman manajemen model (list/download/delete). Bug: tokenizer RoBERTa mengunduh file salah (`model.onnx` bukan `tokenizer.json`).

### 4.37. Perbaikan Job History Dashboard & Auto-Refresh (Event-Driven)

- **Problem**: Aktivitas terakhir di `DashboardView.vue` tidak muncul saat user memproses clip video. Root cause:
  1. Command `scan_video` (dipanggil oleh `previewVideo` di Studio) **tidak pernah** memanggil `upsert_job_history`, sehingga job tidak tersimpan saat URL dimuat.
  2. Composable `useJobHistory()` dipanggil manual via `loadHistory()` di `onMounted` setiap komponen, sehingga tidak reaktif lintas komponen/halaman.
  3. Tidak ada mekanisme real-time refresh saat backend menulis history baru.

- **Solusi**:
  1. **Backend (`scan_video`)**: Menambahkan parameter `app: tauri::AppHandle` dan memanggil `upsert_job_history` dengan status **"Draft"** setelah berhasil menganalisis video YouTube atau lokal. Untuk video lokal, `video_id` dibuat unik menggunakan `std::time::SystemTime` timestamp (`local_{timestamp}`).
  2. **Backend (`upsert_job_history`)**: Menambahkan `app.emit("job-history-updated", ())` setelah menyimpan ke store, sehingga frontend otomatis tahu ada perubahan tanpa polling.
  3. **Frontend (`useJobHistory.ts`)**: 
     - Refactor untuk mendengarkan event Tauri `job-history-updated` via `listen()` di dalam `onMounted`.
     - Setiap kali event diterima, otomatis memanggil `loadHistory()` untuk refresh data.
     - Cleanup listener di `onUnmounted` untuk mencegah memory leak.
  4. **Frontend (Komponen Dashboard)**: Menghapus semua `onMounted(() => loadHistory())` manual di `ActivityGrid.vue`, `StatsOverview.vue`, dan `LibraryView.vue` — composable sekarang auto-load dan auto-refresh.

- **Hasil**: 
  - Saat user memasukkan URL di Studio (`handleLoadVideo`), job langsung muncul di Dashboard dengan status "Draft".
  - Saat proses clip/compilation dimulai, status berubah menjadi "Processing".
  - Saat selesai atau gagal, status menjadi "Completed" atau "Failed".
  - Semua perubahan status **langsung terlihat** di Dashboard tanpa perlu refresh manual atau navigasi ulang.
  - User bisa klik job di Dashboard/Library untuk melanjutkan proses yang gagal atau diulang.

- **File yang diubah**:
  - **Backend**: `src-tauri/src/orchestrator/scan.rs` (tambah `AppHandle` param + panggil `upsert_job_history`), `src-tauri/src/commands/video.rs` (tambah `use tauri::Emitter` + emit event).
  - **Frontend**: `src/composables/useJobHistory.ts` (event listener), `src/components/dashboard/ActivityGrid.vue`, `src/components/dashboard/StatsOverview.vue`, `src/views/LibraryView.vue` (hapus manual `loadHistory`).
- **Solusi**:
  1. **Backend Registry (`src-tauri/src/ai/onnx.rs`)**:
     - Membuat `OnnxModelInfo` struct (Serialize) + const `ONNX_MODEL_REGISTRY` — 6 model: `visual` (emotion_vit.onnx, ~330MB), `voice` (wav2vec2_superb_er.onnx, ~380MB), `audio` (ast_audioset.onnx, ~350MB), `text` (twitter_roberta_emotion.onnx, ~500MB), `text_tokenizer` (twitter_roberta_tokenizer.onnx → **tokenizer.json**, ~3.6MB), `face` (seeta_fd_frontal_v1.0.bin, ~2MB).
     - Setiap entry: `id`, `file`, `url`, `display_name`, `category`, `description`, `approx_size`, `tags[]`.
     - Helper: `find_model(id)`, `find_model_by_file(file)`, `model_path_for(file)`, `models_dir()`, `model_statuses()`.
     - `ensure_model_downloaded` refactor: memprioritaskan URL dari registry by filename (backward-compatible, fallback ke arg `url`).
  2. **Tauri Commands** (registered `lib.rs` invoke handler):
     - `list_onnx_models` → `Vec<OnnxModelStatus>` (`{ id, exists, size_bytes, path }`) — status disk aktual untuk UI.
     - `download_onnx_model(id)` → streaming download via `reqwest::bytes_stream()` + `futures_util::StreamExt`, atomic write ke `.part` temp lalu rename; memancarkan event `onnx-download-progress` (ProgressEvent shape) untuk progress bar real-time.
     - `delete_onnx_model(id)` → hapus file dari `AppData/models/`.
  3. **Analyzer Refactor (SSOT Adoption)**:
     - Semua `OnnxModelManager::new(file, url)` di `analysis/{visual,voice,audio,text}.rs` dan `face/tracker.rs` (2 sites) kini memanggil `find_model(id).map(|m| m.url).unwrap_or("")` — menghilangkan hardcoded URL duplikat.
     - **Bug Fix**: `text.rs` tokenizer sekarang mengunduh `tokenizer.json` (sebelumnya salah pakai `model.onnx` URL).
  4. **Frontend (`src/`)**:
     - `constants/onnxModels.ts` — mirror registry backend (tokenizer URL diperbaiki, approx size → `~3.6 MB`).
     - `components/settings/ModelsSection.vue` — UI baru: list 6 model (status dot, nama, kategori, ukuran, tags), tombol Unduh/Hapus/Unduh Ulang per model, tombol "Unduh Semua", inline progress bar per model, summary strip (terpasang count), listener `onnx-download-progress` (onMounted, unlisten onUnmounted) untuk progress streaming real-time.
  5. **Cargo**: Menambahkan `futures-util = "0.3"` (sudah di lock, no version bump) untuk `StreamExt::next()`.
  6. **Verifikasi**: `cargo check` clean (6.6s), `npm run build` clean (1.6s).
- **Catatan**: File `.part` tidak dibersihkan otomatis saat download dibatalkan mid-way (next `list_onnx_models` mengabaikannya; retry overwrites). Registry sekarang SSOT tunggal untuk semua model ONNX — frontend hanya mirror pasif.

### 4.38. Perbaikan VFX Durasi, Normalisasi Audio Mix, & Debug Mode MSI Afterburner OSD HUD

- **Problem**:
  1. **Visual VFX Terpotong**: Efek meme video (mis. durasi 5 detik) terpotong secara visual hanya tampil 1-2 detik mengikuti *bounding window* emosi AI, meskipun audionya tetap lanjut berbunyi.
  2. **Audio Video Utama Redup Drastis**: Setiap penambahan efek video membuat audio utama game semakin pelan secara eksponensial.
  3. **Debug Mode Terbatas & Terpotong**: Mode debug lama hanya membakar *bounding box* wajah sebelum video di-crop ke 9:16 sehingga teks dan kotak terdistorsi/hilang, serta belum menampilkan telemetri analisis AI secara menyeluruh.

- **Solusi**:
  1. **Durasi Visual Penuh VFX (`vfx.rs`)**: Mengganti ekspresi enable pada filter `overlay` FFmpeg dari `'between(t, start, end)'` menjadi `'gte(t, start)'` dengan opsi `eof_action=pass`. Dengan ini, video efek berjalan sesuai durasi alaminya sampai EOF selesai secara independen. Selain itu, memberi jarak minimal 4 detik antar pemicu efek di `effects.rs`.
  2. **Single-Pass Audio Mix (`amix` dengan `normalize=0`)**: Membuang chaining pairwise `amix` berulang di `apply_vfx` yang membagi sinyal input ($1/2^N$). Seluruh *stream* audio efek yang telah diberi `adelay` dikumpulkan dan dicampur sekaligus bersama track audio utama (`0:a`) dalam satu node tunggal: `amix=inputs=N+1:duration=first:dropout_transition=0:normalize=0`. Volume asli game tetap 100% utuh tanpa redaman.
  3. **MSI Afterburner / RTSS OSD HUD (`ass_writer.rs` & `subtitle.rs`)**:
     - Memperluas `EmotionTimeline` dan `EmotionCacheEntry` untuk menyimpan seluruh modalitas AI (`fusion`, `visual`, `audio`, `voice`, `text`).
     - Mengimplementasikan `generate_msi_afterburner_osd(...)` yang menghasilkan subtitle ASS dengan style `MSI_OSD`: font monospace `Consolas`, `BorderStyle: 3` dengan box latar belakang gelap transparan (`&HA0101010`), teks label amber RTSS (`&H002080FF&`), nilai cyan (`&H00FFFF00&`), skor hijau (`&H0000FF00&`), dan cuplikan teks Whisper putih (`&H00FFFFFF&`).
     - Memindahkan pembakaran debug ASS dari fase *pre-crop* ke fase *post-crop* (setelah watermark) di `burn_video_effects` sehingga HUD selalu tajam, presisi pada resolusi vertikal (1080x1920), dan tidak tertutup elemen visual lainnya.
     - Memperbarui label deskripsi di `EngineSection.vue`.

### 4.39. Migrasi Model Visual ViT FER2013 & Audit Best Practice Arsitektur ONNX

- **Problem**:
  1. Model emosi visual lama dari Xenova perlu dimigrasi ke model komunitas modern `onnx-community/face-emotion-detection-ONNX` (fine-tuned pada FER2013). Urutan label output model baru (Angry=0, Disgust=1, Fear=2, Happy=3, Sad=4, Surprise=5, Neutral=6) berbeda dari model lama, sehingga jika tidak disesuaikan, deteksi emosi visual akan salah klasifikasi secara fatal.
  2. Audit arsitektur pada `src-tauri/src/ai/onnx.rs`:
     - *Separation of Concerns*: Handler Tauri IPC (`list_onnx_models`, `download_onnx_model`, `delete_onnx_model`) bercampur di dalam modul domain AI `ai/onnx.rs`, melanggar konvensi di mana semua perintah IPC berada di `commands/*.rs`.
     - *Concurrency Race Condition*: `OnnxModelManager::ensure_loaded` dapat melempar `CliptzyError` jika dua thread concurrent mencoba mengisi `OnceCell` secara simultan.
     - *Inefisiensi Memori (RAM Spikes)*: `ensure_model_downloaded` lama memuat 350-500 MB file model sekaligus ke dalam buffer RAM (`response.bytes()`) sebelum menulisnya secara blocking ke disk.
     - *Ketiadaan Fallback GPU*: Session builder DirectML langsung melempar error dan membatalkan analisis jika sistem/driver tidak kompatibel dengan DirectML, alih-alih fallback ke CPU.

- **Solusi**:
  1. **Update Registry & Frontend SSOT**:
     - Memperbarui entri `visual` di `ONNX_MODEL_REGISTRY` dan `src/constants/onnxModels.ts` ke URL `https://huggingface.co/onnx-community/face-emotion-detection-ONNX/resolve/main/onnx/model.onnx`, nama file `face_emotion_detection.onnx` (~343 MB), dan tag 7 emosi.
  2. **Penyesuaian Input/Output di `visual.rs`**:
     - Input tensor tetap `pixel_values` (224×224 RGB dengan normalisasi mean=[0.5, 0.5, 0.5] & std=[0.5, 0.5, 0.5]).
     - Output `logits` di-mapping akurat sesuai urutan FER2013: Index 0=Angry, 1=Unknown (Disgust), 2=Fear, 3=Happy, 4=Sad, 5=Shock (Surprise), 6=Neutral.
     - Menggunakan konstruktor cerdas `OnnxModelManager::from_registry("visual")`.
  3. **Refaktor Arsitektur & Best Practice Rust**:
     - **Separation of Concerns**: Memindahkan fungsi Tauri command decorator ke `src-tauri/src/commands/ai.rs`, dan mendaftarkannya seragam di `src-tauri/src/lib.rs` sebagai `commands::ai::*`. `ai/onnx.rs` tetap mengekspor fungsi tersebut untuk backward compatibility.
     - **Thread-Safety OnceCell**: Jika `OnceCell::set` mengembalikan error karena sudah diisi thread lain, aplikasi tidak lagi panik/gagal melainkan melanjutkan eksekusi dengan aman.
     - **Streaming Zero-Cost Download**: `ensure_model_downloaded` kini menggunakan stream chunking langsung ke file temporary `.part` dan rename atomik, menghilangkan alokasi RAM 500 MB dan mencegah file model korup.
      - **DirectML Graceful Fallback**: Jika DirectML execution provider gagal di Windows, sistem otomatis mencatat `log::warn!` dan fallback ke CPU session builder tanpa menggagalkan proses clip video.

### 4.40. Backend SSOT untuk Model ONNX & Eliminasi Duplikasi `onnxModels.ts`

- **Problem**: Metadata model ONNX didefinisikan ganda di backend Rust (`ONNX_MODEL_REGISTRY` di `src-tauri/src/ai/onnx.rs`) dan frontend TypeScript (`src/constants/onnxModels.ts`). Jika model diubah, ditambahkan, atau dihapus di backend, frontend rentan desinkronisasi atau membutuhkan pembaruan manual di dua tempat.
- **Solusi**:
  1. **Ekspansi Kontrak Data `OnnxModelStatus`**: Menjadikan struct `OnnxModelStatus` di Rust sebagai pembawa data lengkap (metadata registry: `id`, `file`, `url`, `display_name`, `category`, `description`, `approx_size`, `tags` + status disk: `exists`, `size_bytes`, `path`).
  2. **Penghapusan Total `src/constants/onnxModels.ts`**: Menghapus file konstanta frontend tersebut secara permanen.
  3. **Refaktor Frontend `ModelsSection.vue`**: Mengganti ketergantungan statis dengan pemanggilan asinkron `invoke<OnnxModelItem[]>("list_onnx_models")`. Semua aksi unduh, unduh ulang, hapus, dan unduh semua kini beroperasi langsung pada status dinamis yang dikembalikan backend.

### 4.41. Model Emosi Teks Multilingual (MiniLM) & Perbaikan Pipeline Spektrogram Audio (AST)

- **Problem**:
  1. `text: []` kosong di `emotions_{idx}.json`: `emotion_phase` berjalan mendahului transkripsi Whisper sehingga file transkrip belum ada di disk; path transkrip di-hardcode ke `transcript_{idx}_tiny.json` bukannya folder `cache/`; dan format deserialisasi JSON tidak cocok dengan objek pembungkus `SegmentTranscriptCacheEntry`. Selain itu, model teks lama (`twitter_roberta_emotion.onnx`) hanya mendukung bahasa Inggris.
  2. `audio: []` kosong di `emotions_{idx}.json`: Pemetaan label AudioSet pada `audio.rs` salah total (indeks 27 & 28 dimapping sebagai scream/yell padahal sebenarnya singing & choir; teriakan asli indeks 8..14 tidak terdaftar); perhitungan Mel filterbank lama belum mengimplementasikan proyeksi filterbank segitiga standar sehingga logits tidak mencapai threshold.
- **Solusi**:
  1. **Migrasi Model Teks Multilingual (`tanaos-emotion-detection-v1-ONNX`)**:
     - Mengganti model `text` di `ONNX_MODEL_REGISTRY` ke model Multilingual MiniLM-L12 (~180 MB) yang mendukung 100+ bahasa termasuk Bahasa Indonesia secara native.
     - Memperbarui `text.rs` untuk memetakan 8 kelas emosi (*joy, anger, fear, sadness, surprise, disgust, excitement, neutral*).
     - Mengimplementasikan parser transkrip fleksibel yang mendukung format objek `SegmentTranscriptCacheEntry` maupun array flat.
  2. **Sinkronisasi Pipeline Transkripsi & Audio 16kHz**:
     - Menyatukan ekstraksi audio menjadi `audio_16k_{idx}.wav` bersama yang digunakan baik oleh `emotion_phase` maupun `subtitle_phase`.
     - `emotion_phase` memicu pemuatan transkrip Whisper (`load_or_transcribe_segment`) terlebih dahulu jika analisis teks aktif. Hasilnya disimpan ke cache sehingga `subtitle_phase` langsung mendapatkan *cache hit* instan (0 ms).
  3. **Implementasi Spektrogram Mel Filterbank & AudioSet Mapping Lengkap**:
     - Mengimplementasikan `build_mel_filterbank` 128-bin segitiga kontinu (20 Hz - 8000 Hz) dengan proteksi filter sempit di frekuensi rendah.
     - Memperbaiki pemetaan AudioSet secara komprehensif: teriakan (*shout, bellow, yell, screaming*, grunt), tawa (*laughter, giggle, snicker, chortle*), tangis (*crying, whimper, moan, sigh*), dan ledakan/tembakan (*gunshot, explosion*).
     - Menyimpan beberapa emosi berbeda per chunk audio (multi-label) dengan threshold probabilitas > 0.15.

### 4.42. AI Context Arbiter & VFX Meme Director (Multimodal Fusion & Kurasi Efek Cerdas)

- **Problem**:
  1. `fusion.rs` sebelumnya menggunakan pembobotan linear statis (`visual: 0.4, audio: 0.2, voice: 0.3, text: 0.1`) tanpa pemahaman konteks semantik lintas modalitas. Sebagai contoh, ketika streamer mengucapkan ucapan menyerah *"udah bang / ampun bang"* dengan ekspresi wajah meringis ketakutan (ViT mendeteksi *Angry*), sistem linear secara salah mengklasifikasikannya sebagai *Angry*, mengabaikan konteks kepanikan/komedi.
  2. `effects.rs` memilih efek video (`video_effects.json`) secara acak murni hanya berdasarkan kecocokan string emosi (`matching.choose(&mut rng)`), tanpa memperhatikan timing komedi, punchline, maupun narasi momen.
- **Solusi**:
  1. **Modul `ContextArbiter` (`src-tauri/src/analysis/arbiter.rs`)**:
     - Mengumpulkan dan menyatukan 4 modalitas sensorik (*Visual Face & Bounding Box, Voice Acoustic Tone, Audio Events Sound Classifier, dan Transkrip Whisper*) ke dalam *multimodal timeline buckets*.
     - Memberikan katalog efek video lengkap (35+ meme) beserta deskripsi semantik humor, intensitas, dan emosinya kepada model AI.
     - **Integrasi LLM Provider**: Jika provider AI aktif (OpenAI / Gemini / Ollama), sistem memanggil model untuk mengarbitrase emosi sejati per segmen dan menentukan jadwal 0–2 VFX meme pada titik klimaks/punchline dengan jeda minimal (*cooldown*).
     - **Smart Contextual Heuristics Arbiter (Offline Engine)**: Jika pengguna tidak mengonfigurasi API key atau sedang offline, sistem deterministik kontekstual mengevaluasi frasa slang/gaming Indonesia (*"udah bang" / "ampun" / "mati gua"* meng-override wajah marah menjadi *Fear*; *"lah kok" / "gimana dah"* menjadi *Confused*; *"anjir" / "kaget"* menjadi *Shock*) serta memilih VFX yang relevan secara semantik (misal: menyerah → *GTA CJ Ah Shit Here We Go Again* atau *KSI NoNoNo*; kaget → *Vineboom* / *IShowSpeed Scream*; bingung → *The Rock Sus*).
  2. **Persistensi Cache & Dekopling Pembakaran Efek**:
     - `EmotionTimeline` dan `EmotionCacheEntry` kini menyimpan `scheduled_effects`.
     - Jadwal efek langsung terintegrasi dengan MSI Afterburner Debug OSD (`VFX MEME: {name} [ACTIVE]`) dan pembakaran greenscreen di `subtitle_phase` (terhubung ke toggle *"Auto B-Roll"*).

### 4.43. Penumpukan VFX Meme (VFX Stacking & Fast Reaction) & Kalibrasi Anti-False-Angry

- **Problem**:
  1. **Visual Emotion Terlalu Sering "Angry"**: Model FER2013 sering mendeteksi wajah fokus gamer/streamer (alis sedikit bertaut saat menatap layar) sebagai *Angry*, karena indeks probabilitas kelas 0 (Angry) lebih tinggi tipis dibanding Neutral, membanjiri klip dengan meme kemarahan (misal: *cat slamming table*) yang tidak sesuai situasi.
  2. **Efek Meme Terlalu Kaku & Lambat**: Jadwal efek video sebelumnya dibatasi jeda *cooldown* kaku sehingga efek tidak dapat saling tumpang tindih (*stacking*) dan membatasi variasi komedi cepat yang dinamis.

- **Solusi**:
  1. **Kalibrasi Margin Netral Emosi Visual (`src-tauri/src/analysis/visual.rs`)**:
     - Memperkenalkan threshold keyakinan minimum: Jika probabilitas tertinggi `< 0.28`, otomatis diklasifikasikan sebagai `Neutral`.
     - Untuk kelas *Angry* (indeks 0): wajib memiliki probabilitas `probs[0] >= 0.42` DAN selisih margin terhadap *Neutral* `(probs[0] - probs[6]) >= 0.10`. Wajah konsentrasi biasa kini secara akurat dideteksi sebagai `Neutral`.
     - Kalibrasi serupa pada *Disgust* (gerakan bibir saat berbicara) dan *Sad* (menunduk melihat keyboard/gamepad).
  2. **Supresi Multimodal Lintas Sensor (`src-tauri/src/analysis/arbiter.rs`)**:
     - Heuristik dan prompt AI Arbiter secara tegas menyaring wajah *Angry*: jika suara audio/nada vokal tenang dan transkrip tidak mengandung umpatan/teriakan, emosi sejati dikembalikan ke `Neutral`.
  3. **Penumpukan Efek Video Bersusun (VFX Stacking / Combo)**:
     - Mengizinkan penumpukan hingga 2 efek video bersamaan (`active_count < 2`) dengan interval pemicu cepat (0.8s – 1.5s), memungkinkan kombinasi *punchline* (contoh: SFX Vineboom seketika diikuti video reaksi FlightReacts tumpang tindih).
     - Menjaga batas variasi dengan memilih 2–6 efek berbeda per segmen tanpa duplikasi efek yang sama dalam jeda 4 detik.
  4. **Pembersihan Bersih FFmpeg Filtergraph (`vfx.rs` & `burner/mod.rs`)**:
     - Mengubah filter overlay dari `'gte(t, start)'` menjadi `'between(t, start, end)'` dengan durasi efek terhitung presisi (`effect.end_time`), sehingga layer efek yang selesai tidak menumpuk permanen di buffer video.
     - Mempertahankan mixing audio satu arah `amix` dengan `normalize=0` agar game audio tetap 100% lantang tanpa reduksi volume saat efek bersusun berbunyi bersamaan.

### 4.44. Efek Visual Kamera & Filter Bawaan FFmpeg (Zero-Asset Builtin VFX) & Proteksi Panjang Command Windows

- **Problem**:
  1. Penambahan banyak efek video pada klip Shorts berisiko menyebabkan *command-line buffer overflow* di OS Windows jika setiap filter ditambahkan sebagai node terpisah secara naif.
  2. Aset video greenscreen eksternal memerlukan dependensi storage dan I/O decode disk, sedangkan video gaming membutuhkan efek dinamis berbasis kamera (guncangan layar / *screen shake*, kilatan *flashbang*, desaturasi *black & white*, distorsi *deep-fried*, *punch zoom*, *red tint*, *negate*, *blur*, *sepia*, dan *rainbow cycling*) yang bereaksi langsung pada video utama.

- **Solusi**:
  1. **Modul `BuiltinVfx` (`src-tauri/src/processing/burner/builtin.rs`)**:
     - Mengimplementasikan 10 efek visual bawaan murni filter internal FFmpeg tanpa dependensi aset file:
       * **Screen Shake**: Guncangan rotasi sudut kamera 1.4 derajat pada frekuensi 45 rad/s (`rotate='sin(t*45)*0.025':ow=iw:oh=ih`) saat amarah/teriakan.
       * **White Flash**: Kilatan putih dramatis saat momen jumpscare/punchline (`eq=brightness='if(between(...),0.65,0.0)'`).
       * **Dramatic B&W**: Grayscale + vignette gelap saat menyerah / pasrah / mati (`hue=s=0,vignette=PI/4`).
       * **Deep-Fried**: Kontras 2.0x dan saturasi 3.0x saat momen kemarahan ekstrem / ear-rape scream (`eq=contrast=2.0:saturation=3.0`).
       * **Punch Zoom**: Snap zoom in 18% ke tengah kanvas saat momen bingung / canggung (`crop=w='...':h='...',scale=1080:1920`).
       * **Red Tint**: Nuansa merah pekat darah (`colorchannelmixer=rr=1.8:gg=0.4:bb=0.4`) saat amarah tinggi, bahaya, atau low HP.
       * **Negate**: Inversi warna negatif horor (`negate`) saat momen cursed, jumpscare menyeramkan, atau plot twist tak terduga.
       * **Focus Blur**: Gaussian blur dramatis (`gblur=sigma=12`) saat momen bengong, freeze otak, pusing, atau keheningan canggung.
       * **Sepia**: Nada hangat klasik sepia vintage (`colorchannelmixer=.393:.769:...`) untuk kilas balik sedih, refleksi kekalahan, atau kenangan.
       * **Rainbow Hue**: Rotasi siklus warna pelangi dinamis (`hue=H=8*PI*t`) saat momen kemenangan, selebrasi GG, atau tawa lepas renyah.
  2. **Proteksi Buffer Command Windows (Boolean Span Consolidation)**:
     - Menggabungkan seluruh rentang waktu aktif untuk tipe efek yang sama menjadi satu ekspresi boolean tunggal: `between(t, s1, e1) + between(t, s2, e2) + ...`.
     - Jumlah node filter yang disuntikkan ke FFmpeg **dijamin tidak pernah melebihi 11 node** bahkan jika ada puluhan kemunculan efek, dengan total panjang string command `< 800 karakter`, menghilangkan 100% risiko limit `CreateProcess` (32.767 karakter) di Windows.
  3. **Penjadwalan Cerdas di Arbiter (`arbiter.rs`)**:
     - Heuristik dan AI Arbiter otomatis memicu:
       * *Shock* → *WhiteFlash*, extreme shock (score >= 0.88) combo dengan *Negate*.
       * *Angry* → *ScreenShake*, kemarahan tinggi (score >= 0.80) combo dengan *RedTint*, dan amarah klimaks (score >= 0.88) combo dengan *DeepFried*.
       * *Confused* → *PunchZoom*, kebingungan tinggi (score >= 0.85) combo dengan *FocusBlur*.
       * *Sad / Surrender* → *DramaticBW*, kesedihan mendalam (score >= 0.85) combo dengan *Sepia*.
       * *Happy* → *RainbowHue*.
     - Durasi mikro (0.2s - 2.5s) dengan cooldown 3.0s antar efek sejenis dan batas maksimal 6 efek per klip.
  4. **Integrasi UI & Telemetri**:
     - Toggle baru `"use_builtin_fx"` di Settings (`AISection.vue`) dan Studio (`InspectorPanel.vue`).
     - MSI Afterburner Debug OSD HUD menampilkan telemetri kamera secara real-time (`VFX : ScreenShake [CAMERA FX]` / `sfx_vineboom + WhiteFlash [ACTIVE]`).

### 4.45. Perbaikan Sinkronisasi Render VFX Meme, FFmpeg Filter WhiteFlash, & Durasi Media Aktual

- **Problem**:
  1. Pada segmen render klip (misal `emotions_16.json`), OSD HUD menampilkan `VFX: the rock_sus [ACTIVE]` dan `VFX: flightreact_woah woah hey hey [ACTIVE]`, namun video overlay meme tidak muncul sama sekali di video hasil render.
  2. Efek `WhiteFlash` tidak terlihat (kecerahan video tidak berubah sama sekali).
  3. Efek kamera `ScreenShake` di detik ke-9 terasa tidak terasa / terlalu samar.
  4. Efek `the rock_sus` di OSD ditampilkan aktif hingga 37.5 detik, namun video aslinya berdurasi lebih pendek.

- **Penyebab Utama & Solusi**:
  1. **Desinkronisasi OSD vs Burner & Pengaturan `use_add_meme`**:
     - Di `config.json`, `"use_add_meme": false`. Di [`subtitle.rs`](file:///C:/cliptzy/src-tauri/src/orchestrator/clip/subtitle.rs), filter membuang seluruh `scheduled_effects` saat `use_add_meme == false` sehingga FFmpeg tidak menerima input meme sama sekali. Namun pembuatan teks OSD di [`ass_writer.rs`](file:///C:/cliptzy/src-tauri/src/transcription/ass_writer.rs) menerima daftar efek mentah sebelum difilter.
     - **Solusi**: Menyinkronkan variabel `effective_scheduled_effects` dan `effective_builtin_effects` ke OSD dan `VideoBurnerConfig`, mengaktifkan `"use_add_meme": true` di konfigurasi default, dan memperjelas label UI menjadi *"VFX Meme (Auto B-Roll)"*.
  2. **Evaluasi `eval=init` pada Filter `eq` FFmpeg**:
     - Filter `eq` pada FFmpeg secara default mengevaluasi ekspresi matematika satu kali di timestamp `t=0.0` (`eval=init`). Ekspresi `eq=brightness='if(between(t,...),0.65,0.0)'` menghasilkan `0.0` permanen karena pada `t=0.0` kondisi `between` bernilai salah, membuat kilatan putih 100% tidak tampak.
     - **Solusi**: Mengganti filter menjadi aktivasi linier timeline murni: `eq=brightness=0.85:enable='between(t,s,e)'`. Pengujian luminance (`signalstats`) membuktikan `YAVG` melonjak instan ke `255` (putih maksimal).
  3. **Durasi Aset Media Dinamis (`get_duration`)**:
     - Sebelumnya durasi setiap efek di-hardcode 3.5 detik di `arbiter.rs`. Namun file `the rock_sus.mp4` durasi aslinya hanya 2.02 detik, sehingga efek menghilang di detik ke-36.02 sementara HUD OSD terus menampilkan `[ACTIVE]` hingga detik ke-37.5.
     - **Solusi**: Menambahkan method [`VideoEffect::get_duration(&self)`](file:///C:/cliptzy/src-tauri/src/processing/effects.rs) yang memetakan durasi riil seluruh 38 aset media, dan menghitung `end_t` arbiter berdasarkan durasi aktual file.
  4. **Peningkatan Amplitudo `ScreenShake`**:
     - Menaikkan sudut getaran kamera dari `0.025` rad (~1.4°) menjadi `0.05` rad (~2.9° pada 50 rad/s) agar getaran terasa jelas di video Shorts vertikal.

### 4.46. Penanganan Error FFmpeg Exit Code 0xffffffef (EEXIST Overwrite Protection)

- **Problem**:
  - Saat merender ulang klip video yang sama tanpa menghapus folder `jobs/` sebelumnya, proses gagal tepat di akhir tahap stacking dengan pesan:
    `Command Error: FFmpeg { code: -1, message: "Process failed: Process execution failed: Process exited with status: exit code: 0xffffffef" }`
- **Root Cause**:
  - Kode exit `0xffffffef` merupakan representasi heksadesimal 32-bit dari `-17`, yaitu `-EEXIST` (POSIX `File exists`) dari FFmpeg `AVERROR(EEXIST)`.
  - Tahap pembuatan thumbnail ([`src-tauri/src/processing/thumbnail.rs`](file:///C:/cliptzy/src-tauri/src/processing/thumbnail.rs)) menggunakan `FFmpegBuilder` tanpa memanggil `.overwrite()`.
  - Ketika `thumbnail_16.jpg` sudah ada di disk dari proses render sebelumnya, FFmpeg menunggu konfirmasi interaktif `Overwrite? [y/N]` di stdin. Karena stdin tertutup/null, FFmpeg otomatis membatalkan eksekusi dengan pesan *"Not overwriting - exiting. Error opening output files: File exists"* dan melempar exit code `0xffffffef`.
- **Solusi**:
  1. Menambahkan pembersihan proaktif file lama (`if output_path.exists() { std::fs::remove_file(output_path); }`) dan pembuatan folder induk jika belum ada.
  2. Menyuntikkan method `.overwrite()` (flag `-y`) pada seluruh builder FFmpeg yang sebelumnya terlewat:
     - [`src-tauri/src/processing/thumbnail.rs`](file:///C:/cliptzy/src-tauri/src/processing/thumbnail.rs) (`generate_thumbnail`)
     - [`src-tauri/src/video/local.rs`](file:///C:/cliptzy/src-tauri/src/video/local.rs) (`cut_local_segment`)
     - [`src-tauri/src/processing/cropper/split_broll.rs`](file:///C:/cliptzy/src-tauri/src/processing/cropper/split_broll.rs) (`build_command`)
