use tauri::{
  window::WindowBuilder, AppHandle, CustomMenuItem, Manager, SystemTrayMenu,
  WindowUrl, Wry, SystemTrayMenuItem,
};

use crate::WINDOW_TITLE;

pub fn handle_system_tray_event(app: &AppHandle<Wry>, event_id: String) {
  match event_id.as_str() {
    "open" => {
      let main_window = app.get_window("main");

      if let Some(window) = main_window {
        window.show().unwrap();
        window.unminimize().unwrap();
        window.set_focus().unwrap();
      } else {
        let r = WindowBuilder::new(app, "main", WindowUrl::App("index.html".into()))
          .inner_size(1400.0, 880.0)
          .title(WINDOW_TITLE)
          .build();

        log::info!("Creating main window {:?}", r);
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
