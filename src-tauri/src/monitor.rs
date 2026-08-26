use sysinfo::{System, Pid, Networks};
use std::sync::Mutex;
use std::time::Instant;
use once_cell::sync::Lazy;

#[derive(serde::Serialize, Clone)]
pub struct ProcessMetrics {
    pub cpu_usage: f32,       // Persentase CPU
    pub memory_mb: u64,       // Memory usage dalam MB
    pub system_memory_mb: u64, // Total memory
    pub system_used_memory_mb: u64,
    pub network_rx_kbps: f32, // Download speed (KB/s)
    pub network_tx_kbps: f32, // Upload speed (KB/s)
    pub has_gpu: bool,
    pub gpu_usage: Option<f32>,
}

static SYSTEM: Lazy<Mutex<System>> = Lazy::new(|| Mutex::new(System::new_all()));
static NETWORKS: Lazy<Mutex<Networks>> = Lazy::new(|| Mutex::new(Networks::new_with_refreshed_list()));
static LAST_REFRESH: Lazy<Mutex<Instant>> = Lazy::new(|| Mutex::new(Instant::now()));

pub fn get_system_metrics() -> ProcessMetrics {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_all();
    
    let mut net = NETWORKS.lock().unwrap();
    net.refresh(true);
    
    let mut last_refresh = LAST_REFRESH.lock().unwrap();
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

    ProcessMetrics {
        cpu_usage: cpu,
        memory_mb: mem,
        system_memory_mb: sys.total_memory() / 1_048_576,
        system_used_memory_mb: sys.used_memory() / 1_048_576,
        network_rx_kbps: rx_kbps,
        network_tx_kbps: tx_kbps,
        has_gpu: false, // sysinfo tidak punya API bawaan untuk GPU usage secara umum
        gpu_usage: None,
    }
}
