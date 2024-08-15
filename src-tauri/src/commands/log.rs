use crate::config::APP_NAME;

/// Tauri command to add a log from the UI via tauri's js API
#[tauri::command]
pub fn log(log: String) {
    log::info!("[{} UI] {}", APP_NAME, log);
}
