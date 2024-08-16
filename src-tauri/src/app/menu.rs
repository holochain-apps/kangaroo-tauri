use std::net::SocketAddr;

use holochain_client::{AdminWebsocket, IssueAppAuthenticationTokenPayload};
use tauri::api::dialog::message;
use tauri::api::process;
use tauri::{AppHandle, CustomMenuItem, Manager, Menu, Submenu, Window, WindowBuilder, Wry};

use crate::commands::profile::open_profile_settings;
use crate::config;
use crate::{app_state::filesystem::AppFileSystem, logs::open_logs_folder};

pub async fn build_main_window(
    fs: AppFileSystem,
    app_handle: &AppHandle,
    app_port: u16,
    admin_port: u16,
) -> Window {
    let ws = match AdminWebsocket::connect(SocketAddr::from(([127, 0, 0, 1], admin_port))).await {
        Ok(ws) => ws,
        Err(e) => panic!("Failed to connect to admin websocket: {:?}", e),
    };
    let result = ws
        .issue_app_auth_token(IssueAppAuthenticationTokenPayload {
            installed_app_id: config::APP_ID.to_string(),
            expiry_seconds: 999999,
            single_use: false,
        })
        .await;
    let app_authentication_token = match result {
        Ok(r) => r.token,
        Err(e) => panic!("Failed to issue app authentication token: {:?}", e),
    };

    WindowBuilder::new(
        &app_handle.app_handle(),
        "main",
        tauri::WindowUrl::App("index.html".into()),
    )
    // optional (OSmenu) -- Adds an OS menu to the app
    .menu(build_menu())
    // optional -- diables file drop handler. Disabling is required for drag and drop to work on certain platforms
    .disable_file_drop_handler()
    .inner_size(config::WINDOW_WIDTH, config::WINDOW_HEIGHT)
    .resizable(true)
    .title(config::WINDOW_TITLE)
    .data_directory(fs.profile_data_dir)
    .center()
    .initialization_script(
        format!(
            r#"window.__HC_LAUNCHER_ENV__ = {{
              'APP_INTERFACE_PORT': {}, 
              'ADMIN_INTERFACE_PORT': {}, 
              'INSTALLED_APP_ID': '{}', 
              'APP_INTERFACE_TOKEN": '{:?}',
            }}"#,
            app_port,
            admin_port,
            config::APP_ID,
            app_authentication_token,
        )
        .as_str(),
    )
    .initialization_script(include_str!("../initialization_scripts/zoomOnScroll.js"))
    .build()
    .unwrap()
}

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
            .add_item(quit.clone()),
    );

    // special menu for macOS
    if cfg!(target_os = "macos") {
        let app_menu_submenu = Submenu::new(
            "Menu",
            Menu::new()
                .add_item(version)
                .add_item(change_profile)
                .add_item(open_logs)
                .add_item(devtools)
                .add_item(restart)
                .add_item(quit),
        );

        return Menu::os_default(config::APP_NAME).add_submenu(app_menu_submenu);
    }

    Menu::new().add_submenu(menu_submenu)
}

pub fn handle_menu_event(event_id: &str, window: &Window<Wry>) {
    let app_handle = window.app_handle();
    let fs = app_handle.state::<AppFileSystem>();
    match event_id {
        "version" => message(
            Some(&window),
            config::APP_NAME,
            format!(
                "Version {}",
                app_handle.package_info().version.to_string().as_str()
            ),
        ),
        "change_profile" => open_profile_settings(app_handle).unwrap(),
        "open_logs" => open_logs_folder(fs.inner().to_owned()),
        "devtools" => window.open_devtools(),
        "restart" => {
            process::kill_children();
            app_handle.restart();
        }
        "quit" => {
            process::kill_children();
            app_handle.exit(0)
        }
        _ => {}
    }
}
