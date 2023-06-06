use std::{
    path::Path,
    process::Command,
};

use fs_extra::{file::{self, CopyOptions}, dir};

use crate::{
    helper::{cmd_success, LINUX_SHARED_LIB_NAME, MACOS_SHARED_LIB_NAME},
    KOTLIN, PYTHON, RUBY, SUPPORTED_LANGUAGES, SWIFT,
};

pub fn generate_bindings(root_dir: &Path, enabled_languages: &[String]) -> anyhow::Result<()> {
    // Define paths
    let udl_path = root_dir.join("uniffi-zcash").join("src").join("zcash.udl");
    let target_bindings_path = root_dir.join("bindings");
    let shared_libs_dir = root_dir.join("shared_libs");

    let linux_shared_lib_path = shared_libs_dir.join(LINUX_SHARED_LIB_NAME);
    let macos_shared_lib_path = shared_libs_dir.join(MACOS_SHARED_LIB_NAME);

    dir::remove(&target_bindings_path)?;

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
