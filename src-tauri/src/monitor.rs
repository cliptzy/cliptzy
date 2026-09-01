use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::time::Instant;
use sysinfo::{Networks, Pid, System};

#[derive(serde::Serialize, Clone)]
pub struct ProcessMetrics {
    pub cpu_usage: f32,        // Persentase CPU
    pub memory_mb: u64,        // Memory usage dalam MB
    pub system_memory_mb: u64, // Total memory
    pub system_used_memory_mb: u64,
    pub network_rx_kbps: f32, // Download speed (KB/s)
    pub network_tx_kbps: f32, // Upload speed (KB/s)
    pub has_gpu: bool,
    pub gpu_usage: Option<f32>,
}

static SYSTEM: Lazy<Mutex<System>> = Lazy::new(|| Mutex::new(System::new_all()));
static NETWORKS: Lazy<Mutex<Networks>> =
    Lazy::new(|| Mutex::new(Networks::new_with_refreshed_list()));
static LAST_REFRESH: Lazy<Mutex<Instant>> = Lazy::new(|| Mutex::new(Instant::now()));

static GPU_USAGE: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static INIT_GPU_MONITOR: std::sync::Once = std::sync::Once::new();

pub fn start_gpu_monitor() {
    INIT_GPU_MONITOR.call_once(|| {
        std::thread::spawn(|| {
            #[cfg(target_os = "windows")]
            {
                use std::process::{Command, Stdio};
                use std::io::{BufRead, BufReader};
                
                let mut child = match Command::new("typeperf")
                    .arg("\\GPU Engine(*)\\Utilization Percentage")
                    .stdout(Stdio::piped())
                    .stderr(Stdio::null())
                    .spawn() {
                        Ok(c) => c,
                        Err(_) => return,
                    };

                if let Some(stdout) = child.stdout.take() {
                    let reader = BufReader::new(stdout);
                    for line in reader.lines() {
                        if let Ok(line) = line {
                            if !line.starts_with("\"") || line.contains("PDH_") || line.contains("Time") { 
                                continue; 
                            }
                            let parts: Vec<&str> = line.split(',').collect();
                            let mut max_val: f32 = 0.0;
                            for part in parts.iter().skip(1) {
                                let clean = part.trim_matches('"');
                                if let Ok(v) = clean.parse::<f32>() {
                                    if v > max_val { max_val = v; }
                                }
                            }
                            GPU_USAGE.store(max_val.to_bits(), std::sync::atomic::Ordering::Relaxed);
                        }
                    }
                }
            }
            
            #[cfg(not(target_os = "windows"))]
            {
                // Fallback for nvidia-smi on Linux/Mac
                use std::process::Command;
                loop {
                    if let Ok(output) = Command::new("nvidia-smi")
                        .args(&["--query-gpu=utilization.gpu", "--format=csv,noheader,nounits"])
                        .output() 
                    {
                        let text = String::from_utf8_lossy(&output.stdout);
                        if let Some(val) = text.trim().lines().next() {
                            if let Ok(v) = val.parse::<f32>() {
                                GPU_USAGE.store(v.to_bits(), std::sync::atomic::Ordering::Relaxed);
                            }
                        }
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            }
        });
    });
}

pub fn get_system_metrics() -> ProcessMetrics {
    let mut sys = SYSTEM.lock().unwrap_or_else(|e| e.into_inner());
    sys.refresh_cpu_all();
    sys.refresh_memory();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    let mut net = NETWORKS.lock().unwrap_or_else(|e| e.into_inner());
    net.refresh(true);

    let mut last_refresh = LAST_REFRESH.lock().unwrap_or_else(|e| e.into_inner());
    let now = Instant::now();
    let elapsed = now.duration_since(*last_refresh).as_secs_f32();
    *last_refresh = now;

    let mut rx_bytes = 0;
    let mut tx_bytes = 0;
    for (_interface_name, data) in net.iter() {
        rx_bytes += data.received();
        tx_bytes += data.transmitted();
    }

    // Hitung kecepatan dalam KB/s. Jika elapsed = 0, fallback ke 0
    let elapsed = if elapsed > 0.0 { elapsed } else { 1.0 };
    let rx_kbps = (rx_bytes as f32 / 1024.0) / elapsed;
    let tx_kbps = (tx_bytes as f32 / 1024.0) / elapsed;

    let pid = Pid::from_u32(std::process::id());

    let (cpu, mem) = if let Some(p) = sys.process(pid) {
        (p.cpu_usage(), p.memory() / 1_048_576)
    } else {
        (0.0, 0)
    };

    start_gpu_monitor();
    let gpu_val = f32::from_bits(GPU_USAGE.load(std::sync::atomic::Ordering::Relaxed));
    let has_gpu = gpu_val > 0.0 || !crate::system::get_system_gpus().is_empty();

    ProcessMetrics {
        cpu_usage: cpu,
        memory_mb: mem,
        system_memory_mb: sys.total_memory() / 1_048_576,
        system_used_memory_mb: sys.used_memory() / 1_048_576,
        network_rx_kbps: rx_kbps,
        network_tx_kbps: tx_kbps,
        has_gpu,
        gpu_usage: Some(gpu_val),
    }
}

#[derive(serde::Serialize, Clone)]
pub struct SystemSpecsCheck {
    pub meets_requirements: bool,
    pub current_memory_gb: f64,
    pub required_memory_gb: f64,
    pub current_cpu_cores: usize,
    pub required_cpu_cores: usize,
    pub missing_reasons: Vec<String>,
}

pub fn check_system_specs() -> SystemSpecsCheck {
    let mut sys = SYSTEM.lock().unwrap_or_else(|e| e.into_inner());
    sys.refresh_cpu_all();
    sys.refresh_memory();

    let total_memory_gb = sys.total_memory() as f64 / 1_073_741_824.0;
    let required_memory_gb = 7.0;

    let cpu_cores = sys.cpus().len();
    let required_cpu_cores = 4;

    let mut missing_reasons = Vec::new();

    if total_memory_gb < required_memory_gb {
        missing_reasons.push(format!(
            "RAM minimal {:.1}GB dibutuhkan (Terdeteksi: {:.1}GB)",
            required_memory_gb, total_memory_gb
        ));
    }

    if cpu_cores < required_cpu_cores {
        missing_reasons.push(format!(
            "CPU minimal 4 cores dibutuhkan (Terdeteksi: {} core)",
            cpu_cores
        ));
    }

    SystemSpecsCheck {
        meets_requirements: missing_reasons.is_empty(),
        current_memory_gb: total_memory_gb,
        required_memory_gb: required_memory_gb,
        current_cpu_cores: cpu_cores,
        required_cpu_cores,
        missing_reasons,
    }
}
