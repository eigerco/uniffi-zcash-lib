use self::error::CLIError;
use crate::SupportedLang;
use clap::{builder::ValueParser, Arg, ArgAction, ArgMatches, Command};
use std::str::FromStr;
use strum::VariantNames;

pub mod error;

pub fn get_matches() -> ArgMatches {
    Command::new("UniFFI Zcash CLI")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A CLI for managing internal repo workflows")
        .subcommand_required(true)
        .subcommand(
            Command::new("bindgen").about(format!(
            "Generates UniFFI bindings for all the supported languages ({}) and places it in the bindings directory",
            SupportedLang::VARIANTS.join(",")
        )))
        .subcommand(
            Command::new("release").about(format!(
            "Prepares a release given a version (semantic versioning), creating all languages ({}) specific packages. It needs to be executed after the bindgen command",
            SupportedLang::VARIANTS.join(",")))
            .arg(arg_version())
            .arg(
                Arg::new("swift_repo_url")
                .long("swift-repo-url")
                .required(true)
                .env("SWIFT_GIT_REPO_URL")
                .help("For auth, use a Github personal access token.\nSee https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token\nExample: https://<github-username>:<github-token>@github.com/<your-repository>.git")
            )
        )
        .subcommand(
            Command::new("publish").about(format!(
            "Publish the previously generated packages (See release command) in all supported languages ({}) registries",
            SupportedLang::VARIANTS.join(",")
            ))
            .arg(arg_version())
            .arg(
                Arg::new("confirmation")
                .short('y')
                .action(ArgAction::SetTrue)
                .required(true)
                .help("This is just a flag for security. Somehow a confirmation that YES, im sure what im doing. I want to publish.")
            )
            .arg(
                Arg::new("only_for_language")
                .long("only-for-language")
                .env("ONLY_FOR_LANGUAGE")
                .value_parser(validator_language())
                .help(format!("Defines if the publish operation should be done only for one language ({}) .Useful in case of partial uploads)", SupportedLang::VARIANTS.join(",")))
            )
            // Python
            .arg(
                Arg::new("python_registry_url")
                .long("python-registry-url")
                .required(true)
                .env("PYTHON_REGISTRY_URL")
                .help("The http[s] URL of the target python package index. i.e https://upload.pypi.org/legacy/")
            )
            .arg(
                Arg::new("python_registry_token")
                .long("python-registry-token")
                .required(true)
                .env("PYTHON_REGISTRY_TOKEN")
                .help("The pypi token, including the prefix 'pypi'.")
            )
            // Ruby
            .arg(
                Arg::new("ruby_registry_url")
                .long("ruby-registry-url")
                .required(true)
                .env("RUBY_REGISTRY_URL")
                .help("The http[s] URL of the target ruby package index. i.e https://rubygems.org")
            )
            .arg(
                Arg::new("ruby_registry_token")
                .long("ruby-registry-token")
                .required(true)
                .env("RUBY_REGISTRY_TOKEN")
                .help("The ruby API key.")
            )
        )
        .get_matches()
}

fn arg_version() -> Arg {
    Arg::new("version")
        .short('v')
        .long("version")
        .required(true)
        .value_parser(validator_semver())
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

/// Checks that provided string matches an internal supported language.
pub fn validator_language() -> ValueParser {
    ValueParser::from(move |input: &str| -> CLIResult<SupportedLang> {
        SupportedLang::from_str(input).map_err(From::from)
    })
}

pub type CLIResult<T> = Result<T, CLIError>;
