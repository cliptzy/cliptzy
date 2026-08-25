#[tauri::command]
pub fn get_system_metrics() -> crate::monitor::ProcessMetrics {
    crate::monitor::get_system_metrics()
}
