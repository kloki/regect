use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use regex::Regex;
use ratatui_textarea::TextArea;

pub struct RegexInput<'a> {
    pub textarea: TextArea<'a>,
}

impl RegexInput<'_> {
    pub fn new() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_placeholder_text("Enter a valid regex");
        textarea.set_style(Style::default().fg(Color::LightGreen));

        textarea.set_block(
            Block::default()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL)
                .title("Regex"),
        );
        Self { textarea }
    }

    pub fn unfocused(&self) -> impl Widget + '_ {
        Paragraph::new(self.textarea.lines()[0].clone()).block(
            Block::new()
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Gray))
                .borders(Borders::ALL)
                .title("Regex"),
        )
    }

    pub fn current_regex(&self) -> Option<Regex> {
        Regex::new(&self.textarea.lines()[0]).ok()
    }

    pub fn current_regex_str(&self) -> String {
        self.textarea.lines()[0].clone()
    }

    pub fn validate(&mut self) {
        if let Err(err) = Regex::new(&self.textarea.lines()[0]) {
            self.textarea
                .set_style(Style::default().fg(Color::LightRed));
            self.textarea.set_block(
                Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::LightRed))
                    .title(format!("{}", err)),
            );
        } else {
            self.textarea
                .set_style(Style::default().fg(Color::LightGreen));
            self.textarea.set_block(
                Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .title("Regex"),
            );
        }
    }
}

pub struct SubstitutionInput<'a> {
    pub textarea: TextArea<'a>,
}

impl SubstitutionInput<'_> {
    pub fn new() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_placeholder_text("Enter substitution string");
        textarea.set_style(Style::default().fg(Color::LightGreen));

        textarea.set_block(
            Block::default()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL)
                .title("Substitution"),
        );
        Self { textarea }
    }

    pub fn unfocused(&self) -> impl Widget + '_ {
        Paragraph::new(self.textarea.lines()[0].clone()).block(
            Block::new()
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Gray))
                .borders(Borders::ALL)
                .title("Substitution"),
        )
    }

    pub fn current_substitution(&self) -> String {
        self.textarea.lines()[0].clone()
    }
}
