use std::{collections::HashMap, net::SocketAddr, time::Duration};

use holochain::{
    conductor::{
        api::{AdminInterfaceConfig, InterfaceDriver},
        config::{ConductorConfig, KeystoreConfig},
    },
    prelude::{
        dependencies::kitsune_p2p_types::config::{KitsuneP2pConfig, TransportConfig},
        AppBundle,
    },
};
use holochain_client::{AdminWebsocket, InstallAppPayload};
use holochain_keystore::MetaLairClient;
use holochain_types::websocket::AllowedOrigins;
use tauri::api::process::Command;

use crate::{
    app_state::filesystem::AppFileSystem,
    config::{APP_ID, BOOTSTRAP_SERVER, DEFAULT_NETWORK_SEED, HOLOCHAIN_VERSION, SIGNALING_SERVER},
    errors::{AppError, AppResult, LairKeystoreError, LaunchHolochainError},
    process::{
        conductor::launch_holochain_process,
        lair::{initialize_keystore, launch_lair_keystore_process},
    },
};

pub async fn launch(fs: &AppFileSystem, password: String) -> AppResult<(MetaLairClient, u16, u16)> {
    let log_level = log::Level::Info;

    if !fs.keystore_dir().exists() {
        std::fs::create_dir_all(fs.keystore_dir())?;
    }

    if !fs.conductor_dir().exists() {
        std::fs::create_dir_all(fs.conductor_dir())?;
    }

    // initialize lair keystore if necessary
    if !fs.keystore_initialized() {
        initialize_keystore(fs.keystore_dir(), password.clone()).await?;
    }

    // spawn lair keystore process and connect to it
    let lair_url =
        launch_lair_keystore_process(log_level.clone(), fs.keystore_dir(), password.clone())
            .await?;

    let meta_lair_client = holochain_keystore::lair_keystore::spawn_lair_keystore(
        lair_url.clone(),
        password.as_bytes().into(),
    )
    .await
    .map_err(|e| LairKeystoreError::SpawnMetaLairClientError(format!("{}", e)))?;

    // write conductor config to file

    let mut config = ConductorConfig::default();
    config.data_root_path = Some(fs.conductor_dir().into());
    config.keystore = KeystoreConfig::LairServer {
        connection_url: lair_url,
    };

    let admin_port = portpicker::pick_unused_port().expect("Cannot find any unused port");

    config.admin_interfaces = Some(vec![AdminInterfaceConfig {
        driver: InterfaceDriver::Websocket {
            port: admin_port.clone(),
            allowed_origins: AllowedOrigins::Any,
        },
    }]);

    let mut network_config = KitsuneP2pConfig::default();
    network_config.bootstrap_service = Some(url2::url2!("{}", BOOTSTRAP_SERVER));
    network_config.transport_pool.push(TransportConfig::WebRTC {
        signal_url: SIGNALING_SERVER.into(),
    });

    config.network = network_config;

    // TODO more graceful error handling
    let config_string =
        serde_yaml::to_string(&config).expect("Could not convert conductor config to string");

    let conductor_config_path = fs.conductor_dir().join("conductor-config.yaml");

    std::fs::write(conductor_config_path.clone(), config_string)
        .expect("Could not write conductor config");

    let command =
        Command::new_sidecar(format!("holochain-v{}", HOLOCHAIN_VERSION)).map_err(|err| {
            AppError::LaunchHolochainError(LaunchHolochainError::SidecarBinaryCommandError(
                format!("{}", err),
            ))
        })?;

    launch_holochain_process(log_level, command, conductor_config_path, password).await?;

    std::thread::sleep(Duration::from_millis(100));

    // Try to connect twice. This fixes the os(111) error for now that occurs when the conducor is not ready yet.
    let mut admin_ws = connect_to_admin_ws_with_retries(admin_port, 3).await?;

    let app_port = {
        let app_interfaces = admin_ws.list_app_interfaces().await.map_err(|e| {
            LaunchHolochainError::CouldNotConnectToConductor(format!(
                "Could not list app interfaces: {:?}",
                e
            ))
        })?;

        if app_interfaces.len() > 0 {
            app_interfaces[0].port
        } else {
            let free_port = portpicker::pick_unused_port().expect("No ports free");

            admin_ws
                .attach_app_interface(free_port, AllowedOrigins::Any, Some(APP_ID.to_owned()))
                .await
                .or(Err(LaunchHolochainError::CouldNotConnectToConductor(
                    "Could not attach app interface".into(),
                )))?;
            free_port
        }
    };

    let network_seed = match fs.read_profile_network_seed() {
        Some(seed) => Some(seed),
        None => DEFAULT_NETWORK_SEED.map(String::from),
    };

    install_app_if_necessary(network_seed, &mut admin_ws).await?;

    Ok((meta_lair_client, app_port, admin_port))
}

pub async fn install_app_if_necessary(
    network_seed: Option<String>,
    admin_ws: &mut AdminWebsocket,
) -> AppResult<()> {
    let apps = admin_ws
        .list_apps(None)
        .await
        .map_err(|e| AppError::ConductorApiError(e))?;

    if !apps
        .iter()
        .map(|info| info.installed_app_id.clone())
        .collect::<Vec<String>>()
        .contains(&APP_ID.to_string())
    {
        let agent_key = admin_ws
            .generate_agent_pub_key()
            .await
            .map_err(|e| AppError::ConductorApiError(e))?;

        // replace-me --- replace the path with the correct path to your .happ file here
        let app_bundle = AppBundle::decode(include_bytes!("../../pouch/replace-me.happ"))
            .map_err(|e| AppError::AppBundleError(e))?;

        admin_ws
            .install_app(InstallAppPayload {
                source: holochain_types::prelude::AppBundleSource::Bundle(app_bundle),
                agent_key: agent_key.clone(),
                network_seed: network_seed.clone(),
                installed_app_id: Some(APP_ID.to_string()),
                membrane_proofs: HashMap::new(),
            })
            .await
            .map_err(|e| AppError::ConductorApiError(e))?;

        admin_ws
            .enable_app(APP_ID.to_string())
            .await
            .map_err(|e| AppError::ConductorApiError(e))?;
    }

    Ok(())
}

async fn connect_to_admin_ws_with_retries(
    admin_port: u16,
    max_retries: u32,
) -> Result<AdminWebsocket, LaunchHolochainError> {
    let mut attempts = 0;

    loop {
        match AdminWebsocket::connect(SocketAddr::from(([127, 0, 0, 1], admin_port))).await {
            Ok(ws) => return Ok(ws),
            Err(_) if attempts < max_retries => {
                attempts += 1;
                log::error!("[HOLOCHAIN] Could not connect to the AdminWebsocket. Attempt {}/{}. Retrying in 5 seconds.", attempts, max_retries);
                std::thread::sleep(Duration::from_secs(5));
            }
            Err(err) => {
                return Err(LaunchHolochainError::CouldNotConnectToConductor(format!(
                    "{}",
                    err
                )));
            }
        }
    }
}
