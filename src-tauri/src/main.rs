// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use crate::errors::{AppError, AppResult};
use filesystem::AppFileSystem;
use futures::lock::Mutex;
use holochain::conductor::{
    config::{AdminInterfaceConfig, ConductorConfig, KeystoreConfig},
    interface::InterfaceDriver,
    Conductor, ConductorHandle,
};
use holochain_types::prelude::AppBundle;

use holochain_client::{AdminWebsocket, InstallAppPayload};

use menu::{build_menu, handle_menu_event};
use tauri::{Manager, WindowBuilder};

use utils::{sign_zome_call, ZOOM_ON_SCROLL};

const APP_ID: &str = "replace-me";
const PASSWORD: &str = "pass";
// replace-me -- optional: You may want to put a network seed here or read it secretly from an environment variable
const NETWORK_SEED: Option<String> = None;


mod errors;
mod filesystem;
mod menu;
mod utils;



fn main() {
    tauri::Builder::default()
        .menu(build_menu())
        .on_menu_event(|event| handle_menu_event(event.menu_item_id(), event.window()))
        .invoke_handler(tauri::generate_handler![sign_zome_call])
        .setup(|app| {

            let handle = app.handle();

            let profile = String::from("default");

            // start conductor and lair
            let fs = AppFileSystem::new(&handle, &profile)?;

            app.manage(fs.clone());

            tauri::async_runtime::block_on(async move {
                let (conductor, app_port, admin_port) = launch(&fs, PASSWORD.to_string()).await.unwrap();

                app.manage(Mutex::new(conductor));

                let _app_window = WindowBuilder::new(
                    app,
                    "app",
                    tauri::WindowUrl::App("index.html".into())
                  )
                    .inner_size(1200.0, 880.0)
                    .resizable(true)
                    .fullscreen(false)
                    .title(APP_ID) // CHANGE.ME
                    .center()
                    .initialization_script(format!("window.__HC_LAUNCHER_ENV__ = {{ 'APP_INTERFACE_PORT': {}, 'ADMIN_INTERFACE_PORT': {}, 'INSTALLED_APP_ID': '{}' }}", app_port, admin_port, APP_ID).as_str())
                    .initialization_script(ZOOM_ON_SCROLL)
                    .build().unwrap();
            });

            Ok(())

        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}




pub async fn launch(
    fs: &AppFileSystem,
    password: String,
) -> AppResult<(ConductorHandle, u16, u16)> {
    let mut config = ConductorConfig::default();
    config.environment_path = fs.conductor_path().into();
    config.keystore = KeystoreConfig::LairServerInProc {
        lair_root: Some(fs.keystore_path()),
    };

    let admin_port = portpicker::pick_unused_port().expect("Cannot find any unused port");

    config.admin_interfaces = Some(vec![AdminInterfaceConfig {
        driver: InterfaceDriver::Websocket {
            port: admin_port.clone(),
        },
    }]);

    // TODO: set the DHT arc depending on whether this is mobile

    let conductor = Conductor::builder()
        .config(config)
        .passphrase(
            Some(
                utils::vec_to_locked(password.into_bytes())
                    .map_err(|e| AppError::IoError(e))?
            )
        )
        .build()
        .await
        .map_err(|e| AppError::ConductorError(e))?;

    let mut admin_ws = utils::get_admin_ws(admin_port).await?;
    let app_port = conductor
        .clone()
        .add_app_interface(either::Either::Left(0))
        .await
        .map_err(|e| AppError::ConductorError(e))?;

    install_app_if_necessary(NETWORK_SEED, &mut admin_ws).await?;

    Ok((conductor, app_port, admin_port))
}

pub async fn install_app_if_necessary(
    network_seed: Option<String>,
    admin_ws: &mut AdminWebsocket,
) -> AppResult<()> {

    let apps = admin_ws.list_apps(None).await
        .map_err(|e| AppError::ConductorApiError(e))?;

    if !apps
        .iter()
        .map(|info| info.installed_app_id.clone())
        .collect::<Vec<String>>()
        .contains(&APP_ID.to_string())
    {
        let agent_key = admin_ws.generate_agent_pub_key().await
            .map_err(|e| AppError::ConductorApiError(e))?;

        // replace-me --- replace the path with the correct path to your .happ file here
        let app_bundle = AppBundle::decode(include_bytes!("../../pouch/replace-me.happ"))
            .map_err(|e| AppError::AppBundleError(e))?;

        admin_ws
            .install_app(InstallAppPayload {
                source: holochain_types::prelude::AppBundleSource::Bundle(
                    app_bundle,
                ),
                agent_key: agent_key.clone(),
                network_seed: network_seed.clone(),
                installed_app_id: Some(APP_ID.to_string()),
                membrane_proofs: HashMap::new(),
            })
            .await
            .map_err(|e| AppError::ConductorApiError(e))?;

        admin_ws.enable_app(APP_ID.to_string()).await
            .map_err(|e| AppError::ConductorApiError(e))?;
    }

    Ok(())
}




