use std::{error::Error, fmt::Display, fs::copy, path::PathBuf};

use clap::Command;

use strum::{Display, EnumIter, EnumString, EnumVariantNames, IntoEnumIterator, VariantNames};

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
        .subcommand(Command::new("generate").about(format!(
            "Generates UniFFI bindings for all the supported languages ({}) and places it in the bindings directory",
            SupportedLangs::VARIANTS.join(",")
        )))
        .get_matches();

    let root_dir = project_root_dir()?;

    match matches.subcommand() {
        Some(("generate", _)) => {
            // Define paths
            let udl_path = root_dir.join("uniffi-zcash/src/zcash.udl");
            let target_bindings_path = root_dir.join("bindings");
            let target_path = root_dir.join("target/release");

            // Generate the dynamic libraries.
            println!("Generating shared library ...");
            std::process::Command::new("cargo")
                .arg("build")
                .arg("--release")
                .current_dir(&root_dir)
                .spawn()?
                .wait_with_output()?;

            let zcash_shared_lib_path = target_path.join("libuniffi_zcash.so");

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
                    .arg(root_dir.join("uniffi-bindgen/uniffi.toml"))
                    .arg("--language")
                    .arg(lang.to_string())
                    .arg("--out-dir")
                    .arg(target_bindings_path.join(lang.to_string()))
                    .spawn()?
                    .wait_with_output()?;

                let shared_lib_dest_path = target_bindings_path
                    .join(lang.to_string())
                    .join("libuniffi_zcash.so");

                copy(&zcash_shared_lib_path, &shared_lib_dest_path)?;

                let bindings_dir = target_bindings_path.join(lang.to_string());

                // Language specific build stuff
                match lang {
                    SupportedLangs::Python => Ok(()),
                    SupportedLangs::Kotlin => Ok(()),
                    SupportedLangs::Swift => {
                        println!("Generating swift module ...");
                        // Needs to generate a swift module first
                        // See https://mozilla.github.io/uniffi-rs/swift/module.html
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
                            .arg(&target_path)
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
        _ => Err("Command not found. See help.".into()),
    }
}

fn project_root_dir() -> CLIResult<PathBuf> {
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

type CLIResult<T> = Result<T, CLIError>;
