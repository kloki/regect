use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use ratatui_textarea::TextArea;
use regex::Regex;

fn block(title: impl Into<String>) -> Block<'static> {
    Block::default()
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .title(title.into())
}

fn unfocused_block(title: impl Into<String>) -> Block<'static> {
    block(title).border_style(Style::default().fg(Color::Gray))
}

pub struct RegexInput<'a> {
    pub textarea: TextArea<'a>,
}

impl RegexInput<'_> {
    pub fn new() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_placeholder_text("Enter a valid regex");
        textarea.set_style(Style::default().fg(Color::LightGreen));
        textarea.set_block(block("Regex"));
        Self { textarea }
    }

    pub fn unfocused(&self) -> impl Widget + '_ {
        Paragraph::new(self.textarea.lines()[0].clone()).block(unfocused_block("Regex"))
    }

    pub fn current_regex(&self) -> Option<Regex> {
        Regex::new(&self.textarea.lines()[0]).ok()
    }

    pub fn current_regex_str(&self) -> String {
        self.textarea.lines()[0].clone()
    }

    pub fn validate(&mut self) {
        let (style, block) = match Regex::new(&self.textarea.lines()[0]) {
            Ok(_) => (Style::default().fg(Color::LightGreen), block("Regex")),
            Err(err) => (
                Style::default().fg(Color::LightRed),
                block(format!("{}", err)).border_style(Style::default().fg(Color::LightRed)),
            ),
        };
        self.textarea.set_style(style);
        self.textarea.set_block(block);
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
        textarea.set_block(block("Substitution"));
        Self { textarea }
    }

    pub fn unfocused(&self) -> impl Widget + '_ {
        Paragraph::new(self.textarea.lines()[0].clone()).block(unfocused_block("Substitution"))
    }

    pub fn current_substitution(&self) -> String {
        self.textarea.lines()[0].clone()
    }
}
