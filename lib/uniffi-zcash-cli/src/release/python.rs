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
    clean_dir(&cfg.package_dir)?;

    dir::copy(
        &cfg.package_template_dir,
        &cfg.package_dir,
        &CopyOptions::new().content_only(true),
    )?;

    // Copy all needed files from previously generated bindings operation

    file::copy(
        cfg.bindings_dir.join(LINUX_SHARED_LIB_NAME),
        cfg.package_dir.join("zcash").join(LINUX_SHARED_LIB_NAME),
        &file::CopyOptions::default(),
    )?;
    file::copy(
        cfg.bindings_dir.join(MACOS_SHARED_LIB_NAME),
        cfg.package_dir.join("zcash").join(MACOS_SHARED_LIB_NAME),
        &file::CopyOptions::default(),
    )?;
    file::copy(
        cfg.bindings_dir.join("zcash.py"),
        cfg.package_dir.join("zcash").join("zcash.py"),
        &file::CopyOptions::default(),
    )?;

    // Modify in place setup.py in order to set version in the template.
    let setup_py_path = cfg.package_dir.join("setup.py");
    in_file_template_replace(setup_py_path, &json!({ "version": cfg.version }))?;

    // Prepare python distribution files
    cmd_success(
        Command::new("python3")
            .arg("-m")
            .arg("pip")
            .arg("install")
            .arg("--user")
            .arg("--upgrade")
            .arg("build")
            .spawn()?
            .wait(),
    )?;

    cmd_success(
        Command::new("python3")
            .arg("-m")
            .arg("build")
            .current_dir(&cfg.package_dir)
            .spawn()?
            .wait(),
    )?;

    // Install lib and test.
    cmd_success(
        Command::new("python3")
            .arg("-m")
            .arg("pip")
            .arg("install")
            .arg("--force-reinstall")
            .arg(".")
            .current_dir(&cfg.package_dir)
            .spawn()?
            .wait(),
    )?;

    let test_app_path = tmp_folder()?;

    dir::copy(
        &cfg.test_app_template_dir,
        &test_app_path,
        &CopyOptions::new().content_only(true),
    )?;

    cmd_success(
        Command::new("python3")
            .arg("app.py")
            .current_dir(test_app_path)
            .spawn()?
            .wait(),
    )
}
