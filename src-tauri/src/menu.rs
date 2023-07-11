use tauri::{CustomMenuItem, Manager, Menu, Submenu, Window, Wry};

pub fn build_menu() -> Menu {
  let restart = CustomMenuItem::new("restart".to_string(), "Restart");
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let devtools = CustomMenuItem::new("devtools".to_string(), "Open DevTools");

  let menu_submenu = Submenu::new(
    "Menu",
    Menu::new()
      // .add_item(version_info.clone())
      // .add_item(open_logs.clone())
      .add_item(restart.clone())
      .add_item(quit.clone())
      .add_item(devtools.clone()),
  );



  // special menu for macOS
  if cfg!(target_os = "macos") {
    let launcher_menu_submenu = Submenu::new(
      "rename-me", // This is the menu title on macOS
      Menu::new()
        .add_item(restart)
        .add_item(quit)
        .add_item(devtools),
    );

    return Menu::os_default("Holochain Launcher")
      .add_submenu(launcher_menu_submenu)
  }

  Menu::new()
    .add_submenu(menu_submenu)
    // .add_submenu(settings_submenu)
}

pub fn handle_menu_event(event_id: &str, window: &Window<Wry>) {
  let app_handle = window.app_handle();
  // let profile = app_handle.state::<Profile>();
  match event_id {
    "restart" => app_handle.restart(),
    "quit" => app_handle.exit(0),
    "devtools" => window.open_devtools(),
    _ => {}
  }
}
