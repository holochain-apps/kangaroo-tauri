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
    let app_version = app_handle.package_info().version;

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

#[derive(Clone)]
pub struct AppFileSystem {
  pub app_data_dir: PathBuf,
  pub app_config_dir: PathBuf,
  pub app_log_dir: PathBuf,
}

impl AppFileSystem {
    pub fn new(app_handle: &AppHandle, profile: &String) -> AppResult<AppFileSystem> {

        let app_data_dir = app_handle
            .path_resolver()
            .app_data_dir()
            .ok_or(AppError::FileSystemError(String::from(
                "Could not resolve the data dir for this app",
            )))?
            .join(breaking_app_version(app_handle)?)
            .join(profile);

        let app_config_dir = app_handle
            .path_resolver()
            .app_config_dir()
            .ok_or(AppError::FileSystemError(String::from(
                "Could not resolve the data dir for this app",
            )))?
            .join(breaking_app_version(app_handle)?)
            .join(profile);

        let app_log_dir = app_handle
            .path_resolver()
            .app_log_dir()
            .ok_or(AppError::FileSystemError(String::from(
                "Could not resolve the log dir for this app",
            )))?
            .join(breaking_app_version(app_handle)?)
            .join(profile);


        Ok(AppFileSystem {
            app_data_dir,
            app_config_dir,
            app_log_dir,
        })
  }

  pub fn keystore_path(&self) -> PathBuf {
      self.app_data_dir.join("keystore")
  }

  pub fn conductor_path(&self) -> PathBuf {
      self.app_data_dir.join("conductor")
  }
}