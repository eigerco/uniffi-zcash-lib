use anyhow::Context;
use clap::ColorChoice;
use nu_ansi_term::{self, Color};
use public_api::tokens::Token;
use serde::Deserialize;

use super::grep_type::GrepType;

const TOO_MANY_GREP_MATCHES_NUM: usize = 30;

#[derive(Deserialize)]
struct Range {
    start: Position,
    end: Position,
}

#[derive(Deserialize)]
struct Position {
    line: usize,
    column: usize,
}

#[derive(Deserialize)]
struct Match {
    range: Range,
    file: String,
    lines: String,
}

// An item representing the thing to search(grep) for in the code.
#[derive(Default)]
pub(crate) struct GrepItem {
    pub(crate) api_diff: String,
    pub(crate) color: ColorChoice,
    pattern: String,
    grep_type: GrepType,
    grep_output: Option<String>,
    matches: Vec<Match>,
}

impl GrepItem {
    /// Runs ast-grep (sg command), which contextually searches for a pattern, based on the programming
    /// language
    pub(crate) fn grep(&mut self, grep_dir: &str) -> anyhow::Result<&Self> {
        let output = std::process::Command::new("sg")
            .arg("run")
            .arg("-p")
            .arg(&self.pattern)
            .arg("-l")
            .arg("rs")
            .arg("--color")
            .arg(self.color.to_string())
            .arg("--heading")
            .arg("always")
            .arg("--context")
            .arg("1")
            .arg(grep_dir)
            .arg("--json")
            .output()
            .with_context(|| format!("failed to grep for \"{}\"", self.pattern))?;

        // treat erroneous grep results as empty
        if !output.status.success() || output.stdout.is_empty() {
            self.grep_output = None;
        } else {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let grep_matches = serde_json::from_str::<Vec<Match>>(stdout.as_ref())?;

            self.matches = grep_matches;
            self.grep_output = Some(stdout.to_string());
        }
        Ok(self)
    }

    /// Prints the grep results for the GrepItem
    pub(crate) fn print_result(&self) {
        // impl never has a grep_pattern set
        if self.grep_type == GrepType::Impl {
            println!("{}", self);
            return;
        }

        // skip empty results
        if self.matches.is_empty() {
            return;
        }

        // display grep item information
        println!("{}", self);

        // skip results if grep returns too many matches
        if self.matches.len() > TOO_MANY_GREP_MATCHES_NUM {
            return;
        }

        let mut last_file_with_match = "";
        for (i, m) in self.matches.iter().enumerate() {
            if last_file_with_match != &m.file {
                if i != 0 {
                    println!("");
                }

                println!("{}", m.file);
                last_file_with_match = &m.file;
            }

            if self.color != ColorChoice::Never {
                // Extract the substrings before and after the specified columns.
                let before_text = &m.lines[..m.range.start.column];
                let colored_text = &m.lines[m.range.start.column..m.range.end.column];
                let after_text = &m.lines[m.range.end.column..];

                // Color the specified substring (in this example, it's red).
                let colored_output = format!(
                    "{}{}{}",
                    before_text,
                    &Color::Red.bold().paint(colored_text).to_string(),
                    after_text
                );
                println!("{} | {}", m.range.start.line, colored_output);
            } else {
                println!("{} | {}", m.range.start.line, m.lines);
            }
        }
        println!("");
    }
}

// Generates a GrepItem from a pubcli API diff, which is represented by a stream of tokens.
impl<'a, T> From<T> for GrepItem
where
    T: Iterator<Item = &'a Token>,
{
    fn from(value: T) -> Self {
        let mut token_is_struct_field = false;
        let mut grep_item = GrepItem {
            color: ColorChoice::Never,
            ..Default::default()
        };

        // Iterate over tokens in a single diff
        let mut peekable_tokens = value.peekable();
        while let Some(token) = peekable_tokens.next() {
            match token {
                Token::Keyword(val) => {
                    // We don't grep for impls, because it's too hard to distinguish the type from the generic,
                    // That's why grep_name is not set and we ask for manual checks.
                    if GrepType::from(val) == GrepType::Impl {
                        grep_item.grep_type = GrepType::Impl;
                        break;
                    }
                }
                // fn, struct, const, mod, else?
                Token::Kind(val) => {
                    grep_item.grep_type = GrepType::from(val.to_string());
                }

                Token::Symbol(_) => {
                    if token_is_struct_field {
                        let mut next_token = String::from("");
                        if let Some(nt) = peekable_tokens.peek() {
                            next_token = nt.text().to_string();
                        }

                        grep_item.pattern = next_token;
                        if grep_item.grep_type == GrepType::Empty {
                            grep_item.grep_type = GrepType::StructField;
                        }
                        break;
                    }
                }

                Token::Type(val) => {
                    if grep_item.grep_type != GrepType::Fn && grep_item.grep_type != GrepType::Impl
                    {
                        let mut next_token = String::from("");
                        if let Some(nt) = peekable_tokens.peek() {
                            next_token = nt.text().to_string();
                        }

                        // check if :: symbol follows the type (indicating a struct field)
                        // then in Token::Symbol get the field
                        if next_token == "::" {
                            token_is_struct_field = true;
                            continue;
                        }

                        grep_item.pattern = val.to_string();
                        if grep_item.grep_type == GrepType::Empty {
                            grep_item.grep_type = GrepType::Struct;
                        }
                        break;
                    }
                }
                // the function name
                Token::Function(val) => {
                    grep_item.pattern = val.to_string();
                    break;
                }
                Token::Identifier(val) => match grep_item.grep_type {
                    GrepType::Const => grep_item.pattern = val.to_string(),
                    GrepType::Mod => grep_item.pattern = val.to_string(),
                    _ => (),
                },
                _ => (),
            }
        }

        grep_item
    }
}

// Displays information about the entity that's going to be grepped.
impl std::fmt::Display for GrepItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let styled_pattern = if self.color == ColorChoice::Never {
            self.pattern.to_owned()
        } else {
            Color::Red.bold().paint(&self.pattern).to_string()
        };

        writeln!(f, "{}", self.api_diff)?;

        if self.matches.len() > TOO_MANY_GREP_MATCHES_NUM {
            writeln!(
                f,
                "There are more than {} matches for \"{}\". Results are not shown for better readability. Please check for it manually.",
                TOO_MANY_GREP_MATCHES_NUM,
                styled_pattern,
            )?;
            return Ok(());
        }

        if self.grep_type == GrepType::Impl {
            writeln!(f, "Check manually for this impl's usage within the project")?;
        } else {
            writeln!(
                f,
                "Found possible usage of \"{}\" {} in the project",
                styled_pattern, self.grep_type,
            )?;
        }
        Ok(())
    }
}
