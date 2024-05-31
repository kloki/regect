use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Widget};
use tui_textarea::TextArea;

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
        Paragraph::new(self.textarea.lines().join("\n")).block(Block::new().borders(Borders::ALL))
    }
}
