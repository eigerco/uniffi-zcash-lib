use std::env::set_current_dir;

use bindgen::{generate_bindings, generate_shared_lib};
use cli::{get_matches, CLIResult};

use helper::workspace_root_dir;

use strum::{Display, EnumIter, EnumString, EnumVariantNames};

mod bindgen;
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

    match matches.subcommand() {
        Some(("bindgen", args)) => {
            let languages: Vec<String> = args
                .try_get_many::<String>("languages")?
                .unwrap()
                .map(Clone::clone)
                .collect();
            let shared_lib_path = generate_shared_lib(&root_dir)?;
            Ok(generate_bindings(&root_dir, &shared_lib_path, &languages)?)
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
