use std::{path::Path, process::Command};

use fs_extra::{
    // dir,
    file::{self, CopyOptions},
};

use crate::{
    helper::{cmd_success, LINUX_SHARED_LIB_NAME, MACOS_SHARED_LIB_NAME},
    KOTLIN, PYTHON, RUBY, SUPPORTED_LANGUAGES, SWIFT,
};

pub fn generate_bindings(root_dir: &Path, enabled_languages: &[String]) -> anyhow::Result<()> {
    // Define paths
    // let config_path = root_dir.join("uniffi-zcash").join("uniffi.toml");
    let releases_path = root_dir.join("target").join("release").join("libuniffi_zcash.dylib");
    let target_bindings_path = root_dir.join("bindings");
    let shared_libs_dir = root_dir.join("shared_libs");

    let linux_shared_lib_path = shared_libs_dir.join(LINUX_SHARED_LIB_NAME);
    let macos_shared_lib_path = shared_libs_dir.join(MACOS_SHARED_LIB_NAME);

    // NOTE eliminate if it exists
    // dir::remove(&target_bindings_path)?;

    println!("Generating language bindings ...");
    SUPPORTED_LANGUAGES
        .into_iter()
        .filter(|sl| enabled_languages.contains(&sl.to_string()))
        .try_for_each(|lang| {
            println!("Generating language bindings for {}", lang);

            let command = Command::new("cargo")
                    .arg("run")
                    .arg("--bin")
                    .arg("uniffi-bindgen")
                    .arg("generate")
                    .arg("--library")
                    .arg(&releases_path)
                    // .arg("--config")
                    // .arg(&config_path)
                    .arg("--language")
                    .arg(lang)
                    .arg("--out-dir")
                    .arg(target_bindings_path.join(lang))
                    .spawn()?
                    .wait();

            cmd_success(command)?;

            let shared_lib_dest_path = target_bindings_path.join(lang);

            file::copy(&linux_shared_lib_path, shared_lib_dest_path.join(LINUX_SHARED_LIB_NAME), &CopyOptions::default())?;
            file::copy(&macos_shared_lib_path, shared_lib_dest_path.join(MACOS_SHARED_LIB_NAME), &CopyOptions::default())?;

            let bindings_dir = target_bindings_path.join(lang);

            // Language specific build stuff
            match lang {
                PYTHON => Ok(()),
                KOTLIN => {
                    let inner_dir = bindings_dir.join("uniffi").join("zcash");
                    file::move_file(
                        bindings_dir.join(LINUX_SHARED_LIB_NAME),
                        inner_dir.join(LINUX_SHARED_LIB_NAME),
                        &CopyOptions::default()
                    )?;
                    file::move_file(
                        bindings_dir.join(MACOS_SHARED_LIB_NAME),
                        inner_dir.join(MACOS_SHARED_LIB_NAME),
                        &CopyOptions::default()
                    )?;
                    file::copy(root_dir.join("jna.jar"), inner_dir.join("jna.jar"), &CopyOptions::default())?;
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
                &_ => panic!("Unrecognized language (programming error). A language was added to supported list, but has no support in code !"),
            }
        })
}
