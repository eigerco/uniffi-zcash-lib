use std::{
    env::set_current_dir,
    fs::{self, remove_dir_all, rename},
    path::{Path, PathBuf},
    process::Command,
};

use cli::{get_matches, CLIResult};

use helper::{cmd_success, workspace_root_dir};

use strum::{Display, EnumIter, EnumString, EnumVariantNames, IntoEnumIterator};

mod cli;
mod helper;
mod publish;
mod release;

#[derive(Debug, Clone, Copy, Display, EnumString, EnumIter, EnumVariantNames, PartialEq)]
#[strum(serialize_all = "kebab_case")]
enum SupportedLang {
    #[strum(serialize = "python")]
    Python,
    #[strum(serialize = "kotlin")]
    Kotlin,
    #[strum(serialize = "swift")]
    Swift,
    #[strum(serialize = "ruby")]
    Ruby,
}

impl From<&SupportedLang> for SupportedLang {
    fn from(value: &SupportedLang) -> Self {
        value.to_owned()
    }
}

fn main() -> CLIResult<()> {
    let matches = get_matches();

    let root_dir = workspace_root_dir()?;
    let bindings_dir = root_dir.join("bindings");
    let packages_dir = root_dir.join("packages");

    set_current_dir(&root_dir)?;

    let enabled_languages = matches
        .try_get_many::<String>("enabled_languages")?
        .unwrap()
        .map(Clone::clone)
        .collect();

    match matches.subcommand() {
        Some(("bindgen", _)) => {
            let shared_lib_path = generate_shared_lib(&root_dir)?;
            generate_bindings(&root_dir, &shared_lib_path, &enabled_languages)?;
            Ok(())
        }
        Some(("release", args)) => {
            let package_template_dir = root_dir.join("uniffi-zcash-cli").join("templates");
            match args.subcommand() {
                Some(("python", args)) => {
                    let lang = "python";
                    let cfg = release::PythonConfig {
                        version: args.try_get_one::<String>("version")?.unwrap().to_owned(),
                        package_template_dir: package_template_dir.join(lang),
                        test_app_template_dir: package_template_dir.join("python_test_app"),
                        bindings_dir: bindings_dir.join(lang),
                        package_dir: packages_dir.join(lang),
                    };
                    Ok(release::python(&cfg)?)
                }
                Some(("ruby", args)) => {
                    let lang = "ruby";
                    let cfg = release::RubyConfig {
                        version: args.try_get_one::<String>("version")?.unwrap().to_owned(),
                        package_template_dir: package_template_dir.join(lang),
                        test_app_template_dir: package_template_dir.join("ruby_test_app"),
                        bindings_dir: bindings_dir.join(lang),
                        package_dir: packages_dir.join(lang),
                    };
                    Ok(release::ruby(&cfg)?)
                }
                Some(("kotlin", args)) => {
                    let lang = "kotlin";
                    let cfg = release::KotlinConfig {
                        version: args.try_get_one::<String>("version")?.unwrap().to_owned(),
                        package_template_dir: package_template_dir.join(lang),
                        test_app_template_dir: package_template_dir.join("kotlin_test_app"),
                        bindings_dir: bindings_dir.join(lang),
                        package_dir: packages_dir.join(lang),
                    };
                    Ok(release::kotlin(&cfg)?)
                }
                Some(("swift", args)) => {
                    let lang = "swift";
                    let cfg = release::SwiftConfig {
                        version: args.try_get_one::<String>("version")?.unwrap().to_owned(),
                        git_repo_url: args
                            .try_get_one::<String>("git_repo_url")?
                            .unwrap()
                            .to_owned(),
                            package_template_dir: package_template_dir.join(lang),
                            test_app_template_dir: package_template_dir.join("swift_test_app"),
                            bindings_dir: bindings_dir.join(lang),
                            package_dir: packages_dir.join(lang),
                    };
                    Ok(release::swift(&cfg)?)
                }
                _ => Err("Command not found. See help.".into()),
            }
        }
        Some(("publish", args)) => match args.subcommand() {
            Some(("python", args)) => {
                let cfg = publish::PythonConfig {
                    lang_package_path: packages_dir.join("python"),
                    registry_url: args
                        .try_get_one::<String>("registry_url")?
                        .unwrap()
                        .to_owned(),
                    registry_username: args
                        .try_get_one::<String>("registry_username")?
                        .unwrap()
                        .to_owned(),
                    registry_password: args
                        .try_get_one::<String>("registry_password")?
                        .unwrap()
                        .to_owned(),
                };
                cfg.lang_package_path.try_exists()?;
                Ok(publish::python(&cfg)?)
            }
            Some(("ruby", args)) => {
                let cfg = publish::RubyConfig {
                    lang_package_path: packages_dir.join("ruby"),
                    version: args.try_get_one::<String>("version")?.unwrap().to_owned(),
                    registry_url: args
                        .try_get_one::<String>("registry_url")?
                        .unwrap()
                        .to_owned(),
                    registry_token: args
                        .try_get_one::<String>("registry_token")?
                        .unwrap()
                        .to_owned(),
                };
                cfg.lang_package_path.try_exists()?;
                Ok(publish::ruby(&cfg)?)
            }
            Some(("kotlin", args)) => {
                let cfg = publish::KotlinConfig {
                    lang_package_path: packages_dir.join("kotlin"),
                    registry_url: args
                        .try_get_one::<String>("registry_url")?
                        .unwrap()
                        .to_owned(),
                    registry_username: args
                        .try_get_one::<String>("registry_username")?
                        .unwrap()
                        .to_owned(),
                    registry_password: args
                        .try_get_one::<String>("registry_password")?
                        .unwrap()
                        .to_owned(),
                };
                cfg.lang_package_path.try_exists()?;
                Ok(publish::kotlin(&cfg)?)
            }
            Some(("swift", args)) => match args.subcommand() {
                Some(("git-repo", args)) => {
                    let cfg = publish::SwiftRepoConfig {
                        lang_package_path: packages_dir.join("swift").join("Zcash"),
                        git_repo_url: args
                            .try_get_one::<String>("git_repo_url")?
                            .unwrap()
                            .to_owned(),
                    };
                    cfg.lang_package_path.try_exists()?;
                    Ok(publish::swift_repo(&cfg)?)
                }
                Some(("registry", args)) => {
                    let cfg = publish::SwiftRegistryConfig {
                        lang_package_path: packages_dir.join("swift").join("Zcash"),
                        version: args.try_get_one::<String>("version")?.unwrap().to_owned(),
                        registry_url: args
                            .try_get_one::<String>("registry_url")?
                            .unwrap()
                            .to_owned(),
                        registry_token: args
                            .try_get_one::<String>("registry_token")?
                            .unwrap()
                            .to_owned(),
                    };
                    cfg.lang_package_path.try_exists()?;
                    Ok(publish::swift_registry(&cfg)?)
                }
                _ => Err("Command not found. See help.".into()),
            },
            _ => Err("Command not found. See help.".into()),
        },
        _ => Err("Command not found. See help.".into()),
    }
}

fn generate_shared_lib(root_dir: &Path) -> CLIResult<PathBuf> {
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

fn generate_bindings(
    root_dir: &Path,
    shared_lib: &Path,
    enabled_languages: &Vec<String>,
) -> CLIResult<()> {
    // Define paths
    let udl_path = root_dir.join("uniffi-zcash").join("src").join("zcash.udl");
    let target_bindings_path = root_dir.join("bindings");

    _ = remove_dir_all(&target_bindings_path);

    println!("Generating language bindings ...");
    SupportedLang::iter()
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
                    .arg(lang.to_string())
                    .arg("--out-dir")
                    .arg(target_bindings_path.join(lang.to_string()))
                    .spawn()?
                    .wait(),
            )?;

            let shared_lib_dest_path = target_bindings_path
                .join(lang.to_string())
                .join("libuniffi_zcash.so");

            fs::copy(shared_lib, shared_lib_dest_path)?;

            let bindings_dir = target_bindings_path.join(lang.to_string());

            // Language specific build stuff
            match lang {
                SupportedLang::Python => Ok(()),
                SupportedLang::Kotlin => {
                    let inner_dir = bindings_dir.join("uniffi").join("zcash");
                    rename(
                        bindings_dir.join("libuniffi_zcash.so"),
                        inner_dir.join("libuniffi_zcash.so"),
                    )?;
                    fs::copy(root_dir.join("jna.jar"), inner_dir.join("jna.jar"))?;
                    Ok(())
                }
                SupportedLang::Swift => {
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
                SupportedLang::Ruby => Ok(()),
            }
        })
}
