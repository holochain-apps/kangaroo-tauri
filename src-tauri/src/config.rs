/// name of the app. Can be changed without breaking your app.
pub const APP_NAME: &str = "forum";

/// App id used to install your app in the Holochain conductor - can be the same as APP_NAME. Changing this means a breaking change to your app.
pub const APP_ID: &str = "forum";

/// Title of the window
pub const WINDOW_TITLE: &str = "forum";

/// Default window width when the app is opened
pub const WINDOW_WIDTH: f64 = 1400.0;

/// Default window height when the app is opened
pub const WINDOW_HEIGHT: f64 = 880.0;

/// Password to the lair keystore
pub const PASSWORD: &str = "pass";

/// replace-me (optional): Depending on your application, you may want to put a network seed here or
/// read it secretly from an environment variable. If so, replace `None` with `Some("your network seed here")`
pub const DEFAULT_NETWORK_SEED: Option<&str> = None;

/// (optional): Change the signaling server if you want
pub const SIGNALING_SERVER: &str = "wss://sbd-0.main.infra.holo.host";

/// (optional) -- change bootstrap server URL here if desired
pub const BOOTSTRAP_SERVER: &str = "https://bootstrap.holo.host";

/// derived from build script, can be overridden
pub const HOLOCHAIN_VERSION: &str = env!("HOLOCHAIN_VERSION");

pub const LAIR_KEYSTORE_VERSION: &str = "0.4.5";
