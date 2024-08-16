use crate::{
    app_state::{
        filesystem::{AppFileSystem, Profile},
        AppState,
    },
    config,
    launch::launch,
    logs::setup_logs,
};
use futures::lock::Mutex;
use serde_json::Value;
use tauri::{App, Manager};
use window::build_main_window;

pub mod system_tray;
pub mod window;

pub fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();

    // convert profile from CLI to option, then read from filesystem instead. if profile from CLI,
    // then set current profile!
    let profile = match read_profile_from_cli(app)? {
        Some(profile) => profile,
        None => {
            // optional (single-instance) -- Allows only a single instance of your app running. Useful in combination with the systray
            handle.plugin(tauri_plugin_single_instance::init(
                move |app, _argv, _cwd| {
                    let main_window = app.get_window("main");
                    if let Some(window) = main_window {
                        window.show().unwrap();
                        window.unminimize().unwrap();
                        window.set_focus().unwrap();
                    } else {
                        let state = app.state::<AppState>().inner().to_owned();
                        tauri::async_runtime::block_on(async {
                            build_main_window(
                                state.fs.clone(),
                                app,
                                state.app_port,
                                state.admin_port,
                            )
                            .await;
                        })
                    }
                },
            ))?;

            let fs_tmp = AppFileSystem::new(&handle, &String::from("default"))?;
            fs_tmp.get_active_profile()
        }
    };

    tauri::async_runtime::block_on(async move {
        let fs = AppFileSystem::new(&handle, &profile).unwrap();
        // set up logs
        if let Err(err) = setup_logs(fs.clone()) {
            println!("Error setting up the logs: {:?}", err);
        }
        let (meta_lair_client, app_port, admin_port) =
            launch(&fs, config::PASSWORD.to_string()).await.unwrap();
        let app_state = AppState {
            fs: fs.clone(),
            app_port,
            admin_port,
            meta_lair_client: Mutex::new(meta_lair_client),
        };
        app.manage(app_state);
        build_main_window(fs, &app.app_handle(), app_port, admin_port).await;
    });

    Ok(())
}

pub fn read_profile_from_cli(app: &mut App) -> Result<Option<Profile>, tauri::Error> {
    // reading profile from cli
    let cli_matches = app.get_cli_matches()?;
    if let Some(Value::String(profile)) = cli_matches
        .args
        .get("profile")
        .and_then(|data| Some(data.value.clone()))
    {
        // Validate the profile name
        if profile == "default" {
            eprintln!("error: the name 'default' is not allowed for a profile.");
            panic!("error: the name 'default' is not allowed for a profile.");
        }
        if profile.contains("/") || profile.contains("\\") || profile.contains("?") {
            eprintln!("error: \"/\", \"\\\", and \"?\" are not allowed in profile names.");
            panic!("error: \"/\", \"\\\", and \"?\" are not allowed in profile names.");
        }
        return Ok(Some(profile));
    }

    Ok(None)
}
