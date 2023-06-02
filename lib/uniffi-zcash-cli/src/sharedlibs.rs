use std::{fs::copy, path::Path, process::Command};

use crate::{
    cli::CLIResult,
    helper::{
        clean_dir, cmd_success, LINUX_SHARED_LIB_NAME, MACOS_SHARED_LIB_NAME, TARGET_LINUX_X86_64,
        TARGET_MACOS_64,
    },
    setup::macos_sdk_require_path,
};

pub fn generate_shared_libs(root_dir: &Path) -> CLIResult<()> {
    let shared_libs_dir = root_dir.join("shared_libs");

    clean_dir(&shared_libs_dir)?;

    println!("Generating .dylib shared library for macos ...");
    cmd_success(
        Command::new("cargo")
            .arg("zigbuild")
            .arg("--release")
            .arg("--target")
            .arg(TARGET_MACOS_64)
            .env("SDKROOT", macos_sdk_require_path())
            .current_dir(root_dir)
            .spawn()?
            .wait(),
    )?;

    copy(
        root_dir
            .join("target")
            .join(TARGET_MACOS_64)
            .join("release")
            .join(MACOS_SHARED_LIB_NAME),
        shared_libs_dir.join(MACOS_SHARED_LIB_NAME),
    )?;

    println!("Generating .so shared library for linux ...");
    cmd_success(
        Command::new("cargo")
            .arg("zigbuild")
            .arg("--release")
            .arg("--target")
            .arg(TARGET_LINUX_X86_64)
            .current_dir(root_dir)
            .spawn()?
            .wait(),
    )?;

    copy(
        root_dir
            .join("target")
            .join(TARGET_LINUX_X86_64)
            .join("release")
            .join(LINUX_SHARED_LIB_NAME),
        shared_libs_dir.join(LINUX_SHARED_LIB_NAME),
    )?;
    Ok(())
}
