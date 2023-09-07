use tauri::{CustomMenuItem, Manager, Menu, Submenu, Window, Wry};
use tauri::api::dialog::message;

use crate::commands::profile::open_profile_settings;
use crate::{logs::open_logs_folder, filesystem::AppFileSystem, APP_NAME};

pub fn build_menu() -> Menu {

	let version = CustomMenuItem::new("version".to_string(), "Version");
	let change_profile = CustomMenuItem::new("change_profile".to_string(), "Change Profile");
	let open_logs = CustomMenuItem::new("open_logs".to_string(), "Open Logs");
	let devtools = CustomMenuItem::new("devtools".to_string(), "Open DevTools");
	let restart = CustomMenuItem::new("restart".to_string(), "Restart");
	let quit = CustomMenuItem::new("quit".to_string(), "Quit");

	let menu_submenu = Submenu::new(
	"Menu",
	Menu::new()
		.add_item(version.clone())
		.add_item(change_profile.clone())
		.add_item(open_logs.clone())
		.add_item(devtools.clone())
		.add_item(restart.clone())
		.add_item(quit.clone())
	);



	// special menu for macOS
	if cfg!(target_os = "macos") {
	let app_menu_submenu = Submenu::new(
		"KanDo", // This is the menu title on macOS. You may for example have it be the name of your app.
		Menu::new()
		.add_item(version)
		.add_item(change_profile)
		.add_item(open_logs)
		.add_item(devtools)
		.add_item(restart)
		.add_item(quit)
	);

	return Menu::os_default(APP_NAME)
		.add_submenu(app_menu_submenu)
	}

	Menu::new()
	.add_submenu(menu_submenu)
}

pub fn handle_menu_event(event_id: &str, window: &Window<Wry>) {
  let app_handle = window.app_handle();
  let fs = app_handle.state::<AppFileSystem>();
  match event_id {
		"version" => message(Some(&window), APP_NAME, format!("Version {}", app_handle.package_info().version.to_string().as_str())),
    "change_profile" => open_profile_settings(app_handle).unwrap(),
		"open_logs" => open_logs_folder(fs.inner().to_owned()),
    "devtools" => window.open_devtools(),
    "restart" => app_handle.restart(),
    "quit" => app_handle.exit(0),
    _ => {}
  }
}
