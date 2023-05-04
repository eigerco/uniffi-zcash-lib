use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{Read, Seek},
    path::PathBuf,
};

use clap::Command;

use strum::{Display, EnumIter, EnumString, EnumVariantNames, IntoEnumIterator, VariantNames};

#[derive(Debug, Display, EnumString, EnumIter, EnumVariantNames)]
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

            // Generate the dynamic libraries.
            println!("{}", "Generating shared library ...");
            std::process::Command::new("cargo")
                .arg("build")
                .arg("--release")
                .current_dir(root_dir.clone())
                .spawn()?
                .wait_with_output()?;

            let mut zcash_so_file = File::open(root_dir.join("target/release/libuniffi_zcash.so"))?;

            println!("{}", "Generating language bindings ...");
            SupportedLangs::iter().try_for_each(|lang| {
                std::process::Command::new("cargo")
                    .arg("run")
                    .arg("-p")
                    .arg("uniffi-bindgen")
                    .arg("generate")
                    .arg(udl_path.clone())
                    .arg("--config")
                    .arg(root_dir.join("uniffi-bindgen/uniffi.toml"))
                    .arg("--language")
                    .arg(lang.to_string())
                    .arg("--out-dir")
                    .arg(target_bindings_path.join(lang.to_string()))
                    .spawn()?
                    .wait_with_output()?;

                // Language specific build stuff
                match lang {
                    SupportedLangs::Python => {
                        copy_so_file_for(lang, target_bindings_path.clone(), &mut zcash_so_file)
                    }
                    SupportedLangs::Kotlin => {
                        copy_so_file_for(lang, target_bindings_path.clone(), &mut zcash_so_file)
                    }
                    SupportedLangs::Swift => {
                        copy_so_file_for(lang, target_bindings_path.clone(), &mut zcash_so_file)
                    }
                    SupportedLangs::Ruby => {
                        copy_so_file_for(lang, target_bindings_path.clone(), &mut zcash_so_file)
                    }
                }
            })
        }
        _ => Err("Command not found. See help.".into()),
    }
}

fn copy_so_file_for<SR: Read + Seek>(
    lang: SupportedLangs,
    bindings_path: PathBuf,
    zcash_so_file: &mut SR,
) -> CLIResult<()> {
    let mut lang_so_file = File::create(
        bindings_path
            .join(lang.to_string())
            .join("libuniffi_zcash.so"),
    )?;
    zcash_so_file.rewind()?;
    std::io::copy(zcash_so_file, &mut lang_so_file)?;
    Ok(())
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
