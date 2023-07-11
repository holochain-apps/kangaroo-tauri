use std::path::PathBuf;

use tauri::AppHandle;

use crate::errors::{AppError, AppResult};




pub fn app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[derive(Clone)]
pub struct AppFileSystem {
  pub app_data_dir: PathBuf,
  pub app_config_dir: PathBuf,
}

impl AppFileSystem {
  pub fn new(app_handle: &AppHandle, profile: &String) -> AppResult<AppFileSystem> {

      let app_data_dir = app_handle
          .path_resolver()
          .app_data_dir()
          .ok_or(AppError::FileSystemError(String::from(
              "Could not resolve the data dir for this app",
          )))?
          .join(app_version())
          .join(profile);

      let app_config_dir = app_handle
          .path_resolver()
          .app_config_dir()
          .ok_or(AppError::FileSystemError(String::from(
              "Could not resolve the data dir for this app",
          )))?
          .join(app_version())
          .join(profile);

      Ok(AppFileSystem {
          app_data_dir,
          app_config_dir,
      })
  }

  pub fn keystore_path(&self) -> PathBuf {
      self.app_data_dir.join("keystore")
  }

  pub fn conductor_path(&self) -> PathBuf {
      self.app_data_dir.join("conductor")
  }
}