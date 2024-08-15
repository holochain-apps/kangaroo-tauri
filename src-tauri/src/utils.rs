use holochain_client::AdminWebsocket;
use std::path::PathBuf;
use tauri::AppHandle;

use crate::errors::{AppError, AppResult, LairKeystoreError};

#[allow(dead_code)]
pub async fn get_admin_ws(admin_port: u16) -> AppResult<AdminWebsocket> {
    let admin_ws = AdminWebsocket::connect(format!("ws://localhost:{}", admin_port))
        .await
        .map_err(|err| {
            AppError::AdminWebsocketError(format!(
                "Could not connect to the admin interface: {}",
                err
            ))
        })?;

    Ok(admin_ws)
}

#[allow(dead_code)]
pub fn vec_to_locked(mut pass_tmp: Vec<u8>) -> std::io::Result<sodoken::BufRead> {
    match sodoken::BufWrite::new_mem_locked(pass_tmp.len()) {
        Err(e) => {
            pass_tmp.fill(0);
            Err(e.into())
        }
        Ok(p) => {
            {
                let mut lock = p.write_lock();
                lock.copy_from_slice(&pass_tmp);
                pass_tmp.fill(0);
            }
            Ok(p.to_read())
        }
    }
}

///On Unix systems, there is a limit to the path length of a domain socket. This function creates a symlink to
/// the lair directory from the tempdir instead and overwrites the connectionUrl in the lair-keystore-config.yaml
#[allow(dead_code)]
pub fn create_and_apply_lair_symlink(keystore_data_dir: PathBuf) -> AppResult<()> {
    let mut keystore_dir = keystore_data_dir.clone();

    let uid = nanoid::nanoid!(13);
    let src_path = std::env::temp_dir().join(format!("lair.{}", uid));
    symlink::symlink_dir(keystore_dir, src_path.clone()).map_err(|e| {
        AppError::LairKeystoreError(LairKeystoreError::ErrorCreatingSymLink(format!(
            "Failed to create symlink directory for lair keystore: {}",
            e
        )))
    })?;
    keystore_dir = src_path;

    // overwrite connectionUrl in lair-keystore-config.yaml to symlink directory
    // 1. read to string
    let mut lair_config_string =
        std::fs::read_to_string(keystore_dir.join("lair-keystore-config.yaml")).map_err(|e| {
            LairKeystoreError::ErrorCreatingSymLink(format!(
                "Failed to read lair-keystore-config.yaml: {}",
                e
            ))
        })?;

    // 2. filter out the line with the connectionUrl
    let connection_url_line = lair_config_string
        .lines()
        .filter(|line| line.contains("connectionUrl:"))
        .collect::<String>();

    // 3. replace the part unix:///home/[user]/.local/share/holochain-launcher/profiles/default/lair/0.2/socket?k=[some_key]
    //    with unix://[path to tempdir]/socket?k=[some_key]
    let split_byte_index = connection_url_line.rfind("socket?").unwrap();
    let socket = &connection_url_line.as_str()[split_byte_index..];
    let tempdir_connection_url = match url::Url::parse(&format!(
        "unix://{}",
        keystore_dir.join(socket).to_str().unwrap(),
    )) {
        Ok(url) => url,
        Err(e) => {
            return Err(AppError::LairKeystoreError(
                LairKeystoreError::ErrorCreatingSymLink(format!(
                    "Failed to parse URL for symlink lair path: {}",
                    e
                )),
            ))
        }
    };

    let new_line = &format!("connectionUrl: {}\n", tempdir_connection_url);

    // 4. Replace the existing connectionUrl line with that new line
    lair_config_string = LinesWithEndings::from(lair_config_string.as_str())
        .map(|line| {
            if line.contains("connectionUrl:") {
                new_line
            } else {
                line
            }
        })
        .collect::<String>();

    // 5. Overwrite the lair-keystore-config.yaml with the modified content
    std::fs::write(
        keystore_dir.join("lair-keystore-config.yaml"),
        lair_config_string,
    )
    .map_err(|e| {
        AppError::LairKeystoreError(LairKeystoreError::ErrorCreatingSymLink(format!(
            "Failed to write lair-keystore-config.yaml after modification: {}",
            e
        )))
    })
}

/// Iterator yielding every line in a string. The line includes newline character(s).
/// https://stackoverflow.com/questions/40455997/iterate-over-lines-in-a-string-including-the-newline-characters
pub struct LinesWithEndings<'a> {
    input: &'a str,
}

impl<'a> From<&'a str> for LinesWithEndings<'a> {
    fn from(input: &'a str) -> Self {
        LinesWithEndings { input }
    }
}

impl<'a> Iterator for LinesWithEndings<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        if self.input.is_empty() {
            return None;
        }
        let split = self
            .input
            .find('\n')
            .map(|i| i + 1)
            .unwrap_or(self.input.len());
        let (line, rest) = self.input.split_at(split);
        self.input = rest;
        Some(line)
    }
}

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
        0 => match app_version.minor {
            0 => format!("0.0.{}", app_version.patch),
            _ => format!("0.{}.x", app_version.minor),
        },
        _ => format!("{}.x.x", app_version.major),
    };

    Ok(breaking_version_string)
}
