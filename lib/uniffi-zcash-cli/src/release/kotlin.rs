use crate::release::Config;
use std::process::Command;

use fs_extra::{
    dir::{self, CopyOptions},
    file,
};

use serde_json::json;

use crate::helper::{
    clean_dir, cmd_success, in_file_template_replace, tmp_folder, LINUX_SHARED_LIB_NAME,
    MACOS_SHARED_LIB_NAME,
};

pub fn run(cfg: &Config) -> anyhow::Result<()> {
    cfg.bindings_dir.try_exists()?;
    clean_dir(&cfg.package_dir)?;

    dir::copy(
        &cfg.package_template_dir,
        &cfg.package_dir,
        &CopyOptions::new().content_only(true),
    )?;

    // Copy all needed files from previously generated bindings operation
    let bindings_code = cfg.bindings_dir.join("uniffi").join("zcash");
    let libs_dir = cfg.package_dir.join("lib").join("libs");

    file::copy(
        bindings_code.join(LINUX_SHARED_LIB_NAME),
        libs_dir.join(LINUX_SHARED_LIB_NAME),
        &file::CopyOptions::default(),
    )?;

    file::copy(
        bindings_code.join(MACOS_SHARED_LIB_NAME),
        libs_dir.join(MACOS_SHARED_LIB_NAME),
        &file::CopyOptions::default(),
    )?;

    file::copy(
        bindings_code.join("zcash.kt"),
        cfg.package_dir
            .join("lib")
            .join("src")
            .join("main")
            .join("kotlin")
            .join("zcash")
            .join("Zcash.kt"),
        &file::CopyOptions::default(),
    )?;

    // Modify in place the build.gradle.kts in order to set version in the template.
    let gradle_path = cfg.package_dir.join("lib").join("build.gradle.kts");
    in_file_template_replace(gradle_path, &json!({ "version": cfg.version }))?;

    // Publish to local Maven, check everything is ok. Next step will exercise the dependency.
    cmd_success(
        Command::new("gradle")
            .arg("publishToMavenLocal")
            .current_dir(&cfg.package_dir)
            .spawn()?
            .wait(),
    )?;

    // Execute the little, built in APP test. Ensure all the build chain is ok.
    let test_app_path = tmp_folder()?;

    dir::copy(
        &cfg.test_app_template_dir,
        &test_app_path,
        &CopyOptions::new().content_only(true),
    )?;

    in_file_template_replace(
        test_app_path.join("app").join("build.gradle.kts"),
        &json!({ "version": cfg.version }),
    )?;

    cmd_success(
        Command::new("gradle")
            .arg("run")
            .current_dir(test_app_path)
            .spawn()?
            .wait(),
    )
}
