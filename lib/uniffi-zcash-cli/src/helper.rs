use std::{
    fs::{read_to_string, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    process::ExitStatus,
};

use handlebars::Handlebars;
use serde::Serialize;

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
