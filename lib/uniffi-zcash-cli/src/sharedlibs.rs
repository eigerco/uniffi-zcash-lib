use std::{
    fs::{copy, create_dir_all},
    path::Path,
    process::Command,
};

use crate::{
    cli::CLIResult,
    helper::{
        cmd_success, LINUX_SHARED_LIB_NAME, MACOS_SHARED_LIB_NAME, TARGETS, TARGET_LINUX_X86_64,
        TARGET_MACOS_64,
    },
};

pub fn generate_shared_libs(root_dir: &Path) -> CLIResult<()> {
    let shared_libs_dir = root_dir.join("shared_libs");

    TARGETS.iter().try_for_each(|arch| {
        cmd_success(
            Command::new("rustup")
                .arg("target")
                .arg("add")
                .arg(arch)
                .spawn()?
                .wait(),
        )
    })?;

    create_dir_all(&shared_libs_dir)?;

    println!("Generating .dylib shared library for macos ...");
    cmd_success(
        Command::new("docker")
            .arg("run")
            .arg("--rm")
            .arg("-v")
            .arg(format!("{}:/io", root_dir.to_string_lossy()))
            .arg("-w")
            .arg("/io")
            .arg("messense/cargo-zigbuild:0.16.2")
            .arg("cargo")
            .arg("zigbuild")
            .arg("--release")
            .arg("--target")
            .arg(TARGET_MACOS_64)
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
            .arg("build")
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
