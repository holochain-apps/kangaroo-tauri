use tauri::{Manager, WindowBuilder};

use crate::app_state::{filesystem::Profile, AppState};

#[tauri::command]
pub fn get_existing_profiles(state: tauri::State<'_, AppState>) -> Result<Vec<Profile>, String> {
    state.fs.get_existing_profiles()
}

#[tauri::command]
pub fn get_active_profile(state: tauri::State<'_, AppState>) -> Profile {
    state.fs.get_active_profile()
}

#[tauri::command]
pub fn set_active_profile(
    state: tauri::State<'_, AppState>,
    profile: String,
) -> Result<(), String> {
    state.fs.set_active_profile(&profile)
}

#[tauri::command]
pub fn set_profile_network_seed(
    state: tauri::State<'_, AppState>,
    profile: String,
    network_seed: Option<String>,
) -> Result<(), String> {
    state.fs.set_profile_network_seed(profile, network_seed)
}

#[tauri::command]
pub fn open_profile_settings(app_handle: tauri::AppHandle) -> tauri::Result<()> {
    if let Some(window) = app_handle.get_window("change_profile") {
        window.show().unwrap();
        window.unminimize().unwrap();
        window.set_focus().unwrap();
    } else {
        let _ = WindowBuilder::new(
            &app_handle,
            "change_profile",
            tauri::WindowUrl::App(
                std::path::PathBuf::from("kangaroo_assets").join("profiles.html"),
            ),
        )
        .title("Change Profile")
        .inner_size(580.0, 400.0)
        .center()
        .minimizable(false)
        .build();
    }
    Ok(())
}
