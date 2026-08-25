use sysinfo::{System, Pid};
use std::sync::Mutex;
use once_cell::sync::Lazy;

#[derive(serde::Serialize, Clone)]
pub struct ProcessMetrics {
    pub cpu_usage: f32,       // Persentase CPU
    pub memory_mb: u64,       // Memory usage dalam MB
    pub system_memory_mb: u64, // Total memory
    pub system_used_memory_mb: u64,
}

static SYSTEM: Lazy<Mutex<System>> = Lazy::new(|| Mutex::new(System::new_all()));

pub fn get_system_metrics() -> ProcessMetrics {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_all();
    
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
    }
}
