use std::{fs, path::PathBuf, process::Command};

use crate::{
    cli::CLIResult,
    helper::{clean_dir, cmd_success, TARGETS},
};

pub fn add_rust_targets() -> CLIResult<()> {
    TARGETS.iter().try_for_each(|arch| {
        cmd_success(
            Command::new("rustup")
                .arg("target")
                .arg("add")
                .arg(arch)
                .spawn()?
                .wait(),
        )
    })
}

pub fn install_zig_build() -> CLIResult<()> {
    cmd_success(
        Command::new("pip3")
            .arg("install")
            .arg("ziglang")
            .spawn()?
            .wait(),
    )?;

    cmd_success(
        Command::new("cargo")
            .arg("install")
            .arg("--force")
            .arg("cargo-zigbuild")
            .spawn()?
            .wait(),
    )
}

const MACOS_SDK_VERSION: &str = "MacOSX11.3";
const MACOS_SDK_SHA256_SUM: &str =
    "cd4f08a75577145b8f05245a2975f7c81401d75e9535dcffbb879ee1deefcbf4";

pub fn install_macos_sdk() -> CLIResult<()> {
    if macos_sdk_require_path().exists() {
        println!(
            "Macos sdk already installed at {}",
            macos_sdk_require_path().to_string_lossy()
        );
        return Ok(());
    }

    let apple_sdk_install_path = macos_sdk_install_path();
    clean_dir(&apple_sdk_install_path)?;
    cmd_success(
        Command::new("wget")
            .arg(format!(
                "https://github.com/phracker/MacOSX-SDKs/releases/download/11.3/{}.sdk.tar.xz",
                MACOS_SDK_VERSION
            ))
            .current_dir(&apple_sdk_install_path)
            .spawn()?
            .wait(),
    )?;

    let tar = fs::read(macos_sdk_tar_path())?;
    let hash = sha256::digest(tar.as_slice());

    if hash.ne(MACOS_SDK_SHA256_SUM) {
        return Err(format!(
            "Hashes differ. Expected {} \n Downloaded {}",
            MACOS_SDK_SHA256_SUM, hash
        )
        .into());
    }

    cmd_success(
        Command::new("tar")
            .arg("-J")
            .arg("-xf")
            .arg("MacOSX11.3.sdk.tar.xz")
            .current_dir(&apple_sdk_install_path)
            .spawn()?
            .wait(),
    )
}

pub fn macos_sdk_install_path() -> PathBuf {
    home_dir().unwrap().join("MacOSXSDK")
}

pub fn macos_sdk_require_path() -> PathBuf {
    macos_sdk_install_path().join(format!("{}.sdk", MACOS_SDK_VERSION))
}

pub fn macos_sdk_tar_path() -> PathBuf {
    macos_sdk_install_path().join(format!("{}.sdk.tar.xz", MACOS_SDK_VERSION))
}

pub fn home_dir() -> CLIResult<PathBuf> {
    match home::home_dir() {
        Some(path) => Ok(path.join(".zcash-uniffi")),
        None => Err("Cannot calculate home dir !".into()),
    }
}
