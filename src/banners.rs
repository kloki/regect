use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
const HEADER: &str = r"┏┓┏┓┏┓┏┓┏╋
┛ ┗ ┗┫┗ ┗┗
";
const FOOTER: &str = r"^h quick reference, ^q quit";

const HELP: &str = r"
Character     Description                  Frequency & Quantifiers   Grouping & Boundaries
.             Any character except newline *    0 or more            ()    Grouping
^             Start of string              +    1 or more            (?:)  Non-capturing group
$             End of string                ?    0 or 1 (optional)    (?<>) Named group
\d            Any digit ([0-9])            {n}  Exactly n            (?=)  Positive lookahead
\D            Any non-digit                {n,} n or more            (?!   Negative lookahead
\w            Any word char ([a-zA-Z0-9_]) {n,m} Between n and m     (?<=  Positive lookbehind
\W            Any non-word char            ?    Non-greedy           (?<!  Negative lookbehind
\s            Any whitespace               |    Alternation (OR)     \b    Word boundary
\S            Any non-whitespace                                     \B    Non-word boundary

Character Sets       Escapes                     Special
[a-z]                \\    Backslash escape      \    Escape character
[^a-z]               \n    Newline               [\^$.|?*+(){}] Escape metacharacters
[a-zA-Z]             \t    Tab                   [\b]  Backspace in char class
[0-9]                \r    Carriage return
[a-zA-Z0-9]          \f    Form feed
";

pub fn header() -> impl Widget {
    Paragraph::new(HEADER)
        .centered()
        .style(Style::default().fg(Color::Cyan))
}
pub fn footer() -> impl Widget {
    Paragraph::new(FOOTER).right_aligned()
}

pub fn help() -> impl Widget {
    Paragraph::new(HELP).block(
        Block::new()
            .border_type(BorderType::Rounded)
            .border_style(Style::default())
            .borders(Borders::ALL)
            .title("Quick Reference"),
    )
}
