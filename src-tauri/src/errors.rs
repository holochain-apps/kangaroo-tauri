use holochain::{conductor::error::ConductorError, prelude::AppBundleError};
use holochain_client::ConductorApiError;


#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Filesystem error: `{0}`")]
    FileSystemError(String),

    #[error("Applets UI server error: `{0}`")]
    AppletsUIServerError(String),

    #[error("Holochain is not running")]
    NotRunning,

    #[error("ConductorApiError: `{0:?}`")]
    ConductorApiError(ConductorApiError),

    #[error("Database error: `{0}`")]
    DatabaseError(String),

    #[error("Failed to conver package version to breaking version string: `{0:?}`")]
    SemVerError(semver::Error),

    #[error(transparent)]
    AppBundleError(#[from] AppBundleError),

    // #[error(transparent)]
    // ZipError(#[from] ZipError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    MrBundleError(#[from] mr_bundle::error::MrBundleError),

    #[error(transparent)]
    ConductorError(#[from] ConductorError),

    #[error(transparent)]
    TauriError(#[from] tauri::Error),

    #[error("Admin Websocket Error: `{0}`")]
    AdminWebsocketError(String),

    #[error("App Websocket Error: `{0}`")]
    AppWebsocketError(String),

    #[error("Error signing zome call: `{0}`")]
    SignZomeCallError(String),
}


pub type AppResult<T> = Result<T, AppError>;
