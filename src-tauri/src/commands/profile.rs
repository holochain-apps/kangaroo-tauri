use tauri::{WindowBuilder, Manager};

use crate::filesystem::{AppFileSystem, Profile};


#[tauri::command]
pub fn get_existing_profiles(
    fs: tauri::State<'_, AppFileSystem>,
) -> Result<Vec<Profile>, String> {
    fs.get_existing_profiles()
}

#[tauri::command]
pub fn get_active_profile(
    fs: tauri::State<'_, AppFileSystem>,
) -> Profile {
    fs.get_active_profile()
}

#[tauri::command]
pub fn set_active_profile(
    fs: tauri::State<'_, AppFileSystem>,
    profile: String
) -> Result<(), String> {
    fs.set_active_profile(&profile)
}

#[tauri::command]
pub fn set_profile_network_seed(
    fs: tauri::State<'_, AppFileSystem>,
    profile: String,
    network_seed: Option<String>,
) -> Result<(), String> {
    fs.set_profile_network_seed(profile, network_seed)
}

#[tauri::command]
pub fn open_profile_settings(
    app_handle: tauri::AppHandle,
) -> tauri::Result<()> {
    if let Some(window) = app_handle.get_window("change_profile") {
        window.show().unwrap();
        window.unminimize().unwrap();
        window.set_focus().unwrap();
    } else {
        let _ = WindowBuilder::new(
            &app_handle,
            "change_profile",
            tauri::WindowUrl::App(std::path::PathBuf::from("kangaroo_assets").join("profiles.html"))
            ).title("Change Profile")
            .inner_size(580.0, 400.0)
            .center()
            .minimizable(false)
            .build();
    }
    Ok(())
}