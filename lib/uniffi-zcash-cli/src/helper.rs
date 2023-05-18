use std::{
    fs::{read_to_string, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
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
