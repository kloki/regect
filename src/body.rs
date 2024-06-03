use ratatui::{
    layout::Constraint,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Row, Table, Widget},
};
use regex::Regex;
use tui_textarea::TextArea;

const BANNER: &str = r"┏┓┏┓┏┓┏┓┏╋
┛ ┗ ┗┫┗ ┗┗
";
const FOOTER: &str = r"^x to switch input, ^h quick reference, ^q quit";

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

pub fn get_color(index: usize) -> Color {
    match index % 5 {
        0 => Color::Green,
        1 => Color::Blue,
        2 => Color::Yellow,
        3 => Color::Cyan,
        _ => Color::Magenta,
    }
}
pub struct TestInput<'a> {
    pub textarea: TextArea<'a>,
}

impl TestInput<'_> {
    pub fn new() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_style(Style::default().fg(Color::LightGreen));
        textarea.set_block(
            Block::default()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL)
                .title("Test Input"),
        );
        Self { textarea }
    }

    pub fn body(&self) -> String {
        self.textarea.lines().join("\n").to_string()
    }
    pub fn highlighted_body(&self, current_regex: Option<Regex>) -> impl Widget + '_ {
        fn append_match(part: &str, lines: &mut Vec<Vec<Span>>, style: Style) {
            let mut last = lines.len() - 1;
            if !part.contains('\n') {
                lines[last].push(Span::styled(part.to_owned(), style));
                return;
            }

            for p in part.split('\n') {
                lines[last].push(Span::styled(p.to_owned(), style));
                last += 1;
                lines.push(vec![]);
            }

            lines.pop();
        }
        let body = self.textarea.lines().join("\n");
        let mut text = Text::default();
        if let Some(regex) = current_regex {
            let mut lines: Vec<Vec<Span>> = vec![vec![]];

            let mut current_index = 0;

            for (i, re_match) in regex.find_iter(&body).enumerate() {
                append_match(
                    &body[current_index..re_match.start()],
                    &mut lines,
                    Style::default(),
                );
                append_match(
                    &body[re_match.start()..re_match.end()],
                    &mut lines,
                    Style::default().fg(Color::Black).bg(get_color(i)),
                );
                current_index = re_match.end();
            }
            append_match(&body[current_index..], &mut lines, Style::default());
            for line in lines {
                text.push_line(Line::from(line));
            }
        } else {
            text = body.into();
        };

        Paragraph::new(text).block(
            Block::new()
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Gray))
                .borders(Borders::ALL)
                .title("Test Input"),
        )
    }
}

pub fn banner() -> impl Widget {
    Paragraph::new(BANNER)
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

pub fn captures(reg: Option<Regex>, body: String) -> impl Widget {
    if let Some(reg) = reg {
        let mut rows: Vec<Row> = vec![];
        let names = reg
            .capture_names()
            .enumerate()
            .map(|(i, x)| match x {
                Some(name) => name.to_string(),
                None => i.to_string(),
            })
            .collect::<Vec<_>>();

        let widths = vec![Constraint::Fill(1); names.len()];

        for (i, cap) in reg.captures_iter(&body).enumerate() {
            rows.push(
                Row::new(cap.iter().map(|sub| match sub {
                    Some(sub) => sub.as_str().to_string(),
                    None => "".to_string(),
                }))
                .style(Style::default().fg(get_color(i))),
            )
        }

        Table::new(rows, widths)
            .column_spacing(1)
            .header(Row::new(names).bottom_margin(1))
            .block(
                Block::new()
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default())
                    .borders(Borders::ALL)
                    .title("Captures"),
            )
    } else {
        let rows: Vec<Row> = vec![];
        let widths = vec![Constraint::Fill(1); 0];
        Table::new(rows, widths).block(
            Block::new()
                .border_type(BorderType::Rounded)
                .border_style(Style::default())
                .borders(Borders::ALL)
                .title("Captures"),
        )
    }
}
