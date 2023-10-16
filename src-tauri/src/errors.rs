use holochain::{conductor::error::ConductorError, prelude::AppBundleError};
use holochain_client::ConductorApiError;
use serde::{Deserialize, Serialize};
use thiserror::Error;


#[derive(Debug, Error)]
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

    #[error(transparent)]
    LairKeystoreError(#[from] LairKeystoreError),

    #[error(transparent)]
    LaunchHolochainError(#[from] LaunchHolochainError),
}


pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Serialize, Deserialize, Debug, Clone)]
pub enum LaunchHolochainError {
    #[error("Failed to launch child: `{0}`")]
    LaunchChildError(#[from] LaunchChildError),

    #[error("Failed to write the password: `{0}`")]
    ErrorWritingPassword(String),

    #[error("Error with the filesystem: `{0}`")]
    IoError(String),

    #[error("Could not connect to the conductor: `{0}`")]
    CouldNotConnectToConductor(String),

    #[error("Could not initialize conductor: `{0}`")]
    CouldNotInitializeConductor(#[from] InitializeConductorError),

    #[error("Failed to overwrite config: `{0}`")]
    FailedToOverwriteConfig(String),

    #[error("Failed to create sidecar binary command: `{0}`")]
    SidecarBinaryCommandError(String),

    #[error("Impossible error: `{0}`")]
    ImpossibleError(String),
}


#[derive(Error, Serialize, Deserialize, Debug, Clone)]
pub enum InitializeConductorError {
    #[error("Unknown Error: `{0}`")]
    UnknownError(String),

    #[error("Could not connect to the database of the conductor: `{0}`")]
    SqliteError(String),

    #[error("Address already in use: `{0}`")]
    AddressAlreadyInUse(String),
}


#[derive(Error, Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
pub enum LairKeystoreError {

    #[error("Failed to launch child: `{0}`")]
    LaunchChildError(#[from] LaunchChildError),

    #[error("Failed to write the password: `{0}`")]
    ErrorWritingPassword(String),

    #[error("Incorrect password")]
    IncorrectPassword,

    #[error("Failed to create LairClient: `{0}`")]
    ErrorCreatingLairClient(String),

    #[error("Failed to create temp dir: `{0}`")]
    ErrorReadingLairConfig(String),

    #[error("Failed to read lair-keysstore-config.yaml: `{0}`")]
    ErrorWritingLairConfig(String),

    #[error("Error creating a symlink of the lair directory: `{0}`")]
    ErrorCreatingSymLink(String),

    #[error("Lair Keystore Error: `{0}`")]
    OtherError(String),

    #[error("Failed to sign zome call: `{0}`")]
    SignZomeCallError(String),

    #[error("Failed to spawn MetaLairClient: `{0}`")]
    SpawnMetaLairClientError(String),
}

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "content")]
pub enum LaunchChildError {
    #[error("Sidecar binary was not found")]
    BinaryNotFound,

    #[error("Failed to execute sidecar binary: `{0}`")]
    FailedToExecute(String),
}