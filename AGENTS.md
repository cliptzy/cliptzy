# 📜 AGENTS.md — Peraturan & Panduan Proyek Cliptzy (Tauri Desktop App)

Dokumen ini adalah **sumber kebenaran utama (single source of truth)** untuk seluruh AI Model dan pengembang yang bekerja pada proyek **Cliptzy Desktop** — sebuah aplikasi YouTube Clipper & Auto Uploader yang dibangun menggunakan arsitektur Hibrida Tauri.

> ⚠️ **BACA SELURUH DOKUMEN INI SEBELUM MENULIS SATU BARIS KODE PUN.**
> Setiap asumsi yang tidak diverifikasi dari dokumen ini dianggap **halusinasi** dan harus dihindari.

---

## 📐 BAGIAN 0: GROUND TRUTH — STATUS PROYEK SAAT INI

> **Bagian ini WAJIB dibaca terlebih dahulu.** Ini menjelaskan kondisi aktual kode agar AI tidak berhalusinasi tentang apa yang sudah ada vs apa yang belum diimplementasi.

### 0.1 Apa yang SUDAH ADA (Fakta)

| Komponen | Status | Detail |
|----------|--------|--------|
| **Tauri + Vue 3 + TypeScript** | ✅ Scaffold dasar | Proyek Tauri v2 sudah diinisialisasi. Frontend masih berupa template default (greeting demo). |
| **Rust backend** | ✅ Minimal | Hanya `lib.rs` (fungsi `greet`) dan `main.rs` (entry point). Belum ada modul engine/health/bootstrapper. |
| **Git Submodule engine** | ✅ Terkonfigurasi | `.gitmodules` → `src-tauri/engine` → `https://github.com/cliptzy/engine`. Submodule sudah ter-clone. |
| **Python engine (core/)** | ✅ Mature | Berisi ~30+ modul produksi: `processor.py`, `subtitle.py`, `face_tracker.py`, `youtube.py`, `ffmpeg.py`, AI detector, uploaders (YouTube/TikTok/Instagram), TTS engine, emotion analyzer, dll. |
| **Python engine (gui/)** | ⚠️ Legacy Flet | Masih ada folder `gui/` berisi kode Flet (app.py, router.py, views/, components/). **HARUS DIHAPUS** — digantikan oleh Vue frontend. |
| **Python engine (main.py)** | ⚠️ Legacy CLI+Flet | Entry point lama dengan argparse CLI + Flet launcher. **HARUS DIGANTI** dengan `server.py` (FastAPI). |
| **pyproject.toml** | ⚠️ Perlu update | Masih berisi dependensi Flet (`flet`, `flet-video`, `flet-audio`, `pystray`, `desktop-notifier`). Belum ada `fastapi`/`uvicorn`. |
| **Engine AGENTS.md** | ✅ Sudah diupdate | Sudah mencerminkan arsitektur FastAPI. Flet rules sudah dihapus. |
| **Engine README.md** | ✅ Sudah diupdate | Sudah mendeskripsikan engine sebagai FastAPI server. |
| **TODO.md (root)** | ✅ Roadmap lengkap | 6 fase development dengan detail teknis Rust + Python + Vue. |

### 0.2 Apa yang BELUM ADA (Belum Diimplementasi)

| Komponen | Status |
|----------|--------|
| Modul Rust `paths.rs`, `engine.rs`, `health.rs`, `commands.rs`, `bootstrapper.rs`, `monitor.rs` | ❌ Belum dibuat |
| Python `server.py` (FastAPI entry point) | ❌ Belum dibuat |
| Python `api/` folder (health, clipper, subtitle, upload routers) | ❌ Belum dibuat |
| Python `api/job_manager.py` | ❌ Belum dibuat |
| Vue components (EngineStatus, BootstrapProgress, LogViewer, dll.) | ❌ Belum dibuat |
| Vue stores (Pinia: engine, clipper, settings) | ❌ Belum dibuat |
| Vue views (Dashboard, Compilation, Upload, Settings) | ❌ Belum dibuat |
| Vue composables (`useEngine.ts`) | ❌ Belum dibuat |
| Tailwind CSS / styling framework | ❌ Belum disetup |
| Pinia / Vue Router | ❌ Belum diinstal |
| Rust dependencies (tokio, reqwest, sysinfo, dll.) | ❌ Belum ditambahkan ke Cargo.toml |
| Engine zip build script | ❌ Belum dibuat |
| CI/CD workflows | ❌ Belum dibuat |

### 0.3 Dependensi yang SUDAH TERINSTAL

**Frontend (package.json):**
- `vue` ^3.5.13, `@tauri-apps/api` ^2, `@tauri-apps/plugin-opener` ^2
- Dev: `vite` ^8, `vue-tsc`, `typescript` ~6, `@vitejs/plugin-vue`, `@tauri-apps/cli` ^2

**Rust (Cargo.toml):**
- `tauri` v2, `tauri-plugin-opener` v2, `serde` v1, `serde_json` v1
- Build: `tauri-build` v2

**Python (pyproject.toml):**
- Python ≥3.13, `uv` sebagai package manager
- Lihat file `src-tauri/engine/pyproject.toml` untuk daftar lengkap (~40+ dependensi)

---

## 🏗️ BAGIAN 1: ARSITEKTUR SISTEM

### 1.1 Diagram Arsitektur

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
│  │          Rust Orchestrator (Tauri Backend)           │  │
│  │  ┌────────────┐ ┌────────┐ ┌──────────────────┐    │  │
│  │  │ engine.rs  │ │health  │ │  bootstrapper.rs │    │  │
│  │  │ (subprocess│ │.rs     │ │  (download +     │    │  │
│  │  │  manager)  │ │        │ │   extract zip)   │    │  │
│  │  └──────┬─────┘ └────────┘ └──────────────────┘    │  │
│  └─────────┼──────────────────────────────────────────┘  │
└────────────┼─────────────────────────────────────────────┘
             │ HTTP (127.0.0.1:<port>)
             │ std::process::Command (child process)
┌────────────▼─────────────────────────────────────────────┐
│         Python FastAPI Server (Child Process)             │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐   │
│  │ /health  │ │ /clipper │ │/subtitle │ │ /upload  │   │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘   │
│  ┌────────────────────────────────────────────────────┐  │
│  │   core/ (Whisper, yt-dlp, FFmpeg, DeepFace,       │  │
│  │    Torch, Kokoro TTS, video processing)           │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
```

### 1.2 Tiga Lapisan Utama

| Layer | Teknologi | Lokasi | Peran |
|-------|-----------|--------|-------|
| **Frontend** | Vue 3 + TypeScript + Vite | `src/` | UI, input user, visualisasi progress |
| **Orchestrator** | Rust (Tauri v2) | `src-tauri/src/` | "Mandor" — spawn/kill Python, health check, proxy HTTP, bootstrapper |
| **AI Engine** | Python (FastAPI) | `src-tauri/engine/` | Pemrosesan berat: Whisper, FFmpeg, yt-dlp, DeepFace, face tracking |

### 1.3 Pola Komunikasi

```
Vue ──invoke()──▶ Rust ──HTTP──▶ Python FastAPI
Vue ◀──event()── Rust ◀──stdout── Python (log stream)
Vue ◀──event()── Rust           (bootstrap-progress, engine-status)
```

- **Frontend → Rust**: Tauri `invoke()` commands (`start_engine`, `stop_engine`, `proxy_request`).
- **Rust → Python**: `std::process::Command` untuk spawn, `reqwest` untuk HTTP requests ke FastAPI.
- **Python → Rust**: stdout/stderr dibaca oleh Rust, diteruskan ke frontend via Tauri events.
- **Rust → Frontend**: Tauri `emit()` events (`bootstrap-progress`, `engine-log`, `engine-status`).

### 1.4 Dev vs Prod Path Strategy

| Konteks | Engine Location | Python Executable |
|---------|-----------------|-------------------|
| **Development** (`cfg!(debug_assertions)`) | `<CARGO_MANIFEST_DIR>/engine/` (submodule) | `.venv/bin/python` (dari `uv sync`) |
| **Production** (release build) | `<AppData>/com.dickymuliafiqri.cliptzy/engine/` (downloaded zip) | `engine/python/bin/python3` (portable) |

---

## 🚫 BAGIAN 2: LARANGAN UTAMA (STRICT PROHIBITIONS)

### 2.1 Larangan Umum (Semua Layer)

1. **DILARANG hardcode path absolut** — Gunakan `paths.rs` (Rust), `core/utils.py` (Python), atau `import.meta.env` (Vue).
2. **DILARANG menebak/mengarang API atau library yang tidak ada** — Verifikasi dari source code aktual atau dokumentasi resmi.
3. **DILARANG mengubah arsitektur tanpa memperbarui dokumentasi** — Semua perubahan arsitektur WAJIB disinkronkan ke `AGENTS.md`, `TODO.md`, dan `SKILLS.md`.
4. **DILARANG menambah dependensi tanpa prosedur resmi** — Rust: edit `Cargo.toml` manual. Python: `uv add`. Frontend: `bun add`.

### 2.2 Larangan Frontend (Vue + TypeScript)

1. **DILARANG mengakses Python API langsung dari Vue** — Semua HTTP request ke Python **HARUS** melalui Rust proxy (`invoke('proxy_request', ...)`). Ini karena port FastAPI bersifat dinamis dan tidak boleh di-expose ke webview.
2. **DILARANG menggunakan `fetch()` atau `axios` untuk berkomunikasi dengan engine** — Gunakan `invoke()` dari `@tauri-apps/api/core`.
3. **DILARANG menggunakan React, Svelte, atau framework lain** — Proyek ini menggunakan **Vue 3** dengan `<script setup>` composition API.
4. **DILARANG menambah state management selain Pinia** — Tidak ada Vuex, tidak ada Redux.

### 2.3 Larangan Rust (Tauri Backend)

1. **DILARANG menjalankan Python tanpa melalui `EngineManager`** — Semua interaksi dengan Python child process harus melalui modul `engine.rs`.
2. **DILARANG membiarkan proses Python berjalan saat window ditutup** — `EngineManager::stop()` WAJIB dipanggil di `on_window_event(Destroyed)`.
3. **DILARANG menggunakan `unwrap()` pada operasi I/O di production** — Gunakan proper error handling (`Result`, `?`, `.map_err()`).
4. **DILARANG binding Python server ke `0.0.0.0`** — HARUS selalu `127.0.0.1`.

### 2.4 Larangan Python Engine

> Lihat `src-tauri/engine/AGENTS.md` untuk aturan lengkap Python engine.

Ringkasan:
1. **DILARANG mengimpor library GUI** (flet, PyQt6, tkinter, dll.)
2. **DILARANG menggunakan `sys.executable`** untuk subprocess Python/pip.
3. **DILARANG membuat endpoint CLI interactive** — Hanya `server.py --port --host`.
4. **DILARANG blocking I/O di API handler** — Gunakan `asyncio.to_thread()`.

---

## 📁 BAGIAN 3: PETA FILE PROYEK (FILE MAP)

```
cliptzy/                          # Root proyek
├── AGENTS.md                     # ← ANDA SEDANG MEMBACA INI
├── SKILLS.md                     # Panduan teknis implementasi per-skill
├── TODO.md                       # Roadmap 6 fase dengan checklist
├── README.md                     # Deskripsi proyek (perlu diupdate)
├── index.html                    # HTML entry point untuk Vite
├── package.json                  # Frontend dependencies (Vue, Tauri API)
├── tsconfig.json                 # TypeScript configuration
├── tsconfig.node.json            # TypeScript config untuk Node tools
├── vite.config.ts                # Vite configuration (port 1420)
├── .gitmodules                   # Git submodule → src-tauri/engine
├── .gitignore                    # Git ignore rules
│
├── src/                          # 🟢 FRONTEND (Vue 3)
│   ├── App.vue                   # ⚠️ SAAT INI: template demo greet
│   ├── main.ts                   # Vue app bootstrap (createApp)
│   ├── vite-env.d.ts             # Vite type declarations
│   └── assets/                   # Static assets (SVG logos)
│   # ---- BELUM DIBUAT (lihat TODO Fase 4) ----
│   # ├── router/index.ts
│   # ├── stores/{engine,clipper,settings}.ts
│   # ├── composables/useEngine.ts
│   # ├── components/{EngineStatus,BootstrapProgress,LogViewer,...}.vue
│   # └── views/{Dashboard,Compilation,Upload,Settings}View.vue
│
├── src-tauri/                    # 🟠 TAURI + RUST
│   ├── Cargo.toml                # Rust dependencies (MINIMAL saat ini)
│   ├── Cargo.lock                # Rust lockfile
│   ├── build.rs                  # Tauri build script (minimal)
│   ├── tauri.conf.json           # Tauri config (identifier, window, bundle)
│   ├── capabilities/             # Tauri v2 capability permissions
│   ├── icons/                    # App icons (icns, ico, png)
│   ├── gen/                      # Tauri generated files
│   ├── target/                   # Rust build artifacts (gitignored)
│   │
│   ├── src/                      # 🔶 RUST SOURCE CODE
│   │   ├── main.rs               # Entry point (calls cliptzy_lib::run())
│   │   └── lib.rs                # ⚠️ SAAT INI: hanya fungsi greet()
│   │   # ---- BELUM DIBUAT (lihat TODO Fase 3) ----
│   │   # ├── paths.rs            # Dev vs Prod path resolution
│   │   # ├── engine.rs           # Python subprocess manager
│   │   # ├── health.rs           # Health check polling
│   │   # ├── commands.rs         # Tauri command handlers
│   │   # ├── bootstrapper.rs     # Engine zip download & extract
│   │   # └── monitor.rs          # sysinfo process monitoring
│   │
│   └── engine/                   # 🔵 PYTHON ENGINE (Git Submodule)
│       ├── AGENTS.md             # ✅ Aturan Python engine (sudah FastAPI)
│       ├── README.md             # ✅ Deskripsi engine (sudah FastAPI)
│       ├── ARCHITECTURE.md       # ⚠️ Perlu update ke FastAPI
│       ├── CHANGELOG.md          # Riwayat perubahan
│       ├── pyproject.toml        # ⚠️ Masih ada deps Flet
│       ├── config.json           # Konfigurasi user defaults
│       ├── main.py               # ⚠️ LEGACY — akan diganti server.py
│       ├── gui/                   # ⚠️ LEGACY FLET — HARUS DIHAPUS
│       ├── core/                 # ✅ Engine core (mature, ~30 modul)
│       │   ├── __init__.py       # Public API exports
│       │   ├── config.py         # AppConfig (17KB, lengkap)
│       │   ├── controller.py     # ClipController (10KB)
│       │   ├── processor.py      # Video processing (22KB)
│       │   ├── subtitle.py       # Whisper + ASS (20KB)
│       │   ├── face_tracker.py   # DeepFace tracking (20KB)
│       │   ├── youtube.py        # yt-dlp integration
│       │   ├── ffmpeg.py         # FFmpeg wrapper
│       │   ├── logger.py         # Centralized logging
│       │   ├── utils.py          # Path helpers (12KB)
│       │   ├── video_effects.py  # Emotion-based effects
│       │   ├── ai/               # LLM highlight detection
│       │   ├── processing/       # Video pipeline (13 modul)
│       │   ├── uploaders/        # YouTube/TikTok/Instagram
│       │   └── use_cases/        # Business logic (7 use cases)
│       ├── fonts/                # Subtitle fonts
│       ├── assets/               # Static assets
│       └── tests/                # Python tests
│       # ---- BELUM DIBUAT (lihat TODO Fase 2) ----
│       # ├── server.py           # FastAPI entry point
│       # └── api/                # API routers
```

---

## ⚙️ BAGIAN 4: ATURAN PER TEKNOLOGI

### 4.1 Aturan Vue 3 + TypeScript

1. **Composition API only** — Gunakan `<script setup lang="ts">`. DILARANG Options API.
2. **Type Safety** — Semua props, emits, dan composable return values WAJIB di-type.
3. **Tauri API imports**:
   - `invoke`: `import { invoke } from '@tauri-apps/api/core'`
   - `listen`: `import { listen } from '@tauri-apps/api/event'`
   - DILARANG mengimpor dari path `@tauri-apps/api` tanpa sub-path.
4. **Styling** — Gunakan Tailwind CSS (belum disetup, lihat TODO Fase 4.1).
5. **State Management** — Pinia stores. Satu store per domain: `engine.ts`, `clipper.ts`, `settings.ts`.
6. **Build tool** — Vite v8. Dev server port: `1420` (fixed, jangan ubah).
7. **Package manager** — `bun` (sesuai `tauri.conf.json`: `beforeDevCommand: "bun run dev"`).

### 4.2 Aturan Rust (Tauri v2)

1. **Tauri v2 API** — Proyek ini menggunakan Tauri **v2**, bukan v1. Perhatikan breaking changes:
   - `tauri::Manager` trait untuk `app.emit()`.
   - `State<'_, T>` untuk managed state.
   - `#[tauri::command]` dengan async support.
   - `on_window_event` callback signature.
2. **Modul Structure** — Setiap concern di file terpisah: `paths.rs`, `engine.rs`, `health.rs`, `commands.rs`, `bootstrapper.rs`, `monitor.rs`. Register via `mod` di `lib.rs`.
3. **Error Handling** — Semua Tauri commands return `Result<T, String>`. Gunakan `.map_err(|e| e.to_string())` untuk konversi error.
4. **Async** — Tauri commands yang melakukan I/O WAJIB `async`. Gunakan `tokio` runtime (sudah terintegrasi di Tauri).
5. **Child Process** — Gunakan `std::process::Command` (bukan `tokio::process::Command`) karena Tauri sudah menyediakan thread pool. Pipe stdout/stderr untuk log bridging.
6. **Crate lib name** — `cliptzy_lib` (bukan `cliptzy`). Lihat `Cargo.toml` `[lib] name`.

### 4.3 Aturan Python Engine

> **Sumber otoritatif**: `src-tauri/engine/AGENTS.md`

Ringkasan kunci:
1. **Entry point**: `server.py` (FastAPI + uvicorn). Bukan `main.py`.
2. **Layer separation**: `api/` (routers) → `core/controller.py` (orchestrator) → `core/` (engine modules).
3. **Package manager**: `uv` (Astral). DILARANG `pip install` manual.
4. **Python version**: ≥3.13.
5. **Long-running jobs**: Return `202 Accepted` + `job_id`, client polls progress.

---

## 🔄 BAGIAN 5: ALUR KERJA (WORKFLOWS)

### 5.1 Development Workflow

```bash
# 1. Clone + setup submodule
git clone https://github.com/cliptzy/cliptzy
cd cliptzy
git submodule update --init --recursive

# 2. Setup Python engine
cd src-tauri/engine
uv sync
cd ../..

# 3. Setup frontend
bun install

# 4. Jalankan development mode
cargo tauri dev
# Ini akan:
#   - Menjalankan `bun run dev` (Vite dev server port 1420)
#   - Compile + run Rust backend
#   - Rust akan spawn Python server (dari .venv)
```

### 5.2 Production Build Workflow

```bash
# 1. Build frontend
bun run build

# 2. Build Tauri (release)
cargo tauri build
# Output: .exe / .dmg / .AppImage (TANPA engine Python)

# 3. Build engine zip (terpisah)
cd src-tauri/engine
python scripts/build_engine_zip.py
# Output: engine.zip (Portable Python + deps + scripts)
# Upload ke releases.cliptzy.com
```

### 5.3 User First-Run Flow

```
User double-click cliptzy.exe
  → Rust: cek engine_dir exists?
  → TIDAK ADA → Tampilkan "Downloading AI Engine..."
    → Download engine.zip dari server
    → Extract ke AppData/com.dickymuliafiqri.cliptzy/engine/
    → Set executable permissions (Unix)
  → ADA → Spawn Python: python server.py --port <random>
  → Health check polling (max 60 attempts, 500ms interval)
  → Engine ready → UI Dashboard aktif
```

---

## 🧪 BAGIAN 6: ATURAN VERIFIKASI & QUALITY GATES

### 6.1 Sebelum Commit

- [ ] **Rust**: `cargo clippy --all-targets` — 0 warnings.
- [ ] **Rust**: `cargo build` — sukses tanpa error.
- [ ] **TypeScript**: `bun run build` (includes `vue-tsc --noEmit`) — 0 errors.
- [ ] **Python**: `make typecheck` (di engine/) — 0 errors.
- [ ] Tidak ada hardcode path absolut.
- [ ] Tidak ada import library yang salah layer (mis. `flet` di `core/`).
- [ ] Dokumentasi sinkron dengan perubahan.

### 6.2 Sebelum Merge

- [ ] `cargo tauri dev` berjalan tanpa crash.
- [ ] Python engine bisa distart dan health check respond `200 OK`.
- [ ] Frontend bisa berkomunikasi dengan engine via Rust proxy.
- [ ] Graceful shutdown bekerja (tidak ada zombie process).

---

## 📌 BAGIAN 7: KESALAHAN YANG HARUS DIHINDARI (ANTI-PATTERNS)

### 7.1 Halusinasi yang Sering Terjadi

| ❌ Halusinasi | ✅ Realita |
|---|---|
| "Sudah ada file `engine.rs`" | BELUM ADA. `src-tauri/src/` hanya berisi `lib.rs` dan `main.rs`. |
| "Sudah ada `server.py` di engine" | BELUM ADA. Entry point saat ini masih `main.py` (legacy Flet+CLI). |
| "Frontend sudah punya Dashboard" | BELUM. `src/App.vue` masih template demo (greet function). |
| "Pinia sudah disetup" | BELUM. `package.json` belum punya `pinia` atau `vue-router`. |
| "Cargo.toml sudah punya tokio" | BELUM. Hanya `tauri`, `tauri-plugin-opener`, `serde`, `serde_json`. |
| "Python engine sudah FastAPI" | BELUM. `pyproject.toml` belum punya `fastapi`/`uvicorn`. Core ready, tapi API layer belum dibuat. |
| "Folder `gui/` sudah dihapus" | BELUM. `src-tauri/engine/gui/` masih ada lengkap dengan kode Flet. |

### 7.2 Anti-Pattern Teknis

1. **Jangan panggil `std::process::Command` langsung di Tauri command** — Wrap di `EngineManager`.
2. **Jangan buat HTTP client baru di setiap request** — Reuse `reqwest::Client` (simpan di state).
3. **Jangan polling dari frontend** — Gunakan Tauri events (`listen()`) untuk push updates.
4. **Jangan simpan port Python di frontend** — Port dikelola oleh Rust, frontend hanya panggil `proxy_request`.
5. **Jangan jalankan `uv sync` di production** — Semua deps harus sudah ada di Portable Python bundle.

---

## 🗺️ BAGIAN 8: REFERENSI DOKUMEN TERKAIT

| Dokumen | Lokasi | Fungsi |
|---------|--------|--------|
| **TODO.md** | `./TODO.md` | Roadmap 6 fase, checklist detail |
| **SKILLS.md** | `./SKILLS.md` | Panduan implementasi teknis per-skill |
| **Engine AGENTS.md** | `src-tauri/engine/AGENTS.md` | Aturan khusus Python engine |
| **Engine README.md** | `src-tauri/engine/README.md` | Deskripsi & API docs engine |
| **Engine ARCHITECTURE.md** | `src-tauri/engine/ARCHITECTURE.md` | Arsitektur internal engine |
| **tauri.conf.json** | `src-tauri/tauri.conf.json` | Konfigurasi Tauri (identifier, window, bundle) |
| **Cargo.toml** | `src-tauri/Cargo.toml` | Dependensi Rust |
| **package.json** | `./package.json` | Dependensi frontend |
| **pyproject.toml** | `src-tauri/engine/pyproject.toml` | Dependensi Python |

---

## 📋 BAGIAN 9: CHECKLIST AI SEBELUM MULAI BEKERJA

Sebelum menulis kode, AI Model WAJIB menjawab pertanyaan berikut:

1. ✅ Apakah saya sudah membaca `AGENTS.md` (dokumen ini)?
2. ✅ Apakah saya sudah membaca `TODO.md` untuk tahu fase mana yang sedang dikerjakan?
3. ✅ Apakah saya sudah membaca `SKILLS.md` untuk tahu panduan teknis spesifik?
4. ✅ Apakah saya tahu **file mana yang sudah ada** vs **yang belum dibuat**? (Lihat Bagian 0)
5. ✅ Apakah saya tahu **dependensi mana yang sudah terinstal** vs **yang perlu ditambah**? (Lihat Bagian 0.3)
6. ✅ Apakah perubahan saya menyentuh file di **layer yang benar**? (Vue/Rust/Python)
7. ✅ Apakah saya sudah cek **engine/AGENTS.md** jika bekerja di kode Python?
8. ✅ Apakah saya **tidak** berhalusinasi tentang keberadaan file/modul? (Lihat Bagian 7.1)

---

_Dokumen ini terakhir diperbarui: 2026-08-25. Sinkronkan dengan kode aktual setiap ada perubahan signifikan._
