use std::path::PathBuf;

use tauri::AppHandle;

use crate::errors::{AppError, AppResult};


/// Returns a string considering the relevant part of the version regarding breaking changes
/// Examples:
/// 3.2.0 becomes 3.x.x
/// 0.2.2 becomes 0.2.x
/// 0.0.5 becomes 0.0.5
/// 0.2.3-alpha.2 remains 0.2.3-alpha.2 --> pre-releases always get their own storage location since we have to assume breaking changes
pub fn breaking_app_version(app_handle: &AppHandle) -> AppResult<String> {
    let app_version = app_handle.package_info().version.clone();

    if app_version.pre.is_empty() == false {
        return Ok(app_version.to_string());
    }

    let breaking_version_string = match app_version.major {
        0 => {
            match app_version.minor {
                0 => format!("0.0.{}", app_version.patch),
                _ => format!("0.{}.x", app_version.minor),
            }
        },
        _ => format!("{}.x.x", app_version.major)
    };

    Ok(breaking_version_string)
}

pub type Profile = String;
#[derive(Clone)]
pub struct AppFileSystem {
  pub app_data_dir: PathBuf,
  pub profile_data_dir: PathBuf,
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

  pub fn keystore_path(&self) -> PathBuf {
      self.profile_data_dir.join("keystore")
  }

  pub fn conductor_path(&self) -> PathBuf {
      self.profile_data_dir.join("conductor")
  }

  pub fn get_existing_profiles(&self) -> Result<Vec<Profile>, String> {
    let mut profiles: Vec<Profile> = vec![];
    match std::fs::read_dir(&self.app_data_dir) {
        Ok(dir) => {
            for entry_result in dir {
                match entry_result {
                    Ok(entry) => match entry.file_type() {
                        Ok(file_type) => {
                            if file_type.is_dir() {
                                profiles.push(entry.file_name().to_string_lossy().to_string());
                            }
                        },
                        Err(e) => log::error!("Failed to get filetype of DirEntry: {}", e),
                    },
                    Err(e) => log::error!("Got corrupted DirEntry: {}", e)
                }
            }
        },
        Err(e) => return Err(format!("Failed to read app data directory: {}", e))
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
        false => String::from("default")
    }
  }

  pub fn set_active_profile(&self, profile: &Profile) -> Result<(), String> {
    let active_profile_path = self.app_data_dir.join(".activeProfile");
    std::fs::write(active_profile_path, profile)
        .map_err(|e| format!("Failed to set active profile: {}", e))
  }
}