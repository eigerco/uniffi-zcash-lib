use nu_ansi_term::{AnsiString, AnsiStrings, Color, Style};
use public_api::tokens::Token;

// Color the given Token to render it with a nice syntax highlighting. The
// theme is inspired by dark+ in VS Code and uses the default colors from the
// terminal to always provide a readable and consistent color scheme.
// An extra color can be provided to be used as background color.
fn color_item_token(token: &Token, bg: Option<Color>) -> AnsiString<'_> {
    let style = |color: Style, text: &str| {
        bg.map_or_else(
            || color.paint(text.to_string()),
            |bg| color.on(bg).paint(text.to_string()),
        )
    };
    match token {
        Token::Symbol(text) => style(Style::default(), text),
        Token::Qualifier(text) => style(Color::Blue.into(), text),
        Token::Kind(text) => style(Color::Blue.into(), text),
        Token::Whitespace => style(Style::default(), " "),
        Token::Identifier(text) => style(Color::Cyan.into(), text),
        Token::Annotation(text) => style(Style::default(), text),
        Token::Self_(text) => style(Color::Blue.into(), text),
        Token::Function(text) => style(Color::Yellow.into(), text),
        Token::Lifetime(text) => style(Color::Blue.into(), text),
        Token::Keyword(text) => style(Color::Blue.into(), text),
        Token::Generic(text) => style(Color::Green.into(), text),
        Token::Primitive(text) => style(Color::Green.into(), text),
        Token::Type(text) => style(Color::Green.into(), text),
    }
}

// Returns a styled string similar to `color_item_token`, but where whole tokens are highlighted if
// they contain a difference.
pub(crate) fn color_item_with_diff(
    diff_slice: &[diff::Result<&&Token>],
    is_old_item: bool,
) -> String {
    let styled_strings = diff_slice
        .iter()
        .filter_map(|diff_result| match diff_result {
            diff::Result::Left(&token) => is_old_item.then(|| {
                Color::Fixed(9)
                    .on(Color::Fixed(52))
                    .bold()
                    .paint(token.text())
            }),
            diff::Result::Both(&token, _) => Some(color_item_token(token, None)),
            diff::Result::Right(&token) => (!is_old_item).then(|| {
                Color::Fixed(10)
                    .on(Color::Fixed(22))
                    .bold()
                    .paint(token.text())
            }),
        })
        .collect::<Vec<_>>();

    AnsiStrings(&styled_strings).to_string()
}

// Returns a styled string for a full stream of tokens.
// Basically colors the diff line.
pub(crate) fn color_token_stream<'a>(
    tokens: impl Iterator<Item = &'a Token>,
    bg: Option<Color>,
) -> String {
    let styled = tokens.map(|t| color_item_token(t, bg)).collect::<Vec<_>>();
    AnsiStrings(&styled).to_string()
}
