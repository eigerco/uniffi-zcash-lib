use std::{
    path::Path,
    process::{Command, ExitStatus},
};

use fs_extra::file::{self, CopyOptions};

use crate::{
    helper::{cmd_success, LINUX_SHARED_LIB_NAME, MACOS_SHARED_LIB_NAME},
    KOTLIN, PYTHON, RUBY, SUPPORTED_LANGUAGES, SWIFT,
};

pub fn generate_bindings(root_dir: &Path, enabled_languages: &[String]) -> anyhow::Result<()> {
    // eliminate directory if it exists already
    fs_extra::dir::remove(root_dir.join("bindings"))?;

    println!("Generating bindings ...");
    SUPPORTED_LANGUAGES
        .into_iter()
        .filter(|sl| enabled_languages.contains(&sl.to_string()))
        .try_for_each(|lang| {
            println!("Generating language bindings for {}", lang);

            let lang_binding_path = root_dir.join("bindings").join(lang);

            let command = generate_binding(root_dir, lang, &lang_binding_path);

            cmd_success(command)?;

            copy_bindings(root_dir, &lang_binding_path)?;

            match lang {
                KOTLIN => kotlin_binding_generation(root_dir, &lang_binding_path),
                SWIFT => swift_binding_generation(&lang_binding_path),
                RUBY | PYTHON => Ok(()),
                &_ => panic!("Unrecognized language (programming error). A language was added to supported list, but has no support in code !"),
            }
        })
}

fn generate_binding(
    root_dir: &Path,
    lang: &str,
    lang_binding_path: &Path,
) -> Result<ExitStatus, std::io::Error> {
    // let config_path = root_dir.join("uniffi-zcash").join("uniffi.toml");

    // #[cfg(target_os = "macos")]
    // let releases_path = root_dir.join("shared_libs").join("libuniffi_zcash.dylib");

    // #[cfg(not(target_os = "macos"))]
    // The static library should be OK for all OSs
    let releases_path = root_dir.join("shared_libs").join("libuniffi_zcash.so");

    Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("uniffi-bindgen")
        .arg("generate")
        .arg("--library")
        .arg(releases_path)
        // .arg("--config")
        // .arg(&config_path)
        .arg("--language")
        .arg(lang)
        .arg("--out-dir")
        .arg(lang_binding_path)
        .spawn()?
        .wait()
}

fn copy_bindings(root_dir: &Path, shared_lib_dest_path: &Path) -> Result<(), anyhow::Error> {
    println!("Copying bindings to proper folder...");

    let shared_libs_dir = root_dir.join("shared_libs");

    let linux_shared_lib_path = shared_libs_dir.join(LINUX_SHARED_LIB_NAME);
    file::copy(
        linux_shared_lib_path,
        shared_lib_dest_path.join(LINUX_SHARED_LIB_NAME),
        &CopyOptions::default(),
    )?;

    let macos_shared_lib_path = shared_libs_dir.join(MACOS_SHARED_LIB_NAME);
    file::copy(
        macos_shared_lib_path,
        shared_lib_dest_path.join(MACOS_SHARED_LIB_NAME),
        &CopyOptions::default(),
    )?;

    Ok(())
}

// We are generating this module for the sake of a complete implementation, but we are probably not going
// to use it. See https://mozilla.github.io/uniffi-rs/swift/module.html
fn swift_binding_generation(bindings_dir: &Path) -> Result<(), anyhow::Error> {
    println!("Generating swift module ...");

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
            .arg(bindings_dir)
            .arg("-L")
            .arg(bindings_dir)
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

fn kotlin_binding_generation(root_dir: &Path, bindings_dir: &Path) -> Result<(), anyhow::Error> {
    let inner_dir = bindings_dir.join("uniffi").join("zcash");

    file::move_file(
        bindings_dir.join(LINUX_SHARED_LIB_NAME),
        inner_dir.join(LINUX_SHARED_LIB_NAME),
        &CopyOptions::default(),
    )?;

    file::move_file(
        bindings_dir.join(MACOS_SHARED_LIB_NAME),
        inner_dir.join(MACOS_SHARED_LIB_NAME),
        &CopyOptions::default(),
    )?;

    // Copying over java native library
    file::copy(
        root_dir.join("jna.jar"),
        inner_dir.join("jna.jar"),
        &CopyOptions::default(),
    )?;

    Ok(())
}
