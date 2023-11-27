use crate::release::Config;
use std::{env, fs::OpenOptions, io::Write, process::Command};

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

    // Generate a /tmp subfolder , so git does not have problems git the parent
    // project repository. From here all operations will be done in that folder.
    let tmp_package_dir = env::temp_dir().join("zcash_uniffi_swift_package_build");
    clean_dir(&tmp_package_dir)?;

    // We will leave a pointer (a text file) to properly signal that we are operating
    // outside the working tree, by adding the absolute path to the temporary subfolder.
    let mut pointer = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(cfg.package_dir.join("processing_at.txt"))?;

    pointer.write_all(tmp_package_dir.to_str().unwrap().as_bytes())?;

    let package_subfolder = tmp_package_dir.join("Zcash");

    dir::create_all(&package_subfolder, false)?;

    let copied_repo_url = cfg.git_repo_url.clone().unwrap();

    println!("git_repo_url: {}", copied_repo_url);

    cmd_success(
        Command::new("git")
            .arg("clone")
            .arg(&copied_repo_url)
            .arg(&package_subfolder)
            .spawn()?
            .wait(),
    )?;

    dir::copy(
        &cfg.package_template_dir,
        &package_subfolder,
        &CopyOptions::new().overwrite(true).content_only(true),
    )?;

    let generated_shared_lib_path = package_subfolder.join("Sources").join("zcashFFI");

    // Copy all needed files from previously generated bindings operation

    file::copy(
        cfg.bindings_dir.join(LINUX_SHARED_LIB_NAME),
        generated_shared_lib_path.join(LINUX_SHARED_LIB_NAME),
        &file::CopyOptions::default(),
    )?;

    file::copy(
        cfg.bindings_dir.join(MACOS_SHARED_LIB_NAME),
        generated_shared_lib_path.join(MACOS_SHARED_LIB_NAME),
        &file::CopyOptions::default(),
    )?;

    file::copy(
        cfg.bindings_dir.join("zcashFFI.h"),
        generated_shared_lib_path.join("uniffi_zcash.h"),
        &file::CopyOptions::default(),
    )?;

    file::copy(
        cfg.bindings_dir.join("zcash.swift"),
        package_subfolder
            .join("Sources")
            .join("Zcash")
            .join("zcash.swift"),
        &file::CopyOptions::default(),
    )?;

    env::set_current_dir(&package_subfolder)?;

    // Commit and tag the version
    cmd_success(Command::new("git").arg("add").arg(".").spawn()?.wait())?;

    cmd_success(
        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(format!("Version {}", &cfg.version))
            .spawn()?
            .wait(),
    )?;

    cmd_success(
        Command::new("git")
            .arg("tag")
            .arg(&cfg.version)
            .spawn()?
            .wait(),
    )?;

    // Execute the test app for testing all generated stuff.
    let test_app_path = tmp_folder()?;

    // Also, copy the shared lib to the root of testing app,
    // so it can be located by search rules.
    // This is needed, as the MacOS integrity protection
    // wipes out all the DYLD_* env vars.
    // See https://developer.apple.com/library/prerelease/mac/documentation/Security/Conceptual/System_Integrity_Protection_Guide/RuntimeProtections/RuntimeProtections.html

    file::copy(
        cfg.bindings_dir.join(MACOS_SHARED_LIB_NAME),
        test_app_path.join(MACOS_SHARED_LIB_NAME),
        &file::CopyOptions::default(),
    )?;

    dir::copy(
        &cfg.test_app_template_dir,
        &test_app_path,
        &CopyOptions::new().content_only(true),
    )?;

    // Use the previously generated git package for testing against.

    let data = &json!({ "version": cfg.version, "git_repo_path": &package_subfolder});
    in_file_template_replace(test_app_path.join("Package.swift"), data)?;

    let linked_lib_path = generated_shared_lib_path.as_path().to_string_lossy();

    cmd_success(
        Command::new("swift")
            .current_dir(test_app_path)
            .arg("run")
            .arg("-Xlinker")
            .arg(format!("-L{linked_lib_path}"))
            .env("LD_LIBRARY_PATH", generated_shared_lib_path)
            .spawn()?
            .wait(),
    )
}
