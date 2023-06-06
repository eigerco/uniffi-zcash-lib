use std::{
    fs::{read_to_string, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
    time::Duration,
};

use anyhow::anyhow;
use fs_extra::dir;
use handlebars::Handlebars;
use retry::{retry_with_index, OperationResult};
use serde::Serialize;
use uuid::Uuid;

pub const LINUX_SHARED_LIB_NAME: &str = "libuniffi_zcash.so";
pub const MACOS_SHARED_LIB_NAME: &str = "libuniffi_zcash.dylib";

pub const TARGET_LINUX_X86_64: &str = "x86_64-unknown-linux-gnu";
pub const TARGET_MACOS_64: &str = "aarch64-apple-darwin";
pub const TARGET_MACOS_X86_64: &str = "x86_64-apple-darwin";
pub const TARGET_MACOS_UNIVERSAL2: &str = "universal2-apple-darwin";
pub const TARGETS: [&str; 3] = [TARGET_LINUX_X86_64, TARGET_MACOS_64, TARGET_MACOS_X86_64];

/// Overwrites the provided file by rendering the provided data on it.
pub fn in_file_template_replace<P, T>(file_path: P, data: &T) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    T: Serialize,
{
    let content = read_to_string(&file_path)?;
    let reg = Handlebars::new();
    let rendered = reg.render_template(&content, data)?;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file_path)?;
    file.write_all(rendered.as_bytes())?;
    Ok(())
}

pub fn workspace_root_dir() -> anyhow::Result<PathBuf> {
    let err_msg = "Cannot find parent path.";
    Ok(std::env::current_exe()?
        .parent()
        .ok_or(anyhow!(err_msg))?
        .parent()
        .ok_or(anyhow!(err_msg))?
        .parent()
        .ok_or(anyhow!(err_msg))?
        .to_owned())
}

/// Wraps a call to [`std::process::Command::wait()`] . In case
/// of error, it returns the status code as error, so users can use ?
/// easily to return immediately.
pub fn cmd_success(cmd_result: io::Result<ExitStatus>) -> anyhow::Result<()> {
    let status = cmd_result?;
    match status.success() {
        true => Ok(()),
        false => match status.code() {
            Some(code) => Err(anyhow!("Command exited with non zero error: {}", code)),
            None => Err(anyhow!("Process terminated by signal")),
        },
    }
}

/// Wraps the retry library and adapts it to our specific use
/// of commands. It does not pretend to decouple foreign types.
pub fn cmd_retry<I>(
    name: &str,
    interval: I,
    max_retries: u64,
    mut cmd: Command,
) -> anyhow::Result<()>
where
    I: IntoIterator<Item = Duration>,
{
    retry_with_index(interval, |current_try| {
        if current_try > max_retries {
            return OperationResult::Err(anyhow!(
                "Command {}: Max tries of {} reached, aborting ...",
                name,
                current_try
            ));
        }

        let cmd = cmd.spawn();

        if let Err(err) = cmd {
            return OperationResult::Err(anyhow!("Error spawning process ! {}", err));
        }

        match cmd_success(cmd.unwrap().wait()) {
            Ok(ok) => OperationResult::Ok(ok),
            Err(err) => OperationResult::Retry(err),
        }
    })
    .map_err(|err| err.error)
}

/// Generates a collision free /tmp folder
pub fn tmp_folder() -> anyhow::Result<PathBuf> {
    let uuid = Uuid::new_v4();
    let name = format!("zcash_uniffi_{}", uuid);
    let path_buff = std::env::temp_dir().join(name);
    dir::create_all(&path_buff, false)?;
    Ok(path_buff)
}

/// Removes the dir if exists and creates the entire
/// path if there are missing elements.
pub fn clean_dir(dir: &PathBuf) -> anyhow::Result<()> {
    Ok(dir::create_all(dir, true)?)
}
