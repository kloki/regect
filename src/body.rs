use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use tui_textarea::TextArea;

const HELP: &str = r"
Character     Description                  Frequency & Quantifiers   Grouping & Boundaries
.             Any character except newline *    0 or more            ()    Grouping
^             Start of string              +    1 or more            (?:)  Non-capturing group
$             End of string                ?    0 or 1 (optional)    (?=)  Positive lookahead
\d            Any digit ([0-9])            {n}  Exactly n            (?!   Negative lookahead
\D            Any non-digit                {n,} n or more            (?<=  Positive lookbehind
\w            Any word char ([a-zA-Z0-9_]) {n,m} Between n and m     (?<!  Negative lookbehind
\W            Any non-word char            ?    Non-greedy           \b    Word boundary
\s            Any whitespace               |    Alternation (OR)     \B    Non-word boundary
\S            Any non-whitespace

Character Sets       Escapes                     Special
[a-z]                \\    Backslash escape      \    Escape character
[^a-z]               \n    Newline               [\^$.|?*+(){}] Escape metacharacters
[a-zA-Z]             \t    Tab                   [\b]  Backspace in char class
[0-9]                \r    Carriage return
[a-zA-Z0-9]          \f    Form feed
";

pub struct Body<'a> {
    pub textarea: TextArea<'a>,
}

impl Body<'_> {
    pub fn new() -> Self {
        let mut textarea = TextArea::default();

        textarea.set_block(
            Block::default()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        );
        Self { textarea }
    }

    pub fn highlighted_body(&self) -> impl Widget + '_ {
        Paragraph::new(self.textarea.lines().join("\n")).block(
            Block::new()
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray))
                .borders(Borders::ALL),
        )
    }

    pub fn help(&self) -> impl Widget + '_ {
        Paragraph::new(HELP).block(
            Block::new()
                .border_type(BorderType::Rounded)
                .border_style(Style::default())
                .borders(Borders::ALL)
                .title("Quick Reference"),
        )
    }
}
