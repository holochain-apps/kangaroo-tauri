use filesystem::AppFileSystem;
use futures::lock::Mutex;
use holochain_keystore::MetaLairClient;

pub mod filesystem;

pub struct AppState {
    pub fs: AppFileSystem,
    pub app_port: u16,
    pub admin_port: u16,
    pub meta_lair_client: Mutex<MetaLairClient>,
}
