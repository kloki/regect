use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use regex::Regex;
use tui_textarea::TextArea;

pub struct RegexInput<'a> {
    pub textarea: TextArea<'a>,
}

impl RegexInput<'_> {
    pub fn new() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_placeholder_text("Enter a valid regex");

        textarea.set_block(
            Block::default()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL)
                .title("regex"),
        );
        Self { textarea }
    }

    pub fn unfocused(&self) -> impl Widget + '_ {
        Paragraph::new(self.textarea.lines()[0].clone()).block(
            Block::new()
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray))
                .borders(Borders::ALL)
                .title("regex"),
        )
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
                    .title("regex"),
            );
        }
    }
}
