use clap::{builder::ValueParser, Arg, ArgMatches, Command};
use strum::VariantNames;

use crate::SupportedLangs;

use self::error::CLIError;

pub mod error;

pub fn get_matches() -> ArgMatches {
    Command::new("UniFFI Zcash CLI")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A CLI for managing internal repo workflows")
        .subcommand_required(true)
        .subcommand(Command::new("bindgen").about(format!(
            "Generates UniFFI bindings for all the supported languages ({}) and places it in the bindings directory",
            SupportedLangs::VARIANTS.join(",")
        )))
        .subcommand(
            Command::new("release").about(format!(
            "Prepares a release given a version (semantic versioning), creating all languages ({}) specific packages. It needs to be executed after the bindgen command",
            SupportedLangs::VARIANTS.join(",")))
            .arg(
                Arg::new("version")
                .short('v')
                .long("version")
                .required(true)
                .value_parser(validator_semver())
            )
            .arg(
                Arg::new("swift_repo_url")
                .long("swift-repo-url")
                .required(true)
                .env("SWIFT_GIT_REPO_URL")
                .help("For auth, use a Github personal access token.\nSee https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token\nExample: https://<github-username>:<github-token>@github.com/<your-repository>.git")
            )
        )
        .get_matches()
}

/// See https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string
const REGEX_SEMVER: &str = r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$";
/// It generates a validator for semantic versioning
pub fn validator_semver() -> ValueParser {
    validator_regex(REGEX_SEMVER, "semver: https://semver.org")
}

/// Creates a clap validator (using ValueParser API) with a regex.
/// # Arguments
///
/// * `regex`   - The regex to test against.
/// * `err_msg` - Is a human friendly message to show in case the parser fails.
pub fn validator_regex(regex: &'static str, err_msg: &'static str) -> ValueParser {
    ValueParser::from(move |input: &str| -> CLIResult<String> {
        let reg = regex::Regex::new(regex).unwrap();
        match reg.is_match(input) {
            true => Ok(input.to_owned()),
            false => Err(format!("Value \"{}\" is not matching format: {}", input, err_msg).into()),
        }
    })
}

pub type CLIResult<T> = Result<T, CLIError>;
