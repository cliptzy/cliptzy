# 🎨 TODO_GUI.md — Rencana Pembangunan Frontend (Tauri + Vue 3)

> **Dokumen ini adalah peta jalan (roadmap) untuk membangun antarmuka pengguna (UI/UX) Cliptzy.**
> Mengabaikan desain generik, kita akan mengimplementasikan visi desain **"Spatial Bento Box"** — sangat modern, *out-of-the-box*, berkelas profesional, responsif, dan dirancang khusus untuk memaksimalkan kapabilitas aplikasi Desktop native.

---

## 🎯 Visi Desain: "Spatial Bento Box"
Kita akan membuang desain "AI Slop" (gradient ungu/pink pasaran dengan komponen material biasa) dan beralih ke antarmuka NLE (Non-Linear Editing) yang revolusioner:
- **Bento Grid System**: Tata letak berbasis grid modular (Bento) yang padat informasi, efisien, dan bersih. Memungkinkan dashboard menampilkan banyak data visual tanpa terlihat penuh.
- **Deep Dark Mode (OLED-inspired)**: Menggunakan palet warna gelap pekat (True Black & Deep Zinc) dengan aksen warna kontras tinggi (seperti *Electric Lime* atau *Cyber Orange*) khusus untuk interaksi utama.
- **Micro-Glassmorphism & Spatial Depth**: Penggunaan efek blur (backdrop-filter) berlapis secara subtil pada panel mengambang, memberikan ilusi kedalaman tanpa memberatkan performa rendering.
- **Borderless / Custom Window Frame**: Memanfaatkan Tauri frameless window dengan custom titlebar agar aplikasi terasa menyatu dan elegan sebagai software desktop premium.
- **Highly Responsive Layout**: UI beradaptasi mulus dari ukuran window ultra-wide hingga ke mode *compact/mini-player*.

---

## 📑 DAFTAR ISI

- [Fase 1: Fondasi & Arsitektur UI](#fase-1-fondasi--arsitektur-ui)
- [Fase 2: Layout Shell & Navigasi Super-fluid](#fase-2-layout-shell--navigasi-super-fluid)
- [Fase 3: Dashboard & Akuisisi Cerdas](#fase-3-dashboard--akuisisi-cerdas)
- [Fase 4: Studio Compiler (Editor Utama)](#fase-4-studio-compiler-editor-utama)
- [Fase 5: Settings & Ekosistem Platform](#fase-5-settings--ekosistem-platform)
- [Fase 6: Polish, Animasi & Micro-interactions](#fase-6-polish-animasi--micro-interactions)
- [ADR: Manajemen State Konfigurasi (Rust vs TS)](#adr-manajemen-state-konfigurasi-rust-vs-ts)

---

## 🏗️ Fase 1: Fondasi & Arsitektur UI

Membangun kerangka desain kustom yang kuat menggunakan Tailwind CSS dan Vue 3.

- [x] **Konfigurasi Tauri Native Window**
  - Ubah `tauri.conf.json`: Set `decorations: false` dan `transparent: true`.
  - Buat komponen `MacOsWindowsTitleBar.vue` (tombol Minimize, Maximize, Close kustom) dengan area yang bisa di-drag (`data-tauri-drag-region`).
- [x] **Setup Typography & Variabel Tailwind**
  - Integrasikan font geometris modern (seperti *Geist*, *Inter Tight*, atau *Space Grotesk*).
  - Tentukan palet warna kustom di `tailwind.config.js` (diimplementasikan via CSS variables di v4):
    - `bg-base`: `#09090B` (Background absolut)
    - `bg-surface`: `#18181B` (Warna kartu Bento)
    - `border-subtle`: `#27272A` (Garis pemisah tipis)
    - `accent-primary`: `#D9F99D` (Aksen hijau neon yang elegan)
- [x] **Pustaka Komponen Modular (Bento Box)**
  - Buat komponen `BentoCard.vue` dengan sudut presisi (`rounded-2xl` atau `rounded-[24px]`), border 1px solid, dan subtle inner highlight.
  - Bangun UI primitive tanpa library tambahan: `GlowButton.vue`, `SpatialInput.vue`, `ToggleSwitch.vue`, `RangeSlider.vue`.
- [x] **State Management (Pinia)**
  - `useVideoStore`: Menyimpan URL aktif, metadata YouTube (judul, heatmap), dan status analisis.
  - `useAppStore`: Menyimpan status sidebar, notifikasi global, dan *real-time processing progress*.

---

## 🧭 Fase 2: Layout Shell & Navigasi Super-fluid

Kerangka navigasi utama yang responsif terhadap perubahan ukuran window desktop.

- [x] **Aplikasi Layout Shell (The Grid)**
  - Gunakan CSS Grid untuk membagi area layar: Sidebar Fleksibel (Kiri) dan Area Konten Dinamis (Kanan).
- [x] **Sidebar Navigasi (Collapsible & Floating)**
  - Tab Menu: 🏠 Dashboard, ✂️ Studio, 📚 Library (Riwayat), ⚙️ Settings.
  - Active state out-of-the-box: Bukan highlight blok warna, melainkan garis tipis bercahaya di samping icon dengan efek transisi mask.
  - **Responsive behavior**: Saat window diperkecil, sidebar berubah menjadi *Bottom Dock* mengambang (ala macOS dock).
- [x] **Global Background Task Monitor**
  - Buat komponen `GlobalStatusBar.vue` di sudut kanan bawah.
  - Komponen ini menangkap event dari backend Rust via Tauri (`listen('clip-progress')`) untuk menampilkan progress bar (download, render, AI parsing) yang terpisah dari halaman aktif.

---

## 🎬 Fase 3: Dashboard & Akuisisi Cerdas

Halaman masuk yang memukau dan berorientasi langsung pada aksi.

- [x] **Dashboard Murni Monitoring**
  - Menampilkan panel aktivitas antrean dan status terkini.
  - Widget analitik sederhana (Sisa penyipanan, Total Diproses).
- [x] **Queue & Recent Activity Grid**
  - Menampilkan video-video yang sedang mengantre atau yang baru selesai di-render dengan layout mason/grid kompak.

---

## ✂️ Fase 4: Studio Compiler (Editor Utama)

Antarmuka inti yang dirancang seperti software NLE Nodal, namun disederhanakan.

- [x] **Layout Studio (3-Pane Spatial Split)**
  - **Kiri (Inspector)**: Pengaturan Cepat (Mode Crop, AI Process, Style Teks). Termasuk peringatan dinamis saat mode yang dipilih membutuhkan aset B-Roll atau Face Tracking.
  - **Tengah (Stage)**: Video Player / Preview Area interaktif dengan orientasi potret.
  - **Kanan (Source & Segments)**: Input URL, Scan Heatmap/AI, dan Daftar Segmen Terpilih.
  - **Bawah (Timeline)**: Track visual untuk video, highlight AI, dan subtitle.
- [x] **Visual Crop & Tracking Controller**
  - UI inovatif untuk mengontrol mode crop dari `cropper.rs`.
  - Tampilkan grid "Safe Zones" overlay (area yang tidak tertutupi oleh tombol Like/Komentar di TikTok/Reels).
- [x] **Action Core (The Generator)**
  - Tombol raksasa di pojok kanan bawah "Execute / Generate Clips" dengan efek *cyber-glow* memicu pipeline di orchestrator Rust.

---

## ⚙️ Fase 5: Settings & Ekosistem Platform

Tampilan pengaturan yang bersih bak panel kontrol spacecraft.

- [x] **Grid Akun Terhubung (SSO Integrations)**
  - Bento cards menampilkan akun yang tersambung (YouTube, TikTok, dll).
  - Status koneksi dengan indikator *pulsing dot* hijau (Connected) atau oranye (Needs Login).
  - Hook langsung ke `invoke('login_with_google')` Tauri command.
- [x] **Engine & Hardware Config Panel**
  - Pemilihan spesifikasi backend: Hardware Acceleration (NVENC/VideoToolbox), alokasi thread.
  - Konfigurasi API Keys (Gemini, Ollama, OpenAI) dengan input field yang di-masking rapi.
- [x] **Storage & Cache Manager**
  - Visualisasi sisa penyimpanan dalam bentuk *circular progress*.
  - Tombol satu klik untuk membersihkan temporary files FFmpeg.
- [x] **B-Roll & Assets Manager**
  - Tampilan manajemen path khusus untuk folder stok video B-Roll (`BrollAssetsSection.vue`).

---

## ✨ Fase 6: Polish, Animasi & Micro-interactions

Penyempurnaan yang membedakan aplikasi "oke" dari aplikasi "kelas dunia".

- [x] **Fluid Layout Transitions (Vue `<TransitionGroup>`)**
  - Transisi halaman yang menggunakan kombinasi `scale` tipis dan `fade`.
  - Grid Bento akan menyusun ulang diri (*masonry shuffle*) secara animasi saat window di-resize.
- [x] **Skeletal Loaders & Suspense**
  - Gunakan rangka skeleton dengan gradien animasi berkecepatan tinggi saat menunggu response Tauri.
- [x] **Toast & Error Handling Kustom**
  - Tampilan error/sukses bergaya *Command Line Output* di pojok kanan atas.
- [x] **Scrollbars & Custom Overlays**
  - Sembunyikan scrollbar native OS dan ganti dengan custom scrollbar tipis.

---

## 📝 ADR: Manajemen State Konfigurasi (Rust vs TS)

> **Konteks:** Menggantikan `clip_config.py` dari versi Python lama. Kita memiliki dua struktur yang mirip saat ini: `src-tauri/src/config/models.rs` (Rust) dan `src/stores/settings.ts` (TypeScript).

**Keputusan: RUST SEBAGAI SINGLE SOURCE OF TRUTH (SSOT)**

Kita akan menetapkan **Rust (`src-tauri/src/config/mod.rs`)** sebagai otoritas utama yang mengatur konfigurasi, menyimpan (I/O) file `config.json`, dan mensinkronisasikan ke Supabase.
TypeScript/Pinia (`settings.ts`) **HANYA BERPERAN SEBAGAI CACHE REAKTIF (MIRROR)** untuk UI.

**Implementasi GUI-nya Dipecah Menjadi 2 Tempat:**
1. **`SettingsView.vue` (Deep Settings):**
   - Segala hal yang bersifat statis/global (Hardware Acceleration, API Keys LLM, Default Padding, Storage Path, Watermark Position, TTS defaults) dipindahkan seluruhnya ke tampilan Settings (sesuai Fase 5).
2. **`StudioView.vue` (Quick/Contextual Settings):**
   - Pengaturan yang kerap diubah *per video* (Mode Crop, Ratio Output, Subtitle Style Hormozi/Karaoke, Toggle AI B-Roll/Whisper) telah ditempatkan di panel Kiri (Inspector) milik Studio (sesuai Fase 4).

**Alasan / Keuntungan Opsi Ini:**
- **Keamanan & Performa:** File I/O tidak ditangani LocalStorage browser, melainkan Native OS File System (AppData).
- **Backend-Driven:** Worker Rust (FFmpeg, Whisper) membutuhkan akses instan ke konfigurasi *tanpa* harus menanyakan ke Frontend. Jika State utamanya di TS, kita harus mengirim *seluruh object config* setiap kali men-trigger job di Rust.
- **Cloud Sync Siap:** Modul `supabase.rs` sudah siap melakukan cloud-sync untuk config jika config file dikelola oleh Rust.

**Rencana Eksekusi Kode:**
- Refactor `src/stores/settings.ts`: Hilangkan `@vueuse/core` (LocalStorage). Ganti dengan aksi `load_config` (memanggil `invoke('load_config')` dari Rust saat *app mount*) dan `watch` Vue untuk otomatis memanggil `invoke('save_config')` ke Rust bila *user* mengotak-atik Settings di UI.
