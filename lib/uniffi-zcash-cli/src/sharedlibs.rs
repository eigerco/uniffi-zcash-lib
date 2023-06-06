use std::{fs::read, path::Path, process::Command};

use fs_extra::file::{self, CopyOptions};

use crate::{
    helper::{
        clean_dir, cmd_success, LINUX_SHARED_LIB_NAME, MACOS_SHARED_LIB_NAME, TARGET_LINUX_X86_64,
        TARGET_MACOS_64, TARGET_MACOS_UNIVERSAL2, TARGET_MACOS_X86_64,
    },
    setup::macos_sdk_require_path,
};

pub fn generate_shared_libs(root_dir: &Path, shared_libs_dir: &Path) -> anyhow::Result<()> {
    clean_dir(shared_libs_dir)?;

    println!(
        "Generating .dylib shared library for {} ...",
        TARGET_MACOS_64
    );
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

    println!(
        "Generating .dylib shared library for {} ...",
        TARGET_MACOS_X86_64
    );
    cmd_success(
        Command::new("cargo")
            .arg("zigbuild")
            .arg("--release")
            .arg("--target")
            .arg(TARGET_MACOS_X86_64)
            .env("SDKROOT", macos_sdk_require_path())
            .current_dir(root_dir)
            .spawn()?
            .wait(),
    )?;

    println!(
        "Generating .dylib shared library for {} ...",
        TARGET_MACOS_UNIVERSAL2
    );

    let mut fat_writer = fat_macho::FatWriter::new();

    fat_writer.add(read(
        root_dir
            .join("target")
            .join(TARGET_MACOS_X86_64)
            .join("release")
            .join(MACOS_SHARED_LIB_NAME),
    )?)?;

    fat_writer.add(read(
        root_dir
            .join("target")
            .join(TARGET_MACOS_64)
            .join("release")
            .join(MACOS_SHARED_LIB_NAME),
    )?)?;

    fat_writer.write_to_file(shared_libs_dir.join(MACOS_SHARED_LIB_NAME))?;

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

    file::copy(
        root_dir
            .join("target")
            .join(TARGET_LINUX_X86_64)
            .join("release")
            .join(LINUX_SHARED_LIB_NAME),
        shared_libs_dir.join(LINUX_SHARED_LIB_NAME),
        &CopyOptions::default(),
    )?;
    Ok(())
}
