// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use crate::errors::{AppError, AppResult};
use filesystem::AppFileSystem;
use futures::lock::Mutex;
use holochain::{conductor::{
    config::{AdminInterfaceConfig, ConductorConfig, KeystoreConfig},
    interface::InterfaceDriver,
    Conductor, ConductorHandle,
}, prelude::kitsune_p2p::dependencies::kitsune_p2p_types::dependencies::lair_keystore_api::prelude::BinDataSized};
use holochain_types::prelude::{AppBundle, ZomeCallUnsigned};
use holochain_zome_types::{Signature, CellId, ZomeName, FunctionName, CapSecret, ExternIO, Timestamp, Nonce256Bits};

use serde::Deserialize;


use holochain_client::{AdminWebsocket, InstallAppPayload, ZomeCall, AgentPubKey};

use menu::build_menu;
use tauri::{Manager, WindowBuilder};

const APP_ID: &str = "replace-me";
const PASSWORD: &str = "pass";
// replace-me -- optional: You may want to put a network seed here or read it secretly from an environment variable
const NETWORK_SEED: Option<String> = None;


mod errors;
mod filesystem;
mod menu;


#[tauri::command]
async fn sign_zome_call(
    conductor: tauri::State<'_, futures::lock::Mutex<ConductorHandle>>,
    zome_call_unsigned: ZomeCallUnsignedTauri,
) -> Result<ZomeCall, String> {
    let zome_call_unsigned_converted: ZomeCallUnsigned = zome_call_unsigned.into();

    let conductor = conductor.lock().await;
    let lair_client = conductor.keystore().lair_client();

    let pub_key = zome_call_unsigned_converted.provenance.clone();
    let mut pub_key_2 = [0; 32];
    pub_key_2.copy_from_slice(pub_key.get_raw_32());

    let data_to_sign = zome_call_unsigned_converted.data_to_sign().unwrap();
        // .map_err(|e| format!("Failed to get data to sign from unsigned zome call: {}", e))
        // .map_err(|e| AppError::SignZomeCallError(e))?;

    let sig = lair_client.sign_by_pub_key(
        BinDataSized::from(pub_key_2),
        None,
        data_to_sign,
    ).await.unwrap();
        // .map_err(|e| AppError::SignZomeCallError(e.to_string()))?;

    let signature = Signature(*sig.0);

    let signed_zome_call = ZomeCall {
        cell_id: zome_call_unsigned_converted.cell_id,
        zome_name: zome_call_unsigned_converted.zome_name,
        fn_name: zome_call_unsigned_converted.fn_name,
        payload: zome_call_unsigned_converted.payload,
        cap_secret: zome_call_unsigned_converted.cap_secret,
        provenance: zome_call_unsigned_converted.provenance,
        nonce: zome_call_unsigned_converted.nonce,
        expires_at: zome_call_unsigned_converted.expires_at,
        signature
    };

    Ok(signed_zome_call)
}

fn main() {
    tauri::Builder::default()
        .menu(build_menu())
        .invoke_handler(tauri::generate_handler![sign_zome_call])
        .setup(|app| {

            let handle = app.handle();

            let profile = String::from("default");

            // start conductor and lair
            let fs = AppFileSystem::new(&handle, &profile)?;

            app.manage(fs.clone());

            tauri::async_runtime::block_on(async move {
                let (conductor, app_port) = launch(&fs, PASSWORD.to_string()).await.unwrap();

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
                    .initialization_script(format!("window.__HC_LAUNCHER_ENV__ = {{ 'APP_INTERFACE_PORT': {}, 'INSTALLED_APP_ID': '{}' }}", app_port, APP_ID).as_str())
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
) -> AppResult<(ConductorHandle, u16)> {
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
                vec_to_locked(password.into_bytes())
                    .map_err(|e| AppError::IoError(e))?
            )
        )
        .build()
        .await
        .map_err(|e| AppError::ConductorError(e))?;

    let mut admin_ws = get_admin_ws(admin_port).await?;
    let app_port = conductor
        .clone()
        .add_app_interface(either::Either::Left(0))
        .await
        .map_err(|e| AppError::ConductorError(e))?;

    install_app_if_necessary(NETWORK_SEED, &mut admin_ws).await?;

    Ok((conductor, app_port))
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

        // unpack happ here
        // CHANGE.ME
        let app_bundle = AppBundle::decode(include_bytes!("../../pouch/talking-stickies_0.3.0-beta-dev.8_2.happ"))
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



pub async fn get_admin_ws(admin_port: u16) -> AppResult<AdminWebsocket> {
    let admin_ws = AdminWebsocket::connect(format!(
        "ws://localhost:{}",
        admin_port
    ))
    .await
    .map_err(|err| {
        AppError::AdminWebsocketError(format!("Could not connect to the admin interface: {}", err))
    })?;

    Ok(admin_ws)
}

fn vec_to_locked(mut pass_tmp: Vec<u8>) -> std::io::Result<sodoken::BufRead> {
    match sodoken::BufWrite::new_mem_locked(pass_tmp.len()) {
        Err(e) => {
            pass_tmp.fill(0);
            Err(e.into())
        }
        Ok(p) => {
            {
                let mut lock = p.write_lock();
                lock.copy_from_slice(&pass_tmp);
                pass_tmp.fill(0);
            }
            Ok(p.to_read())
        }
    }
}


/// The version of an unsigned zome call that's compatible with the serialization
/// behavior of tauri's IPC channel (serde serialization)
/// nonce is a byte array [u8, 32] because holochain's nonce type seems to
/// have "non-serde" deserialization behavior.
#[derive(Deserialize, Debug, Clone)]
pub struct ZomeCallUnsignedTauri {
  pub provenance: AgentPubKey,
  pub cell_id: CellId,
  pub zome_name: ZomeName,
  pub fn_name: FunctionName,
  pub cap_secret: Option<CapSecret>,
  pub payload: ExternIO,
  pub nonce: [u8; 32],
  pub expires_at: Timestamp,
}


impl Into<ZomeCallUnsigned> for ZomeCallUnsignedTauri {
  fn into(self) -> ZomeCallUnsigned {
    ZomeCallUnsigned {
      provenance: self.provenance,
      cell_id: self.cell_id,
      zome_name: self.zome_name,
      fn_name: self.fn_name,
      cap_secret: self.cap_secret,
      payload: self.payload,
      nonce: self.nonce.into(),
      expires_at: self.expires_at,
    }
  }
}
