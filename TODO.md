# 📋 TODO.md — Cliptzy Desktop (Hybrid Architecture Roadmap)

> **Arsitektur**: Tauri (Rust Orchestrator) + Vue 3 (Frontend) + Python FastAPI (AI Engine via Git Submodule)
>
> **Pola Desain**: Bootstrapper Pattern — Engine Python tidak di-bundle dalam `.exe`, melainkan diunduh saat first-run sebagai Portable Python + Dependencies `.zip`.

---

## Fase 1: Inisialisasi Proyek & Setup Submodule (Development Environment)

### 1.1 Struktur Proyek Utama (Tauri + Vue)

- [ ] Verifikasi inisialisasi proyek Tauri + Vue 3 + TypeScript (sudah ada di root).
- [ ] Pastikan `tauri.conf.json` memiliki konfigurasi yang benar:
  - `productName`: `"cliptzy"`
  - `identifier`: `"com.dickymuliafiqri.cliptzy"`
  - `beforeDevCommand`: `"bun run dev"`
  - `devUrl`: `"http://localhost:1420"`
- [ ] Tambahkan dependensi Rust yang dibutuhkan ke `src-tauri/Cargo.toml`:
  - `tokio = { version = "1", features = ["full"] }` — async runtime.
  - `reqwest = { version = "0.12", features = ["stream"] }` — HTTP client untuk download engine zip.
  - `zip = "2"` — ekstraksi file `.zip`.
  - `sysinfo = "0.35"` — monitoring proses (CPU, RAM, kill process).
  - `portpicker = "0.1"` — pencarian port bebas untuk FastAPI.
  - `tauri-plugin-shell = "2"` — spawn child process dari Tauri.
  - `dirs = "6"` — lokasi `AppData` / `Application Support` cross-platform.
  - `tracing = "0.1"` + `tracing-subscriber = "0.3"` — structured logging.

### 1.2 Setup Git Submodule untuk Engine Python

- [ ] Verifikasi `.gitmodules` sudah benar mengarah ke `https://github.com/cliptzy/engine` pada path `src-tauri/engine`.
- [ ] Jalankan `git submodule update --init --recursive` untuk memastikan submodule ter-clone.
- [ ] Pastikan `.gitignore` root tidak meng-ignore folder `src-tauri/engine/` (karena ini adalah submodule, bukan folder biasa).
- [ ] Tambahkan entry di `.gitignore` root:
  ```
  # Python engine artifacts (jangan commit ke repo utama)
  src-tauri/engine/.venv/
  src-tauri/engine/__pycache__/
  src-tauri/engine/clips/
  src-tauri/engine/logs/
  ```

### 1.3 Setup Development Environment Python

- [ ] Pastikan `uv` (Astral) terinstal di mesin developer.
- [ ] Jalankan `cd src-tauri/engine && uv sync` untuk membuat `.venv` dan menginstal semua dependensi.
- [ ] Verifikasi Python engine bisa berjalan secara standalone: `cd src-tauri/engine && uv run python -c "from core import config; print(config)"`.

### 1.4 Konfigurasi Dev vs Prod Path Resolution di Rust

- [ ] Buat modul Rust `src-tauri/src/paths.rs` yang menangani resolusi path:
  ```rust
  // paths.rs — Centralized Path Resolution
  use std::path::PathBuf;
  use dirs;

  /// Mengembalikan path ke direktori engine.
  /// - DEV: `<project_root>/src-tauri/engine/`
  /// - PROD: `<AppData>/com.dickymuliafiqri.cliptzy/engine/`
  pub fn engine_dir() -> PathBuf {
      if cfg!(debug_assertions) {
          // Development: engine ada di submodule lokal
          let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
              .unwrap_or_else(|_| ".".to_string());
          PathBuf::from(manifest_dir).join("engine")
      } else {
          // Production: engine di-download ke AppData
          let app_data = dirs::data_local_dir()
              .expect("Cannot determine AppData directory");
          app_data.join("com.dickymuliafiqri.cliptzy").join("engine")
      }
  }

  /// Path ke Python executable.
  /// - DEV: Menggunakan Python dari `.venv` submodule atau sistem.
  /// - PROD: Menggunakan Portable Python di dalam engine dir.
  pub fn python_executable() -> PathBuf {
      let engine = engine_dir();
      if cfg!(debug_assertions) {
          // Dev: gunakan uv run atau python dari .venv
          let venv_python = if cfg!(target_os = "windows") {
              engine.join(".venv").join("Scripts").join("python.exe")
          } else {
              engine.join(".venv").join("bin").join("python")
          };
          if venv_python.exists() {
              venv_python
          } else {
              PathBuf::from("python3") // fallback ke sistem
          }
      } else {
          // Prod: Portable Python di dalam engine
          if cfg!(target_os = "windows") {
              engine.join("python").join("python.exe")
          } else {
              engine.join("python").join("bin").join("python3")
          }
      }
  }
  ```
- [ ] Register `mod paths;` di `src-tauri/src/lib.rs`.

---

## Fase 2: Pengembangan Python Engine (FastAPI Server)

> **PENTING**: Engine Python di-refactor dari Flet Desktop App menjadi **FastAPI REST API Server** murni.
> Folder `gui/` (Flet UI) akan dihapus. Semua interaksi dilakukan melalui HTTP endpoint.
> Tidak ada eksekusi CLI (`main.py --blablabla`). Hanya ada satu entry point: `uvicorn` server.

### 2.1 Migrasi Arsitektur: Flet → FastAPI

- [ ] Hapus seluruh folder `gui/` dari engine (Flet UI tidak lagi digunakan).
- [ ] Hapus dependensi Flet dari `pyproject.toml`:
  - Hapus: `flet`, `flet-video`, `flet-audio`, `pystray`, `desktop-notifier`.
- [ ] Tambahkan dependensi FastAPI:
  - `uv add fastapi uvicorn[standard]`
- [ ] Hapus file `main.py` lama (yang berisi CLI argparse + Flet launcher).
- [ ] Hapus file legacy: `run.bat`, `run.sh`, `build_executable.py`, `cliptzy.spec`.
- [ ] Perbarui `AGENTS.md` — hapus semua aturan terkait Flet, tambah aturan FastAPI.
- [ ] Perbarui `README.md` — ubah deskripsi menjadi "AI Engine API Server".
- [ ] Perbarui `ARCHITECTURE.md` — refleksikan arsitektur FastAPI baru.

### 2.2 Implementasi FastAPI Server Entry Point

- [ ] Buat `src-tauri/engine/server.py` sebagai entry point utama:
  ```python
  # server.py — Cliptzy AI Engine FastAPI Server
  import uvicorn
  from fastapi import FastAPI
  from contextlib import asynccontextmanager
  import signal
  import sys

  @asynccontextmanager
  async def lifespan(app: FastAPI):
      # Startup: preload heavy models
      from core import config
      from core.logger import log
      log.info("Cliptzy Engine starting up...")
      # Lazy-load models saat diperlukan, bukan di startup
      yield
      # Shutdown: cleanup
      log.info("Cliptzy Engine shutting down...")

  app = FastAPI(
      title="Cliptzy AI Engine",
      version="4.0.0",
      lifespan=lifespan
  )

  # Register routers
  from api.health import router as health_router
  from api.clipper import router as clipper_router
  from api.subtitle import router as subtitle_router
  from api.upload import router as upload_router

  app.include_router(health_router)
  app.include_router(clipper_router)
  app.include_router(subtitle_router)
  app.include_router(upload_router)

  if __name__ == "__main__":
      import argparse
      parser = argparse.ArgumentParser()
      parser.add_argument("--port", type=int, default=9721)
      parser.add_argument("--host", type=str, default="127.0.0.1")
      args = parser.parse_args()
      uvicorn.run(app, host=args.host, port=args.port)
  ```
- [ ] Pastikan server hanya bind ke `127.0.0.1` (localhost only, bukan `0.0.0.0`) untuk keamanan.

### 2.3 Implementasi API Endpoints

- [ ] Buat folder `src-tauri/engine/api/` dengan `__init__.py`.
- [ ] **`api/health.py`** — Health Check & System Info:
  ```python
  @router.get("/health")
  async def health_check():
      return {"status": "ok", "version": "4.0.0"}

  @router.get("/health/models")
  async def model_status():
      # Cek apakah Whisper model sudah ter-cache, GPU tersedia, dll.
      return {"whisper_loaded": ..., "gpu_available": ..., "ffmpeg_available": ...}
  ```
- [ ] **`api/clipper.py`** — Endpoint pemrosesan klip video:
  - `POST /clipper/analyze` — Analisis heatmap YouTube, return daftar segmen.
  - `POST /clipper/process` — Proses single clip (crop, subtitle, effects).
  - `POST /clipper/compile` — Proses kompilasi multi-clip.
  - `GET /clipper/progress/{job_id}` — Polling status progress job (SSE atau WebSocket).
  - `POST /clipper/cancel/{job_id}` — Membatalkan job yang sedang berjalan.
- [ ] **`api/subtitle.py`** — Endpoint transkripsi Whisper:
  - `POST /subtitle/transcribe` — Transkripsi audio file, return segmen + file ASS.
  - `GET /subtitle/models` — List model Whisper yang tersedia + status download.
  - `POST /subtitle/models/download` — Trigger download model Whisper tertentu.
- [ ] **`api/upload.py`** — Endpoint auto-upload:
  - `POST /upload/youtube` — Upload ke YouTube Shorts.
  - `POST /upload/tiktok` — Upload ke TikTok.
  - `POST /upload/instagram` — Upload ke Instagram Reels.
  - `GET /upload/status/{job_id}` — Status upload.

### 2.4 Sistem Job Queue & Progress Reporting

- [ ] Buat `src-tauri/engine/api/job_manager.py`:
  - Implementasi in-memory job queue menggunakan `asyncio.Queue` + `dict[str, JobStatus]`.
  - Setiap job memiliki: `job_id`, `status` (queued/running/completed/failed/cancelled), `progress` (0-100), `message`, `result`.
  - Support SSE (Server-Sent Events) untuk streaming progress ke Rust client.
- [ ] Migrate logika dari `core/controller.py` ke API endpoints — controller menjadi internal orchestrator yang dipanggil oleh API layer.

### 2.5 Adaptasi Core Modules

- [ ] Pastikan `core/` tetap bersih dari dependensi GUI (sudah terpenuhi berdasarkan ARCHITECTURE.md).
- [ ] Refactor `core/controller.py`:
  - Hapus semua referensi ke `event_hook` yang terkait Flet event bus.
  - Ganti dengan callback pattern yang bisa dikonsumsi oleh API layer (atau emit ke job_manager).
- [ ] Refactor `core/logger.py`:
  - Hapus `EventBusLogHandler` (Flet-specific).
  - Tambahkan handler yang menulis log ke file + stdout.
  - Opsional: tambahkan SSE log stream endpoint di `api/health.py`.
- [ ] Pastikan `core/bootstrap.py` tidak mengimpor modul Flet.
- [ ] Verifikasi semua `core/` imports tidak lagi memiliki dependensi ke `flet` atau `gui/`.

### 2.6 Testing API Server

- [ ] Buat `tests/test_api.py` menggunakan `httpx` + `pytest` untuk testing endpoint FastAPI.
- [ ] Test health check: `GET /health` returns `{"status": "ok"}`.
- [ ] Test clipper analyze: `POST /clipper/analyze` dengan URL YouTube valid.
- [ ] Verifikasi server bisa dijalankan: `cd src-tauri/engine && uv run python server.py --port 9721`.

---

## Fase 3: Pengembangan Rust Orchestrator (Subprocess Management, Health Check, Graceful Shutdown)

> **Peran Rust**: Bertindak sebagai "Mandor" yang:
> 1. Meluncurkan proses Python (FastAPI server) sebagai child process.
> 2. Menunggu server siap (health check polling).
> 3. Meneruskan request dari Frontend ke Python via HTTP.
> 4. Mematikan proses Python saat aplikasi ditutup (graceful shutdown).

### 3.1 Modul Engine Process Manager

- [ ] Buat `src-tauri/src/engine.rs` — Manager lifecycle Python engine:
  ```rust
  // engine.rs — Python Engine Subprocess Manager
  use std::process::{Child, Command, Stdio};
  use std::sync::Mutex;
  use tokio::time::{sleep, Duration};

  pub struct EngineManager {
      process: Mutex<Option<Child>>,
      port: u16,
  }

  impl EngineManager {
      pub fn new() -> Self {
          Self {
              process: Mutex::new(None),
              port: 0,
          }
      }

      /// Memulai Python FastAPI server sebagai child process.
      pub fn start(&self) -> Result<u16, String> {
          let port = portpicker::pick_unused_port()
              .ok_or("No available port found")?;

          let python = crate::paths::python_executable();
          let engine_dir = crate::paths::engine_dir();
          let server_script = engine_dir.join("server.py");

          let child = Command::new(&python)
              .arg(&server_script)
              .arg("--port")
              .arg(port.to_string())
              .arg("--host")
              .arg("127.0.0.1")
              .current_dir(&engine_dir)
              .stdout(Stdio::piped())
              .stderr(Stdio::piped())
              .spawn()
              .map_err(|e| format!("Failed to start engine: {}", e))?;

          *self.process.lock().unwrap() = Some(child);
          // self.port diset setelah start
          Ok(port)
      }

      /// Mematikan proses Python secara graceful.
      pub fn stop(&self) {
          if let Some(mut child) = self.process.lock().unwrap().take() {
              // Kirim SIGTERM (Unix) atau taskkill (Windows)
              #[cfg(unix)]
              {
                  unsafe {
                      libc::kill(child.id() as i32, libc::SIGTERM);
                  }
              }
              #[cfg(windows)]
              {
                  let _ = child.kill(); // Windows: langsung kill
              }

              // Tunggu max 5 detik, lalu force kill
              let timeout = std::time::Duration::from_secs(5);
              let start = std::time::Instant::now();
              loop {
                  match child.try_wait() {
                      Ok(Some(_)) => break, // Sudah exit
                      Ok(None) => {
                          if start.elapsed() > timeout {
                              let _ = child.kill(); // Force kill
                              break;
                          }
                          std::thread::sleep(std::time::Duration::from_millis(100));
                      }
                      Err(_) => break,
                  }
              }
          }
      }
  }
  ```
- [ ] Tambahkan `libc = "0.2"` ke `Cargo.toml` (untuk SIGTERM di Unix).

### 3.2 Health Check Polling

- [ ] Buat `src-tauri/src/health.rs` — Modul health check:
  ```rust
  // health.rs — Engine Health Check
  use reqwest::Client;
  use tokio::time::{sleep, Duration, timeout};

  pub async fn wait_for_engine(port: u16, max_retries: u32) -> Result<(), String> {
      let client = Client::new();
      let url = format!("http://127.0.0.1:{}/health", port);

      for attempt in 1..=max_retries {
          match timeout(
              Duration::from_secs(2),
              client.get(&url).send()
          ).await {
              Ok(Ok(response)) if response.status().is_success() => {
                  tracing::info!("Engine ready on port {} (attempt {})", port, attempt);
                  return Ok(());
              }
              _ => {
                  tracing::debug!("Health check attempt {}/{} failed", attempt, max_retries);
                  sleep(Duration::from_millis(500)).await;
              }
          }
      }

      Err(format!("Engine failed to start after {} attempts", max_retries))
  }
  ```

### 3.3 Graceful Shutdown via Tauri Lifecycle

- [ ] Modifikasi `src-tauri/src/lib.rs` untuk mengintegrasikan EngineManager:
  ```rust
  use std::sync::Arc;
  mod paths;
  mod engine;
  mod health;
  mod bootstrapper; // Fase 5
  mod commands;

  #[cfg_attr(mobile, tauri::mobile_entry_point)]
  pub fn run() {
      let engine_manager = Arc::new(engine::EngineManager::new());
      let engine_for_shutdown = engine_manager.clone();

      tauri::Builder::default()
          .plugin(tauri_plugin_opener::init())
          .manage(engine_manager) // State management Tauri
          .invoke_handler(tauri::generate_handler![
              commands::start_engine,
              commands::stop_engine,
              commands::get_engine_status,
              commands::proxy_request,
          ])
          .on_window_event(move |window, event| {
              if let tauri::WindowEvent::Destroyed = event {
                  // CRITICAL: Kill Python process saat window ditutup
                  tracing::info!("Window destroyed, stopping engine...");
                  engine_for_shutdown.stop();
              }
          })
          .run(tauri::generate_context!())
          .expect("error while running tauri application");
  }
  ```
- [ ] Implementasikan juga handler `on_exit` sebagai safety net:
  ```rust
  // Tambahan di Builder:
  .setup(|app| {
      // Gunakan Drop guard atau ctrlc handler sebagai safety net
      let engine = app.state::<Arc<engine::EngineManager>>().inner().clone();
      ctrlc::set_handler(move || {
          engine.stop();
          std::process::exit(0);
      }).ok();
      Ok(())
  })
  ```
- [ ] Tambahkan `ctrlc = "3"` ke `Cargo.toml`.

### 3.4 Tauri Commands (Bridge Frontend ↔ Rust ↔ Python)

- [ ] Buat `src-tauri/src/commands.rs` — Tauri command handlers:
  ```rust
  use std::sync::Arc;
  use tauri::State;
  use crate::engine::EngineManager;

  #[tauri::command]
  pub async fn start_engine(
      engine: State<'_, Arc<EngineManager>>
  ) -> Result<u16, String> {
      let port = engine.start()?;
      crate::health::wait_for_engine(port, 60).await?;
      Ok(port)
  }

  #[tauri::command]
  pub async fn stop_engine(
      engine: State<'_, Arc<EngineManager>>
  ) -> Result<(), String> {
      engine.stop();
      Ok(())
  }

  #[tauri::command]
  pub async fn get_engine_status(
      engine: State<'_, Arc<EngineManager>>
  ) -> Result<String, String> {
      // Cek apakah proses masih jalan + health check
      Ok("running".to_string()) // Simplified
  }

  /// Proxy HTTP request ke Python engine.
  /// Frontend memanggil ini daripada langsung ke localhost
  /// agar port tidak perlu di-expose ke webview.
  #[tauri::command]
  pub async fn proxy_request(
      engine: State<'_, Arc<EngineManager>>,
      method: String,
      path: String,
      body: Option<String>,
  ) -> Result<String, String> {
      let port = engine.port();
      let url = format!("http://127.0.0.1:{}{}", port, path);
      let client = reqwest::Client::new();

      let response = match method.to_uppercase().as_str() {
          "GET" => client.get(&url).send().await,
          "POST" => {
              let mut req = client.post(&url);
              if let Some(b) = body {
                  req = req.header("Content-Type", "application/json").body(b);
              }
              req.send().await
          }
          _ => return Err("Unsupported method".to_string()),
      };

      response
          .map_err(|e| e.to_string())?
          .text()
          .await
          .map_err(|e| e.to_string())
  }
  ```

### 3.5 Monitoring Proses dengan sysinfo

- [ ] Buat `src-tauri/src/monitor.rs` — Resource monitoring:
  - Gunakan crate `sysinfo` untuk memantau penggunaan memori dan CPU dari child process Python.
  - Expose command `get_engine_metrics` ke frontend untuk ditampilkan di status bar.
  - Deteksi jika proses Python crash (exit code non-zero) dan notify frontend.
  ```rust
  use sysinfo::System;

  pub fn get_process_metrics(pid: u32) -> Option<ProcessMetrics> {
      let mut sys = System::new();
      sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[
          sysinfo::Pid::from_u32(pid)
      ]), true);

      sys.process(sysinfo::Pid::from_u32(pid)).map(|p| ProcessMetrics {
          cpu_usage: p.cpu_usage(),
          memory_mb: p.memory() / 1_048_576,
          status: format!("{:?}", p.status()),
      })
  }
  ```

### 3.6 Logging Bridge (Python stdout → Rust → Frontend)

- [ ] Di `engine.rs`, baca `stdout` dan `stderr` dari child process Python secara async.
- [ ] Teruskan log Python ke sistem logging Rust (`tracing`) dan emit sebagai Tauri event ke frontend.
  ```rust
  // Di start():
  let stdout = child.stdout.take().unwrap();
  std::thread::spawn(move || {
      use std::io::BufRead;
      let reader = std::io::BufReader::new(stdout);
      for line in reader.lines() {
          if let Ok(line) = line {
              tracing::info!(target: "python_engine", "{}", line);
              // TODO: emit Tauri event ke frontend
          }
      }
  });
  ```

---

## Fase 4: Pengembangan Frontend & Integrasi State Management

### 4.1 Setup Frontend Framework

- [ ] Install dependensi tambahan Vue:
  - `bun add pinia` — State management.
  - `bun add vue-router` — Client-side routing.
  - `bun add @vueuse/core` — Composable utilities.
  - `bun add -d tailwindcss @tailwindcss/vite` — Styling (opsional, atau gunakan CSS framework lain).
- [ ] Setup Tailwind CSS (atau framework styling pilihan).
- [ ] Buat struktur folder frontend:
  ```
  src/
  ├── App.vue
  ├── main.ts
  ├── router/
  │   └── index.ts
  ├── stores/
  │   ├── engine.ts          // Status engine Python (starting/ready/error)
  │   ├── clipper.ts          // State clipper (segments, jobs, progress)
  │   └── settings.ts         // Pengaturan user
  ├── composables/
  │   └── useEngine.ts        // Composable untuk invoke Tauri commands
  ├── components/
  │   ├── EngineStatus.vue     // Indikator status engine
  │   ├── BootstrapProgress.vue // UI download engine (Fase 5)
  │   ├── LogViewer.vue        // Real-time log viewer
  │   ├── VideoCard.vue        // Card hasil klip
  │   └── ProgressBar.vue      // Progress pemrosesan
  ├── views/
  │   ├── DashboardView.vue    // Halaman utama clipper
  │   ├── CompilationView.vue  // Mode kompilasi
  │   ├── UploadView.vue       // Auto-upload manager
  │   └── SettingsView.vue     // Pengaturan
  └── assets/
      └── styles/
  ```

### 4.2 Engine Lifecycle Integration (Frontend ↔ Rust)

- [ ] Buat `src/composables/useEngine.ts`:
  ```typescript
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';

  export function useEngine() {
    const startEngine = async (): Promise<number> => {
      return await invoke<number>('start_engine');
    };

    const stopEngine = async () => {
      await invoke('stop_engine');
    };

    const proxyRequest = async (
      method: string, path: string, body?: object
    ): Promise<any> => {
      const result = await invoke<string>('proxy_request', {
        method,
        path,
        body: body ? JSON.stringify(body) : null,
      });
      return JSON.parse(result);
    };

    return { startEngine, stopEngine, proxyRequest };
  }
  ```
- [ ] Buat `src/stores/engine.ts` (Pinia store):
  - State: `status` (enum: `idle | starting | downloading | ready | error`), `port`, `errorMessage`.
  - Actions: `initialize()` — cek engine, start, health check.
  - Gunakan `useEngine()` composable.

### 4.3 Implementasi UI Views

- [ ] **`DashboardView.vue`** — Halaman utama:
  - Input URL YouTube.
  - Pilihan crop mode, subtitle, ratio output.
  - Tombol "Analyze" → panggil `POST /clipper/analyze` via proxy.
  - Daftar segmen heatmap yang bisa dipilih.
  - Tombol "Process Selected" → panggil `POST /clipper/process` per segmen.
  - Progress bar real-time per job.
  - Gallery hasil klip (VideoCard components).
- [ ] **`CompilationView.vue`** — Mode kompilasi Top-N:
  - File picker untuk multi-select video lokal.
  - Drag-and-drop reorder list.
  - Input nama momen per video.
  - Dropdown ordering (countdown/countup).
  - Tombol "Generate Compilation".
- [ ] **`UploadView.vue`** — Auto-upload management:
  - List video yang siap upload.
  - Pilihan platform (YouTube/TikTok/Instagram).
  - Status upload per platform.
- [ ] **`SettingsView.vue`** — Pengaturan:
  - Konfigurasi Whisper model.
  - Konfigurasi AI provider (Gemini/OpenAI/Ollama).
  - Konfigurasi FFmpeg path.
  - Konfigurasi subtitle (font, lokasi, delay).

### 4.4 Real-time Progress & Log Viewer

- [ ] Implementasi SSE listener di frontend untuk streaming progress dari Python engine.
- [ ] Buat `LogViewer.vue` — Widget yang menampilkan log dari Python engine secara real-time:
  - Log diterima via Tauri event (yang di-emit oleh Rust dari stdout Python).
  - Auto-scroll ke bawah.
  - Filter berdasarkan level (INFO/WARNING/ERROR).

### 4.5 Startup Flow

- [ ] Implementasikan alur startup di `App.vue`:
  1. Tampilkan splash screen / loading.
  2. Cek apakah engine tersedia (Fase 5: bootstrapper check).
  3. Jika engine tidak ada → tampilkan `BootstrapProgress.vue` (download & extract).
  4. Jika engine ada → panggil `start_engine` Tauri command.
  5. Tunggu health check sukses.
  6. Redirect ke `DashboardView`.

---

## Fase 5: Sistem Bootstrapper (Download & Ekstrak Engine di Rust)

> **Konsep**: Saat production build, folder `engine/` tidak disertakan dalam bundle `.exe`.
> Pada first-run, Rust mendeteksi engine belum ada dan mengunduh `.zip` dari server.

### 5.1 Konfigurasi Build: Exclude Engine dari Bundle

- [ ] Modifikasi `tauri.conf.json` → `bundle.resources` untuk **tidak** menyertakan folder `engine/`.
- [ ] Tambahkan `engine/` ke `src-tauri/.gitignore` untuk build artifacts (bukan submodule itu sendiri).
- [ ] Buat flag build di `build.rs` atau `tauri.conf.json` yang membedakan dev vs prod:
  ```rust
  // build.rs
  fn main() {
      tauri_build::build();
      // Set flag untuk prod build
      println!("cargo:rustc-env=ENGINE_DOWNLOAD_URL=https://releases.cliptzy.com/engine/latest.zip");
      println!("cargo:rustc-env=ENGINE_CHECKSUM_URL=https://releases.cliptzy.com/engine/latest.sha256");
  }
  ```

### 5.2 Implementasi Bootstrapper di Rust

- [ ] Buat `src-tauri/src/bootstrapper.rs`:
  ```rust
  // bootstrapper.rs — Engine Downloader & Extractor
  use std::path::PathBuf;
  use reqwest::Client;
  use tokio::io::AsyncWriteExt;
  use std::io::Read;

  pub struct Bootstrapper {
      engine_dir: PathBuf,
      download_url: String,
      checksum_url: String,
  }

  #[derive(Clone, serde::Serialize)]
  pub struct BootstrapProgress {
      pub stage: String,        // "downloading" | "extracting" | "verifying" | "done" | "error"
      pub progress: f64,        // 0.0 - 1.0
      pub message: String,
      pub bytes_downloaded: u64,
      pub total_bytes: u64,
  }

  impl Bootstrapper {
      pub fn new() -> Self {
          Self {
              engine_dir: crate::paths::engine_dir(),
              download_url: env!("ENGINE_DOWNLOAD_URL").to_string(),
              checksum_url: env!("ENGINE_CHECKSUM_URL").to_string(),
          }
      }

      /// Cek apakah engine sudah terinstall.
      pub fn is_engine_installed(&self) -> bool {
          let server_py = self.engine_dir.join("server.py");
          let python = crate::paths::python_executable();
          server_py.exists() && python.exists()
      }

      /// Download dan extract engine zip.
      /// Emit progress via Tauri event.
      pub async fn bootstrap(
          &self,
          app_handle: tauri::AppHandle,
      ) -> Result<(), String> {
          // 1. Download zip dengan progress tracking
          let zip_path = self.engine_dir.parent().unwrap().join("engine_download.zip");
          self.download_with_progress(&app_handle, &zip_path).await?;

          // 2. Verifikasi checksum SHA-256
          self.verify_checksum(&zip_path).await?;

          // 3. Extract zip ke engine_dir
          self.extract_zip(&app_handle, &zip_path)?;

          // 4. Cleanup zip file
          std::fs::remove_file(&zip_path).ok();

          // 5. Set permission executable (Unix)
          #[cfg(unix)]
          self.set_executable_permissions()?;

          Ok(())
      }

      async fn download_with_progress(
          &self,
          app_handle: &tauri::AppHandle,
          zip_path: &PathBuf,
      ) -> Result<(), String> {
          let client = Client::new();
          let response = client.get(&self.download_url)
              .send().await.map_err(|e| e.to_string())?;

          let total_size = response.content_length().unwrap_or(0);
          let mut downloaded: u64 = 0;

          std::fs::create_dir_all(zip_path.parent().unwrap())
              .map_err(|e| e.to_string())?;
          let mut file = std::fs::File::create(zip_path)
              .map_err(|e| e.to_string())?;

          let mut stream = response.bytes_stream();
          use futures_util::StreamExt;
          while let Some(chunk) = stream.next().await {
              let chunk = chunk.map_err(|e| e.to_string())?;
              std::io::Write::write_all(&mut file, &chunk)
                  .map_err(|e| e.to_string())?;
              downloaded += chunk.len() as u64;

              // Emit progress event ke frontend
              let progress = BootstrapProgress {
                  stage: "downloading".to_string(),
                  progress: if total_size > 0 {
                      downloaded as f64 / total_size as f64
                  } else { 0.0 },
                  message: format!("Downloading AI Engine... {:.1} MB / {:.1} MB",
                      downloaded as f64 / 1_048_576.0,
                      total_size as f64 / 1_048_576.0),
                  bytes_downloaded: downloaded,
                  total_bytes: total_size,
              };
              app_handle.emit("bootstrap-progress", &progress).ok();
          }

          Ok(())
      }

      fn extract_zip(
          &self,
          app_handle: &tauri::AppHandle,
          zip_path: &PathBuf,
      ) -> Result<(), String> {
          let file = std::fs::File::open(zip_path).map_err(|e| e.to_string())?;
          let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
          let total_files = archive.len();

          std::fs::create_dir_all(&self.engine_dir).map_err(|e| e.to_string())?;

          for i in 0..total_files {
              let mut entry = archive.by_index(i).map_err(|e| e.to_string())?;
              let out_path = self.engine_dir.join(
                  entry.mangled_name()
              );

              if entry.is_dir() {
                  std::fs::create_dir_all(&out_path).ok();
              } else {
                  if let Some(parent) = out_path.parent() {
                      std::fs::create_dir_all(parent).ok();
                  }
                  let mut outfile = std::fs::File::create(&out_path)
                      .map_err(|e| e.to_string())?;
                  std::io::copy(&mut entry, &mut outfile)
                      .map_err(|e| e.to_string())?;
              }

              // Emit extract progress
              if i % 100 == 0 || i == total_files - 1 {
                  let progress = BootstrapProgress {
                      stage: "extracting".to_string(),
                      progress: (i + 1) as f64 / total_files as f64,
                      message: format!("Extracting files... {}/{}", i + 1, total_files),
                      bytes_downloaded: 0,
                      total_bytes: 0,
                  };
                  app_handle.emit("bootstrap-progress", &progress).ok();
              }
          }

          Ok(())
      }
  }
  ```
- [ ] Tambahkan `futures-util = "0.3"` ke `Cargo.toml` (untuk `StreamExt`).

### 5.3 Tauri Commands untuk Bootstrapper

- [ ] Tambahkan commands di `commands.rs`:
  ```rust
  #[tauri::command]
  pub async fn check_engine_installed() -> bool {
      let bootstrapper = crate::bootstrapper::Bootstrapper::new();
      bootstrapper.is_engine_installed()
  }

  #[tauri::command]
  pub async fn bootstrap_engine(app_handle: tauri::AppHandle) -> Result<(), String> {
      let bootstrapper = crate::bootstrapper::Bootstrapper::new();
      bootstrapper.bootstrap(app_handle).await
  }
  ```
- [ ] Register commands baru di `lib.rs` invoke_handler.

### 5.4 Frontend Bootstrap UI

- [ ] Buat `src/components/BootstrapProgress.vue`:
  - Listen Tauri event `bootstrap-progress`.
  - Tampilkan progress bar dengan persentase.
  - Tampilkan stage saat ini (Downloading / Extracting / Verifying).
  - Tampilkan ukuran file yang sudah diunduh vs total.
  - Tampilkan pesan error jika gagal, dengan tombol "Retry".

### 5.5 Server-Side: Persiapan Zip Engine

- [ ] Buat script `scripts/build_engine_zip.py` (di repo engine):
  - Package: Portable Python (misal: [python-build-standalone](https://github.com/indygreg/python-build-standalone)) + semua pip dependencies + script engine.
  - Struktur zip:
    ```
    engine.zip/
    ├── python/           # Portable Python runtime
    │   ├── bin/python3   # (Unix) atau python.exe (Windows)
    │   └── lib/          # Standard library + site-packages
    ├── server.py         # Entry point FastAPI
    ├── api/              # API endpoints
    ├── core/             # Engine core modules
    ├── fonts/            # Font assets
    ├── assets/           # Other assets
    └── config.json       # Default config
    ```
  - Generate SHA-256 checksum file.
  - Upload ke CDN/server hosting.
- [ ] Setup hosting untuk engine zip (GitHub Releases, S3, atau CDN custom).

### 5.6 Auto-Update Engine

- [ ] Implementasi version check:
  - Saat startup, Rust mengecek `GET https://releases.cliptzy.com/engine/version.json`.
  - Bandingkan dengan versi lokal (simpan di `engine_dir/version.txt`).
  - Jika ada update, tampilkan notifikasi di frontend dengan opsi "Update Now".
  - Update = download zip baru + replace engine dir.

---

## Fase 6: Packaging & Rilis (Tauri Build)

### 6.1 Konfigurasi Build Tauri

- [ ] Finalisasi `tauri.conf.json`:
  - Set `bundle.targets` sesuai platform target (nsis, msi, dmg, deb, appimage).
  - Pastikan `bundle.resources` tidak menyertakan `engine/` (karena diunduh saat runtime).
  - Set `bundle.icon` dengan ikon Cliptzy yang sesuai.
  - Set `app.windows[0].title` ke `"Cliptzy — YouTube Clipper & Auto Uploader"`.
  - Tambahkan `app.windows[0].minWidth` dan `minHeight` yang sesuai.

### 6.2 Build Production

- [ ] Test build production:
  ```bash
  # Build frontend
  bun run build

  # Build Tauri (release)
  cargo tauri build
  ```
- [ ] Verifikasi `.exe` / `.dmg` / `.AppImage` bisa dijalankan.
- [ ] Verifikasi bootstrapper flow: jalankan exe tanpa engine → download → extract → engine start → health check → UI ready.

### 6.3 Tauri Updater (Auto-Update App)

- [ ] Konfigurasi `tauri-plugin-updater` untuk auto-update aplikasi Tauri itu sendiri:
  - Tambahkan plugin updater ke `Cargo.toml` dan `tauri.conf.json`.
  - Setup endpoint update (`https://releases.cliptzy.com/app/update.json`).
- [ ] Bedakan update app (Tauri) vs update engine (Python zip) — keduanya independen.

### 6.4 CI/CD Pipeline

- [ ] Setup GitHub Actions workflow:
  - **Build App**: Compile Tauri untuk Windows, macOS, dan Linux.
  - **Build Engine Zip**: Package Portable Python + dependencies untuk setiap OS.
  - **Release**: Upload artifacts ke GitHub Releases atau CDN.
- [ ] Buat workflow file `.github/workflows/build.yml`.
- [ ] Buat workflow file `.github/workflows/build-engine.yml` (di repo engine).

### 6.5 Testing End-to-End

- [ ] Test fresh install scenario:
  1. Install `.exe` di mesin bersih (tanpa Python terinstall).
  2. Jalankan → bootstrapper download engine.
  3. Engine start → health check pass.
  4. Input URL YouTube → analyze → process → output clip.
  5. Tutup aplikasi → verifikasi tidak ada zombie process Python.
- [ ] Test update scenario:
  1. Install versi lama.
  2. Trigger auto-update app.
  3. Trigger auto-update engine.
  4. Verifikasi kedua update berjalan tanpa konflik.
- [ ] Test crash recovery:
  1. Kill proses Python secara manual saat processing.
  2. Verifikasi Rust mendeteksi crash dan menampilkan error di UI.
  3. Verifikasi tombol "Restart Engine" bisa menjalankan ulang Python.

### 6.6 Dokumentasi Rilis

- [ ] Perbarui `README.md` root proyek dengan instruksi:
  - Cara development (git submodule + uv sync + cargo tauri dev).
  - Cara build production (cargo tauri build).
  - Arsitektur sistem (diagram Tauri ↔ Rust ↔ Python).
- [ ] Buat `CONTRIBUTING.md` dengan panduan kontribusi.
- [ ] Buat `CHANGELOG.md` dengan format Keep a Changelog.

---

## 📊 Ringkasan Arsitektur

```
┌─────────────────────────────────────────────────────────┐
│                    Tauri Window (Webview)                │
│  ┌───────────────────────────────────────────────────┐  │
│  │              Vue 3 + Pinia + Tailwind             │  │
│  │  ┌──────────┐ ┌──────────┐ ┌────────────────────┐ │  │
│  │  │Dashboard │ │Compiler  │ │ Settings/Upload    │ │  │
│  │  └──────────┘ └──────────┘ └────────────────────┘ │  │
│  └───────────────────┬───────────────────────────────┘  │
│                      │ invoke()                         │
│  ┌───────────────────▼───────────────────────────────┐  │
│  │           Rust Orchestrator (Tauri Backend)       │  │
│  │  ┌────────────┐ ┌────────┐ ┌──────────────────┐   │  │
│  │  │ Engine Mgr │ │ Health │ │  Bootstrapper    │   │  │
│  │  │ (start/    │ │ Check  │ │  (download/      │   │  │
│  │  │  stop/     │ │ Polling│ │   extract zip)   │   │  │
│  │  │  monitor)  │ │        │ │                  │   │  │
│  │  └─────┬──────┘ └────────┘ └──────────────────┘   │  │
│  └────────┼──────────────────────────────────────────┘  │
└───────────┼─────────────────────────────────────────────┘
            │ HTTP (127.0.0.1:port)
            │ std::process::Command
┌───────────▼─────────────────────────────────────────────┐
│         Python FastAPI Server (Child Process)            │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐   │
│  │ /health  │ │ /clipper │ │/subtitle │ │ /upload  │   │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘   │
│  ┌──────────────────────────────────────────────────┐   │
│  │         core/ (Whisper, yt-dlp, FFmpeg,          │   │
│  │          DeepFace, Torch, Kokoro TTS)            │   │
│  └──────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

---

> **Catatan**: Dokumen ini adalah _living document_. Update checklist saat setiap task selesai.
> Setiap fase harus di-review dan di-test sebelum melanjutkan ke fase berikutnya.
