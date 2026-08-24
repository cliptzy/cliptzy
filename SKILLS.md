# 🛠️ SKILLS.md — Panduan Teknis Implementasi Cliptzy

Dokumen ini berisi **panduan teknis detail** untuk setiap skill/komponen yang harus diimplementasi dalam proyek Cliptzy. Gunakan sebagai referensi saat mengerjakan TODO.md.

> **PENTING**: Dokumen ini saling melengkapi dengan `AGENTS.md` (aturan & larangan) dan `TODO.md` (checklist). Baca ketiganya sebelum mulai.

---

## 📑 DAFTAR ISI

1. [Skill: Rust Path Resolution (paths.rs)](#skill-1-rust-path-resolution)
2. [Skill: Python Engine Manager (engine.rs)](#skill-2-python-engine-manager)
3. [Skill: Health Check System (health.rs)](#skill-3-health-check-system)
4. [Skill: Tauri Commands & Proxy (commands.rs)](#skill-4-tauri-commands--proxy)
5. [Skill: Graceful Shutdown](#skill-5-graceful-shutdown)
6. [Skill: Process Monitor (monitor.rs)](#skill-6-process-monitor)
7. [Skill: Bootstrapper (bootstrapper.rs)](#skill-7-bootstrapper)
8. [Skill: FastAPI Server (server.py)](#skill-8-fastapi-server)
9. [Skill: API Job Manager](#skill-9-api-job-manager)
10. [Skill: Vue Engine Integration](#skill-10-vue-engine-integration)
11. [Skill: Vue Frontend Architecture](#skill-11-vue-frontend-architecture)
12. [Skill: Log Bridge Pipeline](#skill-12-log-bridge-pipeline)
13. [Skill: Engine Zip Packaging](#skill-13-engine-zip-packaging)

---

## Skill 1: Rust Path Resolution

**File**: `src-tauri/src/paths.rs`
**Dependensi Rust**: `dirs = "6"` (belum ditambahkan ke Cargo.toml)
**Status**: ❌ Belum dibuat

### Konteks

Modul ini adalah fondasi yang menentukan di mana Rust mencari Python executable dan engine directory. Bedakan antara development (submodule lokal) dan production (AppData).

### Implementasi

```rust
use std::path::PathBuf;

/// Direktori engine Python.
///
/// - DEV (`cfg!(debug_assertions)`):
///   `<CARGO_MANIFEST_DIR>/engine/`
///   Ini adalah submodule git di `src-tauri/engine/`.
///   CARGO_MANIFEST_DIR = path absolut ke `src-tauri/`.
///
/// - PROD (release build):
///   `<AppData>/com.dickymuliafiqri.cliptzy/engine/`
///   - Windows: `C:\Users\<user>\AppData\Local\com.dickymuliafiqri.cliptzy\engine\`
///   - macOS: `~/Library/Application Support/com.dickymuliafiqri.cliptzy/engine/`
///   - Linux: `~/.local/share/com.dickymuliafiqri.cliptzy/engine/`
pub fn engine_dir() -> PathBuf {
    if cfg!(debug_assertions) {
        let manifest = env!("CARGO_MANIFEST_DIR");
        PathBuf::from(manifest).join("engine")
    } else {
        dirs::data_local_dir()
            .expect("Cannot resolve AppData directory")
            .join("com.dickymuliafiqri.cliptzy")
            .join("engine")
    }
}

/// Path ke Python executable.
///
/// - DEV: Cari di `.venv` yang dibuat oleh `uv sync`.
///   Windows: `engine/.venv/Scripts/python.exe`
///   Unix: `engine/.venv/bin/python`
///   Fallback: `python3` dari PATH sistem.
///
/// - PROD: Portable Python di dalam engine zip.
///   Windows: `engine/python/python.exe`
///   Unix: `engine/python/bin/python3`
pub fn python_executable() -> PathBuf {
    let engine = engine_dir();
    if cfg!(debug_assertions) {
        let venv_py = if cfg!(target_os = "windows") {
            engine.join(".venv").join("Scripts").join("python.exe")
        } else {
            engine.join(".venv").join("bin").join("python")
        };
        if venv_py.exists() { venv_py } else { PathBuf::from("python3") }
    } else {
        if cfg!(target_os = "windows") {
            engine.join("python").join("python.exe")
        } else {
            engine.join("python").join("bin").join("python3")
        }
    }
}
```

### Hal yang PERLU DIPERHATIKAN

- `env!("CARGO_MANIFEST_DIR")` di-resolve saat **compile time** oleh `rustc`. Nilainya adalah path absolut ke folder yang berisi `Cargo.toml`, yaitu `src-tauri/`.
- Di production, `CARGO_MANIFEST_DIR` **tidak tersedia** — itulah kenapa kita pakai `cfg!(debug_assertions)` untuk membedakan.
- `dirs::data_local_dir()` mengembalikan `Option<PathBuf>`. `expect()` aman karena semua OS desktop mendukungnya.
- Identifier `com.dickymuliafiqri.cliptzy` HARUS konsisten dengan `tauri.conf.json` > `identifier`.

### Hal yang JANGAN DILAKUKAN

- ❌ Jangan gunakan `std::env::current_dir()` — tidak reliable di production.
- ❌ Jangan gunakan `std::env::current_exe()` — menunjuk ke binary, bukan project root.
- ❌ Jangan hardcode `/Users/` atau `C:\Users\`.

---

## Skill 2: Python Engine Manager

**File**: `src-tauri/src/engine.rs`
**Dependensi Rust**: `portpicker = "0.1"`, `libc = "0.2"` (belum ditambahkan)
**Status**: ❌ Belum dibuat

### Konteks

`EngineManager` adalah struct singleton (disimpan di Tauri managed state) yang mengelola lifecycle Python child process.

### State Machine

```
                    start()
    ┌──────────┐ ──────────▶ ┌──────────┐
    │  Idle    │             │ Starting │
    └──────────┘             └────┬─────┘
         ▲                       │ health check pass
         │ stop()                ▼
    ┌────┴─────┐             ┌──────────┐
    │ Stopped  │ ◀────────── │ Running  │
    └──────────┘   stop()    └────┬─────┘
         ▲                       │ process exits unexpectedly
         │                       ▼
         │                  ┌──────────┐
         └───────────────── │ Crashed  │
              auto-restart? └──────────┘
```

### Struct Design

```rust
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;

pub struct EngineManager {
    /// Child process handle. None = tidak jalan.
    process: Mutex<Option<Child>>,
    /// Port yang dialokasikan untuk FastAPI server.
    port: Mutex<u16>,
}
```

### Catatan Penting tentang `std::process::Command`

```rust
// BENAR: Gunakan Stdio::piped() agar Rust bisa baca stdout/stderr
let child = Command::new(&python_path)
    .arg(&server_script)
    .arg("--port").arg(port.to_string())
    .arg("--host").arg("127.0.0.1")
    .current_dir(&engine_dir)     // PENTING: set CWD ke engine/
    .stdout(Stdio::piped())       // Agar bisa di-stream ke log
    .stderr(Stdio::piped())       // Tangkap error juga
    .spawn()
    .map_err(|e| format!("Failed to spawn engine: {}", e))?;
```

- `.current_dir()` WAJIB diset ke `engine_dir()` karena Python engine menggunakan relative path untuk `config.json`, `fonts/`, `assets/`, dll.
- `.stdout(Stdio::piped())` memungkinkan Rust membaca output Python untuk log bridging (lihat Skill 12).

### Stop / Kill Strategy

```
1. Kirim SIGTERM (Unix) atau TerminateProcess (Windows)
2. Tunggu max 5 detik (poll try_wait setiap 100ms)
3. Jika masih hidup setelah 5 detik → force kill
4. Log hasil shutdown
```

### Hal yang PERLU DIPERHATIKAN

- `Mutex<Option<Child>>` bukan `RwLock` karena `Child` tidak implement `Send + Sync` secara trivial. Gunakan `Mutex` untuk simplicity.
- `portpicker::pick_unused_port()` mengembalikan `Option<u16>` — handle `None` case.
- Pada Windows, DILARANG menggunakan `libc::kill()`. Gunakan `child.kill()` atau Windows API.

---

## Skill 3: Health Check System

**File**: `src-tauri/src/health.rs`
**Dependensi Rust**: `reqwest = { version = "0.12", features = ["stream"] }`, `tokio` (sudah via Tauri)
**Status**: ❌ Belum dibuat

### Konteks

Setelah `EngineManager::start()` spawn Python, Rust perlu menunggu sampai FastAPI server benar-benar siap menerima request. Ini dilakukan via polling `GET /health`.

### Parameter

| Parameter | Nilai | Alasan |
|-----------|-------|--------|
| Max retries | 60 | Python + FastAPI + model loading bisa lambat (~30 detik) |
| Interval | 500ms | Cukup cepat untuk responsif, tidak terlalu agresif |
| Timeout per request | 2 detik | Jika server belum up, connect timeout cepat gagal |
| Expected response | `200 OK` + JSON `{"status": "ok"}` | Memastikan server benar-benar ready, bukan cuma TCP listening |

### Flow

```
for attempt in 1..=60:
    try:
        GET http://127.0.0.1:{port}/health (timeout 2s)
        if 200 OK → return Ok(())
    catch:
        sleep 500ms
        continue
return Err("Engine failed to start")
```

### Hal yang PERLU DIPERHATIKAN

- Gunakan `tokio::time::timeout()` untuk membungkus `reqwest::Client::get().send()` — agar tidak hang jika server stuck.
- `reqwest::Client` HARUS di-reuse (jangan buat instance baru per request). Simpan di `EngineManager` atau parameter function.
- Health check HARUS async karena kita di konteks Tauri command yang async.

---

## Skill 4: Tauri Commands & Proxy

**File**: `src-tauri/src/commands.rs`
**Status**: ❌ Belum dibuat

### Konteks

Tauri commands adalah bridge antara Vue frontend dan Rust backend. Frontend memanggil `invoke('command_name', args)` dan Rust menjalankan logikanya.

### Daftar Commands yang Diperlukan

| Command | Params | Returns | Deskripsi |
|---------|--------|---------|-----------|
| `start_engine` | — | `Result<u16, String>` (port) | Start Python + health check |
| `stop_engine` | — | `Result<(), String>` | Stop Python gracefully |
| `get_engine_status` | — | `Result<EngineStatus, String>` | Cek apakah running, port, PID |
| `check_engine_installed` | — | `Result<bool, String>` | Cek apakah engine ada di disk |
| `bootstrap_engine` | — | `Result<(), String>` | Download + extract engine zip |
| `proxy_request` | `method`, `path`, `body` | `Result<String, String>` | Forward HTTP ke Python |
| `get_engine_metrics` | — | `Result<ProcessMetrics, String>` | CPU/RAM usage via sysinfo |

### Proxy Pattern

```rust
#[tauri::command]
pub async fn proxy_request(
    engine: State<'_, Arc<EngineManager>>,
    method: String,
    path: String,
    body: Option<String>,
) -> Result<String, String> {
    let port = engine.port(); // ← getter dari EngineManager
    let url = format!("http://127.0.0.1:{}{}", port, path);
    // ... reqwest call ...
}
```

### Kenapa Proxy Pattern?

1. **Port dinamis** — Port dipilih random oleh `portpicker`, frontend tidak tahu portnya.
2. **Keamanan** — Frontend webview tidak boleh langsung akses localhost arbitrary port.
3. **Lifecycle coupling** — Rust tahu kapan engine mati, bisa return error yang proper.
4. **Single point of control** — Semua request melewati Rust, bisa di-log/intercept.

### Hal yang JANGAN DILAKUKAN

- ❌ Jangan expose port ke frontend via Tauri event lalu biarkan frontend `fetch()` langsung.
- ❌ Jangan buat reqwest Client baru di setiap invocation — simpan di state atau lazy_static.

---

## Skill 5: Graceful Shutdown

**Status**: ❌ Belum diimplementasi

### Konteks

Ini adalah salah satu requirement PALING KRITIS. Jika Python process tidak di-kill saat window ditutup, akan ada zombie process yang memakan RAM (model Whisper, Torch, dll. bisa 2-4 GB).

### Implementasi Multi-Layer

```
Layer 1: on_window_event(Destroyed) → engine.stop()
    ↓ (jika Layer 1 gagal — misal panic)
Layer 2: ctrlc handler → engine.stop() + process::exit(0)
    ↓ (jika Layer 2 juga gagal)
Layer 3: OS-level → proses mati karena parent mati
    (ini TIDAK bisa diandalkan di semua OS)
```

### Implementasi di lib.rs

```rust
// Layer 1: Window event
.on_window_event(move |_window, event| {
    if let tauri::WindowEvent::Destroyed = event {
        tracing::info!("Window destroyed, killing engine...");
        engine_for_shutdown.stop();
    }
})

// Layer 2: Ctrl+C handler (di .setup())
.setup(|app| {
    let engine = app.state::<Arc<EngineManager>>().inner().clone();
    ctrlc::set_handler(move || {
        tracing::info!("Ctrl+C received, killing engine...");
        engine.stop();
        std::process::exit(0);
    }).ok();
    Ok(())
})
```

### Hal yang PERLU DIPERHATIKAN

- `on_window_event` closure membutuhkan `move`. `engine_for_shutdown` harus di-`clone()` dari `Arc` sebelum masuk closure.
- `ctrlc::set_handler` hanya bisa dipanggil **sekali**. Panggil kedua kali akan error (tapi `.ok()` menelan error).
- Di macOS, menutup window TIDAK selalu keluar dari app (app masih hidup di dock). Pastikan kita handle ini.

---

## Skill 6: Process Monitor

**File**: `src-tauri/src/monitor.rs`
**Dependensi Rust**: `sysinfo = "0.35"` (belum ditambahkan)
**Status**: ❌ Belum dibuat

### Konteks

Memantau resource usage dari Python child process. Berguna untuk menampilkan di status bar dan mendeteksi crash.

### Struct

```rust
#[derive(serde::Serialize, Clone)]
pub struct ProcessMetrics {
    pub cpu_usage: f32,       // Persentase CPU
    pub memory_mb: u64,       // Memory usage dalam MB
    pub status: String,       // "Running", "Sleeping", "Zombie", dll.
    pub pid: u32,             // Process ID
}
```

### Hal yang PERLU DIPERHATIKAN

- `sysinfo::System::new()` lalu `refresh_processes()` untuk mendapatkan info proses.
- API `sysinfo` v0.35 berubah signifikan dari v0.30. Gunakan `ProcessesToUpdate::Some(&[pid])` untuk efisiensi.
- `cpu_usage()` mengembalikan 0.0 pada panggilan pertama — perlu dua kali `refresh` dengan jeda untuk mendapat nilai akurat.

---

## Skill 7: Bootstrapper

**File**: `src-tauri/src/bootstrapper.rs`
**Dependensi Rust**: `reqwest` (stream), `zip = "2"`, `futures-util = "0.3"`, `sha2 = "0.10"` (belum ditambahkan)
**Status**: ❌ Belum dibuat

### Konteks

Saat production, engine Python tidak di-bundle dalam `.exe`. Pada first-run, Rust mendeteksi engine belum ada dan mengunduh `.zip` berisi Portable Python + semua library + script dari server.

### Deteksi Engine Installed

```rust
pub fn is_engine_installed(&self) -> bool {
    let server_py = self.engine_dir.join("server.py");
    let python = crate::paths::python_executable();
    // Kedua file HARUS ada
    server_py.exists() && python.exists()
}
```

### Download Flow

```
1. Download zip dari URL (stream chunked)
   - Emit Tauri event `bootstrap-progress` setiap chunk
   - Progress = bytes_downloaded / total_bytes
2. Verify SHA-256 checksum
   - Download checksum file dari URL
   - Compute SHA-256 dari downloaded zip
   - Compare — jika tidak cocok, delete zip + error
3. Extract zip ke engine_dir
   - Emit progress setiap 100 files
4. Set executable permissions (Unix only)
   - chmod +x python binary
   - chmod +x ffmpeg binary (jika ada)
5. Cleanup zip file
6. Emit progress stage="done"
```

### Tauri Event Schema

```json
{
  "stage": "downloading",       // "downloading" | "extracting" | "verifying" | "done" | "error"
  "progress": 0.45,             // 0.0 - 1.0
  "message": "Downloading AI Engine... 234.5 MB / 520.0 MB",
  "bytes_downloaded": 245890048,
  "total_bytes": 545259520
}
```

### Hal yang PERLU DIPERHATIKAN

- Engine zip bisa SANGAT BESAR (500MB - 2GB) karena berisi PyTorch, Whisper models, dll. Download HARUS chunked dan resumable.
- `zip` crate v2 API: `ZipArchive::new(file)` → `by_index(i)` → `read_to_end()` atau `std::io::copy()`.
- Emit progress **tidak** setiap chunk (terlalu sering) — emit setiap ~1MB atau setiap 100 files saat extract.
- `build.rs` harus set `ENGINE_DOWNLOAD_URL` dan `ENGINE_CHECKSUM_URL` sebagai compile-time env vars.

---

## Skill 8: FastAPI Server

**File**: `src-tauri/engine/server.py`
**Dependensi Python**: `fastapi`, `uvicorn[standard]` (belum ditambahkan via `uv add`)
**Status**: ❌ Belum dibuat

### Konteks

Entry point tunggal untuk Python engine. Menggantikan `main.py` (legacy Flet+CLI).

### Structure

```python
# server.py
import uvicorn
from fastapi import FastAPI
from contextlib import asynccontextmanager

@asynccontextmanager
async def lifespan(app: FastAPI):
    # Startup
    from core.logger import log
    log.info("Cliptzy AI Engine starting...")
    yield
    # Shutdown
    log.info("Cliptzy AI Engine shutting down...")

app = FastAPI(title="Cliptzy AI Engine", version="4.0.0", lifespan=lifespan)

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

### Hal yang PERLU DIPERHATIKAN

- Import `core.*` di dalam `lifespan()` context, bukan di top-level, karena beberapa modul core melakukan heavy initialization saat import (mis. `core.bootstrap` yang memodifikasi `subprocess.Popen`).
- `uvicorn.run()` blocks — ini intended karena Rust yang mengelola lifecycle.
- DILARANG menggunakan `uvicorn.run(app, reload=True)` di production — reload menyebabkan re-import yang bisa konflik.
- Router prefix convention: `/health`, `/clipper`, `/subtitle`, `/upload`.

---

## Skill 9: API Job Manager

**File**: `src-tauri/engine/api/job_manager.py`
**Status**: ❌ Belum dibuat

### Konteks

Video processing (download, transcode, subtitle, upload) adalah operasi berat yang memakan waktu lama (30 detik - 10 menit). API tidak boleh blocking. Gunakan Job Manager.

### Pattern

```python
import asyncio
import uuid
from dataclasses import dataclass, field
from enum import Enum
from typing import Any

class JobStatus(str, Enum):
    QUEUED = "queued"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"
    CANCELLED = "cancelled"

@dataclass
class Job:
    job_id: str = field(default_factory=lambda: str(uuid.uuid4()))
    status: JobStatus = JobStatus.QUEUED
    progress: float = 0.0       # 0.0 - 100.0
    message: str = ""
    result: Any = None
    error: str | None = None
    _cancel_flag: bool = False

    @property
    def is_cancelled(self) -> bool:
        return self._cancel_flag

    def cancel(self):
        self._cancel_flag = True

class JobManager:
    def __init__(self):
        self._jobs: dict[str, Job] = {}

    def create_job(self) -> Job:
        job = Job()
        self._jobs[job.job_id] = job
        return job

    def get_job(self, job_id: str) -> Job | None:
        return self._jobs.get(job_id)

    def cancel_job(self, job_id: str) -> bool:
        job = self._jobs.get(job_id)
        if job and job.status == JobStatus.RUNNING:
            job.cancel()
            return True
        return False

# Global singleton
job_manager = JobManager()
```

### API Endpoint Pattern

```python
@router.post("/clipper/process", status_code=202)
async def process_clip(request: ClipRequest):
    job = job_manager.create_job()
    # Offload ke background — JANGAN await di sini!
    asyncio.create_task(_process_clip_background(job, request))
    return {"job_id": job.job_id, "status": job.status}

@router.get("/clipper/progress/{job_id}")
async def get_progress(job_id: str):
    job = job_manager.get_job(job_id)
    if not job:
        raise HTTPException(status_code=404, detail="Job not found")
    return {"job_id": job.job_id, "status": job.status,
            "progress": job.progress, "message": job.message,
            "result": job.result, "error": job.error}
```

### Hal yang PERLU DIPERHATIKAN

- Gunakan `asyncio.create_task()` untuk fire-and-forget background work.
- Di dalam background task, gunakan `await asyncio.to_thread(blocking_function)` untuk CPU-bound work (FFmpeg, Whisper).
- `Job._cancel_flag` dicheck oleh worker loops. Contoh: `if job.is_cancelled: break`.
- In-memory store hilang saat server restart — ini acceptable karena server hanya hidup selama Tauri app terbuka.

---

## Skill 10: Vue Engine Integration

**File**: `src/composables/useEngine.ts`, `src/stores/engine.ts`
**Dependensi**: `pinia` (belum diinstal)
**Status**: ❌ Belum dibuat

### Composable: useEngine.ts

```typescript
import { invoke } from '@tauri-apps/api/core';

// Type definitions
interface EngineStatus {
  status: 'idle' | 'starting' | 'running' | 'error' | 'crashed';
  port: number | null;
  pid: number | null;
}

interface ProxyResponse<T = any> {
  data: T;
}

export function useEngine() {
  /**
   * Start Python engine. Returns port number.
   * Rust akan: spawn Python → health check → return port.
   */
  const startEngine = async (): Promise<number> => {
    return await invoke<number>('start_engine');
  };

  /**
   * Stop Python engine gracefully.
   */
  const stopEngine = async (): Promise<void> => {
    await invoke<void>('stop_engine');
  };

  /**
   * Proxy HTTP request melalui Rust ke Python engine.
   * Frontend TIDAK BOLEH langsung fetch ke localhost.
   */
  const proxyGet = async <T = any>(path: string): Promise<T> => {
    const raw = await invoke<string>('proxy_request', {
      method: 'GET', path, body: null,
    });
    return JSON.parse(raw) as T;
  };

  const proxyPost = async <T = any>(path: string, body?: object): Promise<T> => {
    const raw = await invoke<string>('proxy_request', {
      method: 'POST', path,
      body: body ? JSON.stringify(body) : null,
    });
    return JSON.parse(raw) as T;
  };

  return { startEngine, stopEngine, proxyGet, proxyPost };
}
```

### Pinia Store: engine.ts

```typescript
import { defineStore } from 'pinia';
import { ref } from 'vue';
import { useEngine } from '@/composables/useEngine';
import { listen } from '@tauri-apps/api/event';

type EngineState = 'idle' | 'checking' | 'downloading' | 'starting' | 'ready' | 'error';

export const useEngineStore = defineStore('engine', () => {
  const status = ref<EngineState>('idle');
  const port = ref<number | null>(null);
  const errorMessage = ref<string | null>(null);
  const bootstrapProgress = ref<number>(0);

  const { startEngine, proxyGet } = useEngine();

  async function initialize() {
    try {
      // Step 1: Check if engine installed
      status.value = 'checking';
      const installed = await invoke<boolean>('check_engine_installed');

      if (!installed) {
        // Step 2: Bootstrap (download + extract)
        status.value = 'downloading';
        await invoke('bootstrap_engine');
      }

      // Step 3: Start engine
      status.value = 'starting';
      port.value = await startEngine();
      status.value = 'ready';
    } catch (e) {
      status.value = 'error';
      errorMessage.value = String(e);
    }
  }

  return { status, port, errorMessage, bootstrapProgress, initialize };
});
```

### Hal yang PERLU DIPERHATIKAN

- `invoke()` adalah async dan melempar error sebagai string (karena Rust return `Result<T, String>`).
- `listen()` digunakan untuk real-time events (bootstrap progress, engine logs). Jangan polling dari frontend.
- `@/composables/` path alias perlu dikonfigurasi di `tsconfig.json` dan `vite.config.ts`.

---

## Skill 11: Vue Frontend Architecture

**Status**: ❌ Belum dibuat (semua belum diimplementasi)

### Framework Dependencies (Belum Diinstal)

```bash
bun add pinia vue-router @vueuse/core
bun add -d tailwindcss @tailwindcss/vite
```

### Routing

```typescript
// src/router/index.ts
import { createRouter, createWebHistory } from 'vue-router';

const routes = [
  { path: '/', name: 'dashboard', component: () => import('@/views/DashboardView.vue') },
  { path: '/compilation', name: 'compilation', component: () => import('@/views/CompilationView.vue') },
  { path: '/upload', name: 'upload', component: () => import('@/views/UploadView.vue') },
  { path: '/settings', name: 'settings', component: () => import('@/views/SettingsView.vue') },
];

export const router = createRouter({
  history: createWebHistory(),
  routes,
});
```

### App.vue Startup Flow

```
1. Mount App.vue
2. Call engineStore.initialize()
3. While status != 'ready':
   - Show splash/loading screen
   - If status == 'downloading': show BootstrapProgress
   - If status == 'starting': show "Starting AI Engine..."
   - If status == 'error': show error + retry button
4. When status == 'ready':
   - Show main layout (sidebar + router-view)
```

### Hal yang PERLU DIPERHATIKAN

- Tauri v2 menggunakan `createWebHistory()` (bukan `createWebHashHistory()`) — Tauri webview mendukung HTML5 history mode.
- Lazy loading routes (`() => import(...)`) untuk code splitting.
- `@` path alias: Tambahkan di `vite.config.ts`:
  ```typescript
  resolve: {
    alias: { '@': path.resolve(__dirname, 'src') }
  }
  ```
  Dan di `tsconfig.json`:
  ```json
  "paths": { "@/*": ["src/*"] }
  ```

---

## Skill 12: Log Bridge Pipeline

**Status**: ❌ Belum diimplementasi

### Konteks

Pipeline: Python stdout → Rust thread → Tauri event → Vue LogViewer

### Layer 1: Python → stdout

Python engine harus log ke stdout (selain file):
```python
# core/logger.py — pastikan ada StreamHandler ke sys.stdout
import logging
log = logging.getLogger("cliptzy")
log.addHandler(logging.StreamHandler(sys.stdout))
```

### Layer 2: Rust reads stdout

```rust
// Di engine.rs, setelah spawn:
let stdout = child.stdout.take().expect("Failed to capture stdout");
let app_handle_clone = app_handle.clone();

std::thread::spawn(move || {
    use std::io::BufRead;
    let reader = std::io::BufReader::new(stdout);
    for line in reader.lines().flatten() {
        tracing::debug!(target: "engine", "{}", line);
        // Emit ke frontend
        let _ = app_handle_clone.emit("engine-log", &line);
    }
});
```

### Layer 3: Vue listens

```typescript
import { listen } from '@tauri-apps/api/event';

const logs = ref<string[]>([]);

onMounted(async () => {
  await listen<string>('engine-log', (event) => {
    logs.value.push(event.payload);
    // Auto-scroll, max 1000 lines, etc.
    if (logs.value.length > 1000) logs.value.shift();
  });
});
```

### Hal yang PERLU DIPERHATIKAN

- `child.stdout.take()` — take() memindahkan ownership, hanya bisa dipanggil **sekali**. Panggil segera setelah spawn.
- Thread pembaca stdout **akan block** sampai child process mati (stdout EOF). Ini expected behavior.
- Emit Tauri event per-line bisa flood jika Python log sangat verbose. Pertimbangkan batching (kumpulkan 10 lines, emit sekali).
- stderr juga perlu di-capture dan di-pipe (spawn thread terpisah).

---

## Skill 13: Engine Zip Packaging

**File**: `src-tauri/engine/scripts/build_engine_zip.py`
**Status**: ❌ Belum dibuat

### Konteks

Script build-time (bukan runtime!) yang membuat distributable zip berisi:

```
engine.zip/
├── python/               # Portable Python (python-build-standalone)
│   ├── bin/python3       # (Unix) atau python.exe (Windows)
│   └── lib/              # stdlib + site-packages (semua deps pre-installed)
├── server.py             # FastAPI entry point
├── api/                  # API routers
├── core/                 # Engine core modules
├── fonts/                # Subtitle fonts
├── assets/               # Static assets
├── config.json           # Default configuration
└── version.txt           # "4.0.0" — untuk update check
```

### Sumber Portable Python

Gunakan [python-build-standalone](https://github.com/indygreg/python-build-standalone):
- Download release yang sesuai (contoh: `cpython-3.13.x+<date>-x86_64-unknown-linux-gnu-install_only_stripped.tar.gz`)
- Extract → `python/`
- Install semua deps: `python/bin/python3 -m pip install -r requirements.txt`
- Strip unnecessary files (tests, docs, `.pyc` cache) untuk reduce size

### Platform-Specific Builds

| Platform | Python Build | FFmpeg | Ukuran Estimasi |
|----------|-------------|--------|-----------------|
| Windows x64 | `cpython-3.13-x86_64-pc-windows-msvc-shared-install_only.tar.gz` | `ffmpeg.exe` from gyan.dev | ~1.5 GB |
| macOS x64 | `cpython-3.13-x86_64-apple-darwin-install_only.tar.gz` | `ffmpeg` from evermeet.cx | ~1.5 GB |
| macOS arm64 | `cpython-3.13-aarch64-apple-darwin-install_only.tar.gz` | — | ~1.5 GB |
| Linux x64 | `cpython-3.13-x86_64-unknown-linux-gnu-install_only_stripped.tar.gz` | `ffmpeg` from static build | ~1.5 GB |

### Hal yang PERLU DIPERHATIKAN

- ZIP bisa SANGAT BESAR (1-2 GB) karena PyTorch alone ~800MB. Pertimbangkan split download (base + models).
- Generate SHA-256 checksum: `sha256sum engine.zip > engine.zip.sha256`
- Upload ke GitHub Releases atau CDN yang mendukung large file downloads.
- Script ini dijalankan di CI/CD, bukan oleh user.

---

## 📋 CHECKLIST SEBELUM IMPLEMENTASI SKILL

Sebelum mengimplementasi skill manapun, verifikasi:

1. [ ] Skill ini termasuk dalam fase mana di `TODO.md`?
2. [ ] Semua dependensi yang dibutuhkan sudah ditambahkan? (Cargo.toml / package.json / pyproject.toml)
3. [ ] Skill sebelumnya yang menjadi prerequisite sudah selesai?
4. [ ] Saya sudah membaca `AGENTS.md` untuk aturan layer yang relevan?
5. [ ] File yang akan saya buat/edit **benar-benar belum ada**? (Cek `AGENTS.md` Bagian 0.2)

### Dependency Graph

```
Skill 1 (paths.rs)
    └── Skill 2 (engine.rs) ← depends on paths.rs
        ├── Skill 3 (health.rs) ← depends on engine.rs (port)
        ├── Skill 5 (shutdown) ← depends on engine.rs (stop)
        ├── Skill 6 (monitor.rs) ← depends on engine.rs (PID)
        └── Skill 12 (log bridge) ← depends on engine.rs (stdout)
    └── Skill 7 (bootstrapper) ← depends on paths.rs
    └── Skill 4 (commands.rs) ← depends on engine, health, bootstrapper

Skill 8 (server.py) ← independent dari Rust
    └── Skill 9 (job manager) ← depends on server.py

Skill 10 (Vue engine) ← depends on Skill 4 (commands exist)
    └── Skill 11 (Vue architecture) ← depends on Skill 10
```

---

_Dokumen ini terakhir diperbarui: 2026-08-25. Sinkronkan dengan perubahan arsitektur._
