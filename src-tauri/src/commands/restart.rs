
// restarts the Holochain Launcher
#[tauri::command]
pub fn restart(
  app_handle: tauri::AppHandle,
) -> Result<(), String> {
  log::warn!("A Restart of the app has been requested. Restarting...");
  app_handle.restart();
  Ok(())
}
