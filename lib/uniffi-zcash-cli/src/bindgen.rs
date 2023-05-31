use std::{
    fs::{self, copy, create_dir_all, remove_dir_all, rename},
    path::Path,
    process::Command,
};

use crate::{
    cli::CLIResult,
    helper::{
        cmd_success, LINUX_SHARED_LIB_NAME, MACOS_SHARED_LIB_NAME, TARGETS, TARGET_LINUX_X86_64,
        TARGET_MACOS_64,
    },
    KOTLIN, PYTHON, RUBY, SUPPORTED_LANGUAGES, SWIFT,
};

pub fn generate_shared_lib(root_dir: &Path) -> CLIResult<()> {
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

pub fn generate_bindings(root_dir: &Path, enabled_languages: &[String]) -> CLIResult<()> {
    // Define paths
    let udl_path = root_dir.join("uniffi-zcash").join("src").join("zcash.udl");
    let target_bindings_path = root_dir.join("bindings");
    let shared_libs_dir = root_dir.join("shared_libs");

    let linux_shared_lib_path = shared_libs_dir.join(LINUX_SHARED_LIB_NAME);
    let macos_shared_lib_path = shared_libs_dir.join(MACOS_SHARED_LIB_NAME);

    _ = remove_dir_all(&target_bindings_path);

    println!("Generating language bindings ...");
    SUPPORTED_LANGUAGES
        .into_iter()
        .filter(|sl| enabled_languages.contains(&sl.to_string()))
        .try_for_each(|lang| {
            println!("Generating language bindings for {}", lang);
            cmd_success(
                Command::new("cargo")
                    .arg("run")
                    .arg("-p")
                    .arg("uniffi-bindgen")
                    .arg("generate")
                    .arg(&udl_path)
                    .arg("--config")
                    .arg(root_dir.join("uniffi-bindgen").join("uniffi.toml"))
                    .arg("--language")
                    .arg(lang)
                    .arg("--out-dir")
                    .arg(target_bindings_path.join(lang))
                    .spawn()?
                    .wait(),
            )?;

            let shared_lib_dest_path = target_bindings_path.join(lang);

            fs::copy(&linux_shared_lib_path, shared_lib_dest_path.join(LINUX_SHARED_LIB_NAME))?;
            fs::copy(&macos_shared_lib_path, shared_lib_dest_path.join(MACOS_SHARED_LIB_NAME))?;

            let bindings_dir = target_bindings_path.join(lang);

            // Language specific build stuff
            match lang {
                PYTHON => Ok(()),
                KOTLIN => {
                    let inner_dir = bindings_dir.join("uniffi").join("zcash");
                    rename(
                        bindings_dir.join(LINUX_SHARED_LIB_NAME),
                        inner_dir.join(LINUX_SHARED_LIB_NAME),
                    )?;
                    rename(
                        bindings_dir.join(MACOS_SHARED_LIB_NAME),
                        inner_dir.join(MACOS_SHARED_LIB_NAME),
                    )?;
                    fs::copy(root_dir.join("jna.jar"), inner_dir.join("jna.jar"))?;
                    Ok(())
                }
                SWIFT => {
                    println!("Generating swift module ...");
                    // We are generating this module for completion, but we are probably not going
                    // to use it. See https://mozilla.github.io/uniffi-rs/swift/module.html
                    cmd_success(
                        Command::new("swiftc")
                            .arg("-v")
                            .arg("-module-name")
                            .arg("zcash")
                            .arg("-emit-library")
                            .arg("-o")
                            .arg(bindings_dir.join("libuniffi_zcash_swift_module.dylib"))
                            .arg("-emit-module")
                            .arg("-emit-module-path")
                            .arg(&bindings_dir)
                            .arg("-L")
                            .arg(&bindings_dir)
                            .arg(format!("-l{}", "uniffi_zcash"))
                            .arg("-Xcc")
                            .arg(format!(
                                "-fmodule-map-file={}",
                                bindings_dir.join("zcashFFI.modulemap").to_string_lossy() // Should not contain no unicode chars.
                            ))
                            .arg(bindings_dir.join("zcash.swift"))
                            .spawn()?
                            .wait(),
                    )?;
                    Ok(())
                }
                RUBY => Ok(()),
                &_ => panic!("Unrecognized language (programming error). A language was added, but has no support in code !"),
            }
        })
}
