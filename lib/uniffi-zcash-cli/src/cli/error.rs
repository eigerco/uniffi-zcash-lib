use std::{error::Error, fmt::Display};

use clap::parser::MatchesError;
use strum::ParseError;

#[derive(Debug)]
pub struct CLIError {
    pub message: String,
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

impl From<String> for CLIError {
    fn from(value: String) -> Self {
        Self { message: value }
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

impl From<retry::Error<CLIError>> for CLIError {
    fn from(value: retry::Error<CLIError>) -> Self {
        value.error
    }
}

impl From<ParseError> for CLIError {
    fn from(value: ParseError) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}
