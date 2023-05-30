use std::{
    fs::{create_dir_all, read_to_string, remove_dir_all, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
    time::Duration,
};

use handlebars::Handlebars;
use retry::{retry_with_index, OperationResult};
use serde::Serialize;
use uuid::Uuid;

use crate::cli::CLIResult;

/// Overwrites the provided file by rendering the provided data on it.
pub fn in_file_template_replace<P, T>(file_path: P, data: &T) -> CLIResult<()>
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

pub fn workspace_root_dir() -> CLIResult<PathBuf> {
    let err_msg = "Cannot find parent path.";
    Ok(std::env::current_exe()?
        .parent()
        .ok_or(err_msg)?
        .parent()
        .ok_or(err_msg)?
        .parent()
        .ok_or(err_msg)?
        .to_owned())
}

/// Wraps a call to [`std::process::Command::wait()`] . In case
/// of error, it returns the status code as error, so users can use ?
/// easily to return immediately.
pub fn cmd_success(cmd_result: io::Result<ExitStatus>) -> CLIResult<()> {
    let status = cmd_result?;
    match status.success() {
        true => Ok(()),
        false => match status.code() {
            Some(code) => Err(format!("Command exited with non zero error: {}", code).into()),
            None => Err("Process terminated by signal".into()),
        },
    }
}

/// Wraps the retry library and adapts it to our specific use
/// of commands. It does not pretend to decouple foreign types.
pub fn cmd_retry<I>(name: &str, interval: I, max_retries: u64, mut cmd: Command) -> CLIResult<()>
where
    I: IntoIterator<Item = Duration>,
{
    Ok(retry_with_index(interval, |current_try| {
        if current_try > max_retries {
            let message = format!(
                "Command {}: Max tries of {} reached, aborting ...",
                name, current_try
            );
            return OperationResult::Err(message.as_str().into());
        }

        let cmd = cmd.spawn();

        if let Err(err) = cmd {
            return OperationResult::Err(err.to_string().into());
        }

        match cmd_success(cmd.unwrap().wait()) {
            Ok(ok) => OperationResult::Ok(ok),
            Err(err) => OperationResult::Retry(err),
        }
    })?)
}

/// Generates a collision free /tmp folder
pub fn tmp_folder() -> CLIResult<PathBuf> {
    let uuid = Uuid::new_v4();
    let name = format!("zcash_uniffi_{}", uuid);
    let path_buff = std::env::temp_dir().join(name);
    create_dir_all(&path_buff)?;
    Ok(path_buff)
}

pub fn clean_dir(dir: &PathBuf) -> CLIResult<()> {
    _ = remove_dir_all(dir);
    Ok(create_dir_all(dir)?)
}

/// Returns the shared lib name taking into account the current platform.
pub fn shared_lib_name() -> String {
    format!("libuniffi_zcash{}", std::env::consts::DLL_SUFFIX)
}