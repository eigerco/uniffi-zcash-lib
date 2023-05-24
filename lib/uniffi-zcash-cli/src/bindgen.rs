use std::{
    fs::{self, remove_dir_all, rename},
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    cli::CLIResult, helper::cmd_success, KOTLIN, PYTHON, RUBY, SUPPORTED_LANGUAGES, SWIFT,
};

pub fn generate_shared_lib(root_dir: &Path) -> CLIResult<PathBuf> {
    println!("Generating shared library ...");
    cmd_success(
        Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(root_dir)
            .spawn()?
            .wait(),
    )?;
    Ok(root_dir
        .join("target")
        .join("release")
        .join("libuniffi_zcash.so"))
}

pub fn generate_bindings(
    root_dir: &Path,
    shared_lib: &Path,
    enabled_languages: &[String],
) -> CLIResult<()> {
    // Define paths
    let udl_path = root_dir.join("uniffi-zcash").join("src").join("zcash.udl");
    let target_bindings_path = root_dir.join("bindings");

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

            let shared_lib_dest_path = target_bindings_path
                .join(lang)
                .join("libuniffi_zcash.so");

            fs::copy(shared_lib, shared_lib_dest_path)?;

            let bindings_dir = target_bindings_path.join(lang);

            // Language specific build stuff
            match lang {
                PYTHON => Ok(()),
                KOTLIN => {
                    let inner_dir = bindings_dir.join("uniffi").join("zcash");
                    rename(
                        bindings_dir.join("libuniffi_zcash.so"),
                        inner_dir.join("libuniffi_zcash.so"),
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
                            .arg("-module-name")
                            .arg("zcash")
                            .arg("-emit-library")
                            .arg("-o")
                            .arg(bindings_dir.join("libuniffi_zcash.dylib"))
                            .arg("-emit-module")
                            .arg("-emit-module-path")
                            .arg(&bindings_dir)
                            .arg("-L")
                            .arg(root_dir.join("target").join("release"))
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
