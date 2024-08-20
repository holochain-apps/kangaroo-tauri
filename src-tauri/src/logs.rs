use log::LevelFilter;
use log4rs::{
    self,
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

use crate::app_state::filesystem::AppFileSystem;
use crate::config::APP_ID;

pub fn setup_logs(fs: AppFileSystem) -> Result<(), String> {
    let logs_path = fs.profile_log_dir.join(format!("{}.log", APP_ID));

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[{d}] {l} - {m}\n")))
        .build(logs_path)
        .map_err(|err| format!("Could not build log config: {:?}", err))?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .map_err(|err| format!("Could not init log config: {:?}", err))?;

    log4rs::init_config(config).map_err(|err| format!("Could not init log config: {:?}", err))?;

    Ok(())
}

/// Opens the folder where the logs are stored for the given profile
pub fn open_logs_folder(fs: AppFileSystem) -> () {
    let logs_dir = fs.profile_log_dir;

    if let Err(err) = opener::open(logs_dir) {
        log::error!("Error opening logs folder: {}", err);
    }
}
