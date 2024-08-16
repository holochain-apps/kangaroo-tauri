use super::menu::build_main_window;
use crate::app_state::AppState;
use tauri::{
    api::process, AppHandle, CustomMenuItem, Manager, SystemTrayMenu, SystemTrayMenuItem, Wry,
};

pub fn handle_system_tray_event(app: &AppHandle<Wry>, event_id: &str) {
    match event_id {
        "open" => {
            let main_window = app.get_window("main");

            if let Some(window) = main_window {
                window.show().unwrap();
                window.unminimize().unwrap();
                window.set_focus().unwrap();
            } else {
                let state = app.state::<AppState>().inner().to_owned();
                tauri::async_runtime::block_on(async {
                    build_main_window(state.fs.clone(), app, state.app_port, state.admin_port)
                        .await;
                });
            }
        }
        "restart" => {
            process::kill_children();
            app.app_handle().restart();
        }
        "quit" => {
            process::kill_children();
            app.exit(0);
        }
        _ => (),
    }
}

pub fn app_system_tray() -> SystemTrayMenu {
    SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("open".to_string(), "Open"))
        .add_item(CustomMenuItem::new("restart".to_string(), "Restart"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
}
