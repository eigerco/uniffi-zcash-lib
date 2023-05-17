use std::{
    env::set_current_dir,
    error::Error,
    fmt::Display,
    fs::{self, copy, create_dir_all, remove_dir_all, rename, OpenOptions},
    io::Write,
    path::{self, Path, PathBuf},
};

use clap::{parser::MatchesError, Arg, Command};

use fs_extra::{
    dir::{self, CopyOptions},
    file::read_to_string,
};
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use strum::{Display, EnumIter, EnumString, EnumVariantNames, IntoEnumIterator, VariantNames};
use uuid::Uuid;

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
    let matches = Command::new("UniFFI Zcash CLI")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A CLI for managing internal repo workflows")
        .subcommand_required(true)
        .subcommand(Command::new("bindgen").about(format!(
            "Generates UniFFI bindings for all the supported languages ({}) and places it in the bindings directory",
            SupportedLangs::VARIANTS.join(",")
        )))
        .subcommand(Command::new("release").about(format!(
            "Prepares a release given a version (semantic versioning), creating all languages ({}) specific packages. It needs to be executed after the bindgen command",
            SupportedLangs::VARIANTS.join(",")
        ))
        .arg(Arg::new("version").required(true)))
        .get_matches();

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
            prepare_release(&root_dir, version)?;
            Ok(())
        }
        _ => Err("Command not found. See help.".into()),
    }
}

fn prepare_release(root_dir: &Path, version: &str) -> CLIResult<()> {
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
                    .wait_with_output()?;

                std::process::Command::new("python")
                    .arg("-m")
                    .arg("build")
                    .current_dir(&lang_pack_dir)
                    .spawn()?
                    .wait_with_output()?;
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
                    .wait_with_output()?;

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
                    .wait_with_output()?;
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
                    .wait_with_output()?;
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
                    .wait_with_output()?;
            }
            Ok(())
        }
        SupportedLangs::Swift => Ok(()),
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
                    .wait_with_output()?;
            }

            // Install and test
            {
                std::process::Command::new("gem")
                    .arg("install")
                    .arg(format!("./zcash-{}.gem", version))
                    .current_dir(lang_pack_dir)
                    .spawn()?
                    .wait_with_output()?;

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
                    .wait_with_output()?;
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

/// Overwrites the provided file by rendering the provided data on it.
fn in_file_template_replace<P, T>(file_path: P, data: &T) -> CLIResult<()>
where
    P: AsRef<Path>,
    T: Serialize,
{
    let content = read_to_string(&file_path)?;
    let reg = Handlebars::new();
    let rendered = reg.render_template(&content, data)?;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file_path)?;
    file.write_all(rendered.as_bytes())?;
    Ok(())
}

fn workspace_root_dir() -> CLIResult<PathBuf> {
    let err_msg = "Cannot find parent path.";
    Ok(std::env::current_exe()?
        .parent()
        .ok_or(err_msg)?
        .parent()
        .ok_or(err_msg)?
        .parent()
        .ok_or(err_msg)?
        .to_owned())
}

fn generate_shared_lib(root_dir: &Path) -> CLIResult<PathBuf> {
    println!("Generating shared library ...");
    std::process::Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(root_dir)
        .spawn()?
        .wait_with_output()?;
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
            .wait_with_output()?;

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
                    .wait_with_output()?;
                Ok(())
            }
            SupportedLangs::Ruby => Ok(()),
        }
    })
}

#[derive(Debug)]
struct CLIError {
    message: String,
}

impl Error for CLIError {}

impl Display for CLIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl From<&str> for CLIError {
    fn from(value: &str) -> Self {
        CLIError {
            message: value.to_string(),
        }
    }
}

impl From<std::io::Error> for CLIError {
    fn from(value: std::io::Error) -> Self {
        CLIError {
            message: value.to_string(),
        }
    }
}

impl From<MatchesError> for CLIError {
    fn from(value: MatchesError) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<fs_extra::error::Error> for CLIError {
    fn from(value: fs_extra::error::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<handlebars::RenderError> for CLIError {
    fn from(value: handlebars::RenderError) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

type CLIResult<T> = Result<T, CLIError>;
