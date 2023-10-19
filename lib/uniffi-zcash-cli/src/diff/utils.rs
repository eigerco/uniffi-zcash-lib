use anyhow::anyhow;
use std::{
    env::temp_dir,
    fs,
    path::{Path, PathBuf},
};

use cargo_metadata::MetadataCommand;

pub(crate) fn get_manifest_from_package_name(
    virtual_manifest_path: String,
    lib_name: &str,
) -> anyhow::Result<String> {
    let metadata = MetadataCommand::new()
        .manifest_path(Path::new(&virtual_manifest_path))
        .no_deps()
        .exec()?;

    let package_found = metadata
        .packages
        .iter()
        .find(|package| package.name == lib_name);

    match package_found {
        Some(p) => Ok(p.manifest_path.to_string()),
        None => Err(anyhow!("failed to find crate manifest for {}", lib_name)),
    }
}

// Copy files from source to destination recursively.
pub(crate) fn copy_dir_if_not_exists(
    source: impl AsRef<Path>,
    destination: impl AsRef<Path>,
) -> anyhow::Result<()> {
    if Path::exists(destination.as_ref()) {
        return Ok(());
    }

    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_dir_if_not_exists(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

// Get a temporray path (without creating anything) for a certain direcotry name
pub(crate) fn get_temp_path_for(dir_name: &str) -> PathBuf {
    let new_temp_dir = temp_dir().join(dir_name);
    new_temp_dir
}
