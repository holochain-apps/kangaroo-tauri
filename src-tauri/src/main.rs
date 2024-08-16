// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::{
    setup_app,
    system_tray::{app_system_tray, handle_system_tray_event},
    window::handle_menu_event,
};
use commands::{
    log::log,
    profile::{
        get_active_profile, get_existing_profiles, open_profile_settings, set_active_profile,
        set_profile_network_seed,
    },
    restart::restart,
    sign_zome_call::sign_zome_call,
};
use tauri::{RunEvent, SystemTray, SystemTrayEvent};

mod app;
mod app_state;
mod commands;
mod config;
mod errors;
mod launch;
mod logs;
mod process;
mod utils;

fn main() {
    let builder_result = tauri::Builder::default()
        .on_menu_event(|event| handle_menu_event(event.menu_item_id(), event.window()))
        // optional (systray) -- Adds your app with an icon to the OS system tray.
        .system_tray(SystemTray::new().with_menu(app_system_tray()))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => handle_system_tray_event(app, &id),
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            sign_zome_call,
            log,
            set_active_profile,
            get_active_profile,
            get_existing_profiles,
            set_profile_network_seed,
            open_profile_settings,
            restart,
        ])
        .setup(setup_app)
        .build(tauri::generate_context!());

    match builder_result {
        Ok(builder) => {
            builder.run(|_app_handle, event| {
                // This event is emitted upon quitting the App via cmq+Q on macOS.
                // Sidecar binaries need to get explicitly killed in this case (https://github.com/holochain/launcher/issues/141)
                if let RunEvent::Exit = event {
                    tauri::api::process::kill_children();
                }

                // optional (systray):
                // This event is emitted upon pressing the x to close the App window
                // The app is prevented from exiting to keep it running in the background with the system tray
                // Remove those lines below with () if you don't want the systray functionality
                if let RunEvent::ExitRequested { api, .. } = event {
                    api.prevent_exit();
                }
            });
        }
        Err(err) => log::error!("Error building the app: {:?}", err),
    }
}
