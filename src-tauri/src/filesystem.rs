use std::path::PathBuf;

use tauri::AppHandle;

use crate::errors::{AppError, AppResult};


/// Returns a string considering the relevant part of the version regarding breaking changes
/// Examples:
/// 3.2.0 becomes 3.x.x
/// 0.2.2 becomes 0.2.x
/// 0.0.5 becomse 0.0.5
pub fn breaking_app_version() -> AppResult<String> {
    let cargo_version = semver::Version::parse(env!("CARGO_PKG_VERSION"))
        .map_err(|e| AppError::SemVerError(e))?;

    // has major digit? If yes, only consider major component
    let breaking_version_string = match cargo_version.major {
        0 => {
            match cargo_version.minor {
                0 => format!("0.0.{}", cargo_version.patch),
                _ => format!("0.{}.x", cargo_version.minor),
            }
        },
        _ => format!("{}.x.x", cargo_version.major)
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
            .join(breaking_app_version()?)
            .join(profile);

        let app_config_dir = app_handle
            .path_resolver()
            .app_config_dir()
            .ok_or(AppError::FileSystemError(String::from(
                "Could not resolve the data dir for this app",
            )))?
            .join(breaking_app_version()?)
            .join(profile);

        let app_log_dir = app_handle
            .path_resolver()
            .app_log_dir()
            .ok_or(AppError::FileSystemError(String::from(
                "Could not resolve the log dir for this app",
            )))?
            .join(breaking_app_version()?)
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