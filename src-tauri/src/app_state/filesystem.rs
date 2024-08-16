use std::path::PathBuf;

use tauri::AppHandle;

use crate::{
    errors::{AppError, AppResult},
    utils::breaking_app_version,
};

pub type Profile = String;

#[derive(Debug, Clone)]
pub struct AppFileSystem {
    pub app_data_dir: PathBuf,
    pub profile_data_dir: PathBuf,
    #[allow(dead_code)]
    pub profile_config_dir: PathBuf,
    pub profile_log_dir: PathBuf,
}

impl AppFileSystem {
    pub fn new(app_handle: &AppHandle, profile: &Profile) -> AppResult<AppFileSystem> {
        let app_data_dir = app_handle
            .path_resolver()
            .app_data_dir()
            .ok_or(AppError::FileSystemError(String::from(
                "Could not resolve the data dir for this app",
            )))?
            .join(breaking_app_version(app_handle)?);

        let profile_data_dir = app_data_dir.join(profile);

        let profile_config_dir = app_handle
            .path_resolver()
            .app_config_dir()
            .ok_or(AppError::FileSystemError(String::from(
                "Could not resolve the data dir for this app",
            )))?
            .join(breaking_app_version(app_handle)?)
            .join(profile);

        let profile_log_dir = app_handle
            .path_resolver()
            .app_log_dir()
            .ok_or(AppError::FileSystemError(String::from(
                "Could not resolve the log dir for this app",
            )))?
            .join(breaking_app_version(app_handle)?)
            .join(profile);

        Ok(AppFileSystem {
            app_data_dir,
            profile_data_dir,
            profile_config_dir,
            profile_log_dir,
        })
    }

    pub fn keystore_dir(&self) -> PathBuf {
        self.profile_data_dir.join("keystore")
    }

    pub fn keystore_initialized(&self) -> bool {
        self.keystore_dir()
            .join("lair-keystore-config.yaml")
            .exists()
    }

    pub fn conductor_dir(&self) -> PathBuf {
        self.profile_data_dir.join("conductor")
    }

    pub fn get_existing_profiles(&self) -> Result<Vec<Profile>, String> {
        let mut profiles = Vec::new();

        let dir_entries = std::fs::read_dir(&self.app_data_dir)
            .map_err(|e| format!("Failed to read app data directory: {}", e))?;

        for entry in dir_entries {
            let entry = entry.map_err(|e| {
                log::error!("Got corrupted DirEntry: {}", e);
                format!("Failed to get DirEntry: {}", e)
            })?;

            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    profiles.push(entry.file_name().to_string_lossy().to_string());
                }
            } else {
                log::error!("Failed to get filetype of DirEntry: {:?}", entry);
            }
        }

        Ok(profiles)
    }

    pub fn get_active_profile(&self) -> Profile {
        let active_profile_path = self.app_data_dir.join(".activeProfile");
        match active_profile_path.exists() {
            true => match std::fs::read_to_string(active_profile_path) {
                Ok(profile) => profile,
                Err(e) => {
                    log::error!("Failed to read active profile from file: {}", e);
                    eprintln!("Error: Failed to read active profile from file: {}", e);
                    String::from("default")
                }
            },
            false => String::from("default"),
        }
    }

    pub fn set_active_profile(&self, profile: &Profile) -> Result<(), String> {
        let active_profile_path = self.app_data_dir.join(".activeProfile");
        std::fs::write(active_profile_path, profile)
            .map_err(|e| format!("Failed to set active profile: {}", e))
    }

    /// Writes the network seed to a file in the profile directory
    pub fn set_profile_network_seed(
        &self,
        profile: String,
        network_seed: Option<String>,
    ) -> Result<(), String> {
        if let Some(seed) = network_seed {
            let new_profile_data_dir = self.app_data_dir.join(profile);
            std::fs::create_dir_all(new_profile_data_dir.clone())
                .map_err(|e| format!("Failed to create new profile data directory: {}", e))?;
            let network_seed_path = new_profile_data_dir.join(".networkSeed");
            std::fs::write(network_seed_path, seed)
                .map_err(|e| format!("Failed to write network seed to profile directory: {}", e))?
        }
        Ok(())
    }

    pub fn read_profile_network_seed(&self) -> Option<String> {
        let network_seed_path = self.profile_data_dir.join(".networkSeed");
        match network_seed_path.exists() {
            true => match std::fs::read_to_string(network_seed_path) {
                Ok(seed) => Some(seed),
                Err(e) => {
                    log::error!("Failed to read network seed from file: {}", e);
                    eprintln!("Error: Failed to read network seed from file: {}", e);
                    None
                }
            },
            false => None,
        }
    }
}
