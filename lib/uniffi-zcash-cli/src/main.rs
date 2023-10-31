use std::env::set_current_dir;

use bindgen::generate_bindings;
use clap::ColorChoice;
use cli::get_matches;

use anyhow::anyhow;
use diff::generate_diff;
use helper::{workspace_root_dir, PathChecker};
use setup::{add_rust_targets, install_dokka_cli, install_macos_sdk, install_zig_build};
use sharedlibs::generate_shared_libs;
use uniffi_zcash_test::test_data::generate_test_data;
use zcash_proofs::download_sapling_parameters;

mod bindgen;
mod cli;
mod diff;
mod docgen;
mod helper;
mod publish;
mod release;
mod setup;
mod sharedlibs;

const PYTHON: &str = "python";
const RUBY: &str = "ruby";
const KOTLIN: &str = "kotlin";
const SWIFT: &str = "swift";

const SUPPORTED_LANGUAGES: [&str; 4] = [PYTHON, RUBY, KOTLIN, SWIFT];

fn main() -> anyhow::Result<()> {
    let matches = get_matches();

    let root_dir = workspace_root_dir()?;
    let shared_libs_dir = root_dir.join("shared_libs");
    let bindings_dir = root_dir.join("bindings");
    let packages_dir = root_dir.join("packages");
    let docs_dir = root_dir.join("docs");

    set_current_dir(&root_dir)?;

    match matches.subcommand() {
        Some(("setup", args)) => match args.subcommand() {
            Some(("buildenv", _)) => {
                add_rust_targets()?;
                install_zig_build()?;
                Ok(install_macos_sdk()?)
            }
            Some(("builddoc", _)) => Ok(install_dokka_cli()?),
            Some(("saplingparams", _)) => match download_sapling_parameters(None) {
                Ok(paths) => {
                    println!(
                        "Downloaded spend parameters at : {}",
                        paths.spend.to_string_lossy()
                    );
                    println!(
                        "Downloaded output parameters at : {}",
                        paths.output.to_string_lossy()
                    );
                    Ok(())
                }
                Err(err) => Err(anyhow!(err.to_string())),
            },
            Some(("testdata", _)) => {
                generate_test_data(true);
                Ok(())
            }
            _ => Err(anyhow!("Command not found. See help.")),
        },

        Some(("sharedlibs", _)) => Ok(generate_shared_libs(&root_dir, &shared_libs_dir)?),
        Some(("bindgen", args)) => {
            shared_libs_dir
                .informed_exists("Are the shared libs already built ? Check CLI help.")?;

            let languages: Vec<String> = args
                .try_get_many::<String>("languages")?
                .unwrap()
                .map(Clone::clone)
                .collect();
            Ok(generate_bindings(&root_dir, &languages)?)
        }
        Some(("release", args)) => {
            bindings_dir
                .informed_exists("Are the language bindings already built ? Check CLI help.")?;
            let package_template_dir = root_dir.join("uniffi-zcash-cli").join("templates");
            match args.subcommand() {
                Some((PYTHON, args)) => {
                    let cfg = release::PythonConfig {
                        version: args.try_get_one::<String>("version")?.unwrap().to_owned(),
                        package_template_dir: package_template_dir.join(PYTHON),
                        test_app_template_dir: package_template_dir.join("python_test_app"),
                        bindings_dir: bindings_dir.join(PYTHON),
                        package_dir: packages_dir.join(PYTHON),
                    };
                    Ok(release::python(&cfg)?)
                }
                Some((RUBY, args)) => {
                    let cfg = release::RubyConfig {
                        version: args.try_get_one::<String>("version")?.unwrap().to_owned(),
                        package_template_dir: package_template_dir.join(RUBY),
                        test_app_template_dir: package_template_dir.join("ruby_test_app"),
                        bindings_dir: bindings_dir.join(RUBY),
                        package_dir: packages_dir.join(RUBY),
                    };
                    Ok(release::ruby(&cfg)?)
                }
                Some((KOTLIN, args)) => {
                    let cfg = release::KotlinConfig {
                        version: args.try_get_one::<String>("version")?.unwrap().to_owned(),
                        package_template_dir: package_template_dir.join(KOTLIN),
                        test_app_template_dir: package_template_dir.join("kotlin_test_app"),
                        bindings_dir: bindings_dir.join(KOTLIN),
                        package_dir: packages_dir.join(KOTLIN),
                    };
                    Ok(release::kotlin(&cfg)?)
                }
                Some((SWIFT, args)) => {
                    let cfg = release::SwiftConfig {
                        version: args.try_get_one::<String>("version")?.unwrap().to_owned(),
                        git_repo_url: args
                            .try_get_one::<String>("git_repo_url")?
                            .unwrap()
                            .to_owned(),
                        package_template_dir: package_template_dir.join(SWIFT),
                        test_app_template_dir: package_template_dir.join("swift_test_app"),
                        bindings_dir: bindings_dir.join(SWIFT),
                        package_dir: packages_dir.join(SWIFT),
                    };
                    Ok(release::swift(&cfg)?)
                }
                _ => Err(anyhow!("Command not found. See help.")),
            }
        }
        Some(("publish", args)) => {
            packages_dir
                .informed_exists("Are the language packages already built ? Check CLI help.")?;

            match args.subcommand() {
                Some((PYTHON, args)) => {
                    let cfg = publish::PythonConfig {
                        lang_package_path: packages_dir.join(PYTHON),
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
                Some((RUBY, args)) => {
                    let cfg = publish::RubyConfig {
                        lang_package_path: packages_dir.join(RUBY),
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
                Some((KOTLIN, args)) => {
                    let cfg = publish::KotlinConfig {
                        lang_package_path: packages_dir.join(KOTLIN),
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
                Some((SWIFT, args)) => match args.subcommand() {
                    Some(("git-repo", args)) => {
                        let cfg = publish::SwiftRepoConfig {
                            lang_package_path: packages_dir.join(SWIFT),
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
                            lang_package_path: packages_dir.join(SWIFT),
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
                    _ => Err(anyhow!("Command not found. See help.")),
                },
                _ => Err(anyhow!("Command not found. See help.")),
            }
        }
        Some(("docgen", args)) => {
            packages_dir
                .informed_exists("Are the language packages already built ? Check CLI help.")?;

            match args.subcommand() {
                Some((PYTHON, args)) => {
                    let version = args.try_get_one::<String>("version")?.unwrap().to_owned();
                    Ok(docgen::python(
                        &packages_dir.join(PYTHON),
                        &docs_dir.join(PYTHON).join(version),
                    )?)
                }
                Some((RUBY, args)) => {
                    let version = args.try_get_one::<String>("version")?.unwrap().to_owned();
                    Ok(docgen::ruby(
                        &packages_dir.join(RUBY),
                        &docs_dir.join(RUBY).join(version),
                    )?)
                }
                Some((KOTLIN, args)) => {
                    let version = args.try_get_one::<String>("version")?.unwrap().to_owned();
                    Ok(docgen::kotlin(
                        &packages_dir.join(KOTLIN),
                        &docs_dir.join(KOTLIN).join(version),
                    )?)
                }
                Some((SWIFT, args)) => {
                    let version = args.try_get_one::<String>("version")?.unwrap().to_owned();
                    Ok(docgen::swift(
                        &packages_dir.join(SWIFT),
                        &docs_dir.join(SWIFT).join(version),
                    )?)
                }
                _ => Err(anyhow!("Command not found. See help.")),
            }
        }
        Some(("diff", args)) => {
            let lib_name = args.try_get_one::<String>("lib_name")?.unwrap();
            let color = args
                .try_get_one::<ColorChoice>("color")?
                .unwrap_or(&ColorChoice::Never)
                .to_owned();
            let lib_old_version = args.try_get_one::<String>("lib_old_version")?.unwrap();
            let lib_new_version = args.try_get_one::<String>("lib_new_version")?.unwrap();
            let grep_dir = args.try_get_one::<String>("grep_dir")?.unwrap();

            let path_default = String::from("");
            let librustzcash_path = args
                .try_get_one::<String>("librustzcash_path")?
                .unwrap_or(&path_default);

            generate_diff(
                lib_name,
                lib_new_version,
                lib_old_version,
                grep_dir,
                color,
                librustzcash_path,
            )?;

            Ok(())
        }
        _ => Err(anyhow!("Command not found. See help.")),
    }
}
