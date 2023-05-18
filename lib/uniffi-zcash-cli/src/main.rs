use std::{
    env::set_current_dir,
    fs::{self, copy, create_dir_all, remove_dir_all, rename, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use cli::{get_matches, CLIResult};
use fs_extra::{
    dir::{self, CopyOptions},
    file::read_to_string,
};
use helper::{workspace_root_dir, in_file_template_replace};
use serde_json::json;
use strum::{Display, EnumIter, EnumString, EnumVariantNames, IntoEnumIterator};
use uuid::Uuid;

mod cli;
mod helper;

#[derive(Debug, Clone, Copy, Display, EnumString, EnumIter, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum SupportedLangs {
    #[strum(serialize = "python")]
    Python,
    #[strum(serialize = "kotlin")]
    Kotlin,
    #[strum(serialize = "swift")]
    Swift,
    #[strum(serialize = "ruby")]
    Ruby,
}

fn main() -> CLIResult<()> {
    let matches = get_matches();

    let root_dir = workspace_root_dir()?;
    set_current_dir(&root_dir)?;

    match matches.subcommand() {
        Some(("bindgen", _)) => {
            let shared_lib_path = generate_shared_lib(&root_dir)?;
            generate_bindings(&root_dir, &shared_lib_path)?;
            Ok(())
        }
        Some(("release", args)) => {
            let version = args.try_get_one::<String>("version")?.unwrap();
            let swift_repo_url = args.try_get_one::<String>("swift_repo_url")?.unwrap();
            prepare_release(&root_dir, version, swift_repo_url)?;
            Ok(())
        }
        _ => Err("Command not found. See help.".into()),
    }
}

fn prepare_release(root_dir: &Path, version: &str, swift_repo_url: &str) -> CLIResult<()> {
    let bindings_path = root_dir.join("bindings");
    if !bindings_path.exists() {
        return Err("This command depends on the output of bindgen . Execute it first.".into());
    }
    let packaging_dir = root_dir.join("packages");
    let package_template_dir = root_dir.join("uniffi-zcash-cli").join("templates");

    _ = remove_dir_all(&packaging_dir);
    create_dir_all(&packaging_dir)?;

    SupportedLangs::iter().try_for_each(|lang| match lang {
        SupportedLangs::Python => {
            dir::copy(
                package_template_dir.join(lang.to_string()),
                &packaging_dir,
                &CopyOptions::new(),
            )?;

            let lang_pack_dir = packaging_dir.join(lang.to_string());

            // Copy all needed files from previously generated bindings operation
            {
                let bindings = bindings_path.join(lang.to_string());
                copy(
                    bindings.join("libuniffi_zcash.so"),
                    lang_pack_dir.join("zcash").join("libuniffi_zcash.so"),
                )?;
                copy(
                    bindings.join("zcash.py"),
                    lang_pack_dir.join("zcash").join("zcash.py"),
                )?;
            }

            // Modify in place setup.py in order to set version in the template.
            {
                let setup_py_path = lang_pack_dir.join("setup.py");
                in_file_template_replace(setup_py_path, &json!({ "version": version }))?;
            }

            // Prepare python distribution files
            {
                std::process::Command::new("python")
                    .arg("-m")
                    .arg("pip")
                    .arg("install")
                    .arg("--user")
                    .arg("--upgrade")
                    .arg("build")
                    .spawn()?
                    .wait()?;

                std::process::Command::new("python")
                    .arg("-m")
                    .arg("build")
                    .current_dir(&lang_pack_dir)
                    .spawn()?
                    .wait()?;
            }
            // Install lib and test.
            {
                std::process::Command::new("python")
                    .arg("-m")
                    .arg("pip")
                    .arg("install")
                    .arg("--force-reinstall")
                    .arg(".")
                    .current_dir(lang_pack_dir)
                    .spawn()?
                    .wait()?;

                let test_app_path = tmp_folder()?;

                dir::copy(
                    package_template_dir.join("python_test_app"),
                    &test_app_path,
                    &CopyOptions::new().content_only(true),
                )?;

                std::process::Command::new("python")
                    .arg("app.py")
                    .current_dir(test_app_path)
                    .spawn()?
                    .wait()?;
            }
            Ok(())
        }
        SupportedLangs::Kotlin => {
            dir::copy(
                package_template_dir.join(lang.to_string()),
                &packaging_dir,
                &CopyOptions::new(),
            )?;

            let lang_pack_dir = packaging_dir.join(lang.to_string());

            // Copy all needed files from previously generated bindings operation
            {
                let bindings = bindings_path.join(lang.to_string());
                let bindings_code = bindings.join("uniffi").join("zcash");
                copy(
                    bindings_code.join("libuniffi_zcash.so"),
                    lang_pack_dir.join("lib").join("libs").join("libuniffi_zcash.so"),
                )?;
                copy(
                    bindings_code.join("zcash.kt"),
                    lang_pack_dir.join("lib")
                    .join("src")
                    .join("main")
                    .join("kotlin")
                    .join("zcash")
                    .join("Zcash.kt"),
                )?;
            }

            // Modify in place the build.gradle.kts in order to set version in the template.
            {
                let gradle_path = lang_pack_dir.join("lib").join("build.gradle.kts");
                in_file_template_replace(gradle_path, &json!({ "version": version }))?;
            }

            // Publish to local Maven, check everything is ok. Next step will exercise the dependency.
            {
                std::process::Command::new("gradle")
                    .arg("publishToMavenLocal")
                    .current_dir(&lang_pack_dir)
                    .spawn()?
                    .wait()?;
            }

            // Execute the little, built in APP test. Ensure all the build chain is ok.
            {
                let test_app_path = tmp_folder()?;

                dir::copy(
                    package_template_dir.join("kotlin_test_app"),
                    &test_app_path,
                    &CopyOptions::new().content_only(true),
                )?;

                in_file_template_replace(
                    test_app_path.join("app").join("build.gradle.kts"),
                    &json!({ "version": version }),
                )?;
                std::process::Command::new("gradle")
                    .arg("run")
                    .current_dir(test_app_path)
                    .spawn()?
                    .wait()?;
            }
            Ok(())
        }
        SupportedLangs::Swift => {
            let lang_pack_dir = packaging_dir.join(lang.to_string()).join("Zcash");
            {
                std::process::Command::new("git")
                    .arg("clone")
                    .arg(swift_repo_url)
                    .arg(&lang_pack_dir)
                    .spawn()?
                    .wait()?;
            }
            
            {
                dir::copy(
                    package_template_dir.join(lang.to_string()),
                    &lang_pack_dir,
                    &CopyOptions::new().overwrite(true).content_only(true),
                )?;
            }

            // Copy all needed files from previously generated bindings operation
            {
                let bindings = bindings_path.join(lang.to_string());
                copy(
                    bindings.join("libuniffi_zcash.so"),
                    lang_pack_dir.join("Sources").join("zcashFFI").join("libuniffi_zcash.so"),
                )?;
                copy(
                    bindings.join("zcashFFI.h"),
                    lang_pack_dir.join("Sources").join("zcashFFI").join("uniffi_zcash.h"),
                )?;
                copy(
                    bindings.join("zcash.swift"),
                    lang_pack_dir.join("Sources").join("Zcash").join("zcash.swift"),
                )?;
            }
            // Commit and tag the version
            {
                std::process::Command::new("git")
                .arg("add")
                .arg(".")
                .current_dir(&lang_pack_dir)
                .spawn()?
                .wait()?;

                std::process::Command::new("git")
                .arg("commit")
                .arg("-m")
                .arg(format!("Version {}", version))
                .current_dir(&lang_pack_dir)
                .spawn()?
                .wait()?;

                std::process::Command::new("git")
                .arg("tag")
                .arg(version)
                .current_dir(&lang_pack_dir)
                .spawn()?
                .wait()?;
            }

            // Execute the test app for testing all generated stuff.
            {
                let test_app_path = tmp_folder()?;

                dir::copy(
                    package_template_dir.join("swift_test_app"),
                    &test_app_path,
                    &CopyOptions::new().content_only(true),
                )?;

                // Use the previously generated git package for testing against.
                let data = &json!({ "version": version, "git_repo_path": &lang_pack_dir});
                in_file_template_replace(test_app_path.join("Package.swift"), data)?;

                let generated_shared_lib_path = lang_pack_dir.join("Sources").join("zcashFFI");
                std::process::Command::new("swift")
                    .current_dir(test_app_path)
                    .arg("run")
                    .arg("-Xlinker")
                    .arg(format!("-L{}", generated_shared_lib_path.as_path().to_string_lossy()))
                    .env("LD_LIBRARY_PATH", generated_shared_lib_path)
                    .spawn()?
                    .wait()?;
            }
            Ok(())
        },
        SupportedLangs::Ruby => {
            dir::copy(
                package_template_dir.join(lang.to_string()),
                &packaging_dir,
                &CopyOptions::new(),
            )?;

            let lang_pack_dir = packaging_dir.join(lang.to_string());

            // Copy all needed files from previously generated bindings operation
            {
                let bindings = bindings_path.join(lang.to_string());
                copy(
                    bindings.join("libuniffi_zcash.so"),
                    lang_pack_dir.join("lib").join("libuniffi_zcash.so"),
                )?;
                copy(
                    bindings.join("zcash.rb"),
                    lang_pack_dir.join("lib").join("zcash.rb"),
                )?;
            }

            // Modify in place the gemspec in order to set version in the template.
            {
                let gemspec_path = lang_pack_dir.join("zcash.gemspec");
                in_file_template_replace(gemspec_path, &json!({ "version": version }))?;
            }

            // Super hack 🔥. In order to be able to load shared library (.so) provided in the gem,
            // we need either to provide to the "ffi_lib" function loader (see zcash.rb) an absolute path
            // or a library name which was previously added to $LD_LIBRARY_PATH for lookup. 
            //
            // In our case we want the former option. That is normally done by using the 
            // caller file (zcash.rb) as reference, calculating the absolute path from its path.
            // But the zcash.rb file is generated by UniFFI and its out of our control.
            // So, we search and replace after the "bindgen" command generates it:
            {
                let binding_file = lang_pack_dir.join("lib").join("zcash.rb");
                let content = read_to_string(&binding_file)?;
                let result = content.replace(
                    "ffi_lib 'libuniffi_zcash.so'",
                    "ffi_lib File.join(File.dirname(File.expand_path(__FILE__)), '/libuniffi_zcash.so')",
                );
                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(binding_file)?;
                file.write_all(result.as_bytes())?;
            }

            // Prepare Ruby distribution files
            {
                std::process::Command::new("gem")
                    .arg("build")
                    .arg("zcash.gemspec")
                    .current_dir(&lang_pack_dir)
                    .spawn()?
                    .wait()?;
            }

            // Install and test
            {
                std::process::Command::new("gem")
                    .arg("install")
                    .arg(format!("./zcash-{}.gem", version))
                    .current_dir(lang_pack_dir)
                    .spawn()?
                    .wait()?;

                let test_app_path = tmp_folder()?;
                dir::copy(
                    package_template_dir.join("ruby_test_app"),
                    &test_app_path,
                    &CopyOptions::new().content_only(true),
                )?;

                std::process::Command::new("ruby")
                    .arg("app.rb")
                    .current_dir(test_app_path)
                    .spawn()?
                    .wait()?;
            }
            Ok(())
        }
    })
}

/// Generates a collision free /tmp folder
fn tmp_folder() -> CLIResult<PathBuf> {
    let uuid = Uuid::new_v4();
    let name = format!("zcash_uniffi_{}", uuid);
    let path_buff = std::env::temp_dir().join(name);
    create_dir_all(&path_buff)?;
    Ok(path_buff)
}

fn generate_shared_lib(root_dir: &Path) -> CLIResult<PathBuf> {
    println!("Generating shared library ...");
    std::process::Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(root_dir)
        .spawn()?
        .wait()?;
    Ok(root_dir
        .join("target")
        .join("release")
        .join("libuniffi_zcash.so"))
}

fn generate_bindings(root_dir: &Path, shared_lib: &Path) -> CLIResult<()> {
    // Define paths
    let udl_path = root_dir.join("uniffi-zcash").join("src").join("zcash.udl");
    let target_bindings_path = root_dir.join("bindings");

    _ = remove_dir_all(&target_bindings_path);

    println!("Generating language bindings ...");
    SupportedLangs::iter().try_for_each(|lang| {
        println!("Generating language bindings for {}", lang);
        std::process::Command::new("cargo")
            .arg("run")
            .arg("-p")
            .arg("uniffi-bindgen")
            .arg("generate")
            .arg(&udl_path)
            .arg("--config")
            .arg(root_dir.join("uniffi-bindgen").join("uniffi.toml"))
            .arg("--language")
            .arg(lang.to_string())
            .arg("--out-dir")
            .arg(target_bindings_path.join(lang.to_string()))
            .spawn()?
            .wait()?;

        let shared_lib_dest_path = target_bindings_path
            .join(lang.to_string())
            .join("libuniffi_zcash.so");

        fs::copy(shared_lib, shared_lib_dest_path)?;

        let bindings_dir = target_bindings_path.join(lang.to_string());

        // Language specific build stuff
        match lang {
            SupportedLangs::Python => Ok(()),
            SupportedLangs::Kotlin => {
                let inner_dir = bindings_dir.join("uniffi").join("zcash");
                rename(
                    bindings_dir.join("libuniffi_zcash.so"),
                    inner_dir.join("libuniffi_zcash.so"),
                )?;
                fs::copy(root_dir.join("jna.jar"), inner_dir.join("jna.jar"))?;
                Ok(())
            }
            SupportedLangs::Swift => {
                println!("Generating swift module ...");
                // We are generating this module for completion, but we are probably not going
                // to use it. See https://mozilla.github.io/uniffi-rs/swift/module.html
                std::process::Command::new("swiftc")
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
                    .wait()?;
                Ok(())
            }
            SupportedLangs::Ruby => Ok(()),
        }
    })
}
