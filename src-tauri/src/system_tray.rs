use tauri::{AppHandle, CustomMenuItem, Manager, SystemTrayMenu, Wry, SystemTrayMenuItem};
use crate::{filesystem::AppFileSystem, build_main_window};

pub fn handle_system_tray_event(app: &AppHandle<Wry>, event_id: String) {
  match event_id.as_str() {
    "open" => {
      let main_window = app.get_window("main");

      if let Some(window) = main_window {
        window.show().unwrap();
        window.unminimize().unwrap();
        window.set_focus().unwrap();
      } else {
        let fs = app.state::<AppFileSystem>().inner().to_owned();
        let (app_port, admin_port) = app.state::<(u16, u16)>().inner().to_owned();
        let _r = build_main_window(fs, app, app_port, admin_port);
      }
    }
    "restart" => app.app_handle().restart(),
    "quit" => app.exit(0),
    _ => (),
  }
}

pub fn app_system_tray() -> SystemTrayMenu {

  let mut menu = SystemTrayMenu::new();

  menu = menu.add_item(CustomMenuItem::new("open".to_string(), "Open"));
  menu = menu.add_item(CustomMenuItem::new("restart".to_string(), "Restart"));
  menu = menu.add_native_item(SystemTrayMenuItem::Separator);
  menu = menu.add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

  menu
}
