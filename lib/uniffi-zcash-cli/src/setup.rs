use std::{env::temp_dir, path::PathBuf, process::Command};

use crate::{
    cli::CLIResult,
    helper::{clean_dir, cmd_success, TARGETS},
};

pub fn add_rust_targets() -> CLIResult<()> {
    Ok(TARGETS.iter().try_for_each(|arch| {
        cmd_success(
            Command::new("rustup")
                .arg("target")
                .arg("add")
                .arg(arch)
                .spawn()?
                .wait(),
        )
    })?)
}

pub fn install_zig_build() -> CLIResult<()> {
    cmd_success(
        Command::new("pip")
            .arg("install")
            .arg("ziglang")
            .spawn()?
            .wait(),
    )?;

    Ok(cmd_success(
        Command::new("cargo")
            .arg("install")
            .arg("cargo-zigbuild")
            .spawn()?
            .wait(),
    )?)
}

const MACOS_SDK_VERSION: &str = "MacOSX11.3";

pub fn install_macos_sdk() -> CLIResult<()> {
    let apple_sdk_install_path = macos_sdk_install_path();
    clean_dir(&apple_sdk_install_path)?;
    cmd_success(
        Command::new("wget")
            .arg(format!("https://github.com/phracker/MacOSX-SDKs/releases/download/11.3/{}.sdk.tar.xz", MACOS_SDK_VERSION))
            .current_dir(&apple_sdk_install_path)
            .spawn()?
            .wait()
    )?;

    Ok(cmd_success(
        Command::new("tar")
            .arg("-J")
            .arg("-xf")
            .arg("MacOSX11.3.sdk.tar.xz")
            .current_dir(&apple_sdk_install_path)
            .spawn()?
            .wait(),
    )?)
}

pub fn macos_sdk_install_path() -> PathBuf {
    temp_dir().join("MacOSXSDK")
}

pub fn macos_sdk_require_path() -> PathBuf {
    macos_sdk_install_path().join(format!("{}.sdk", MACOS_SDK_VERSION))
}
