use std::{io, io::StdoutLock};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    terminal::Terminal,
    Frame,
};
use tui_textarea::{Input, Key};

use crate::{
    banners::{footer, header, help},
    body::{captures, TestInput},
    regex_input::RegexInput,
};
#[derive(Clone)]
enum EditMode {
    RegexEdit,
    BodyEdit,
}

enum InfoMode {
    QuickReference,
    Captures,
}

enum Action {
    Continue,
    Quit,
}

pub struct App<'a> {
    edit_mode: EditMode,
    info_mode: InfoMode,
    regex_input: RegexInput<'a>,
    body: TestInput<'a>,
    layout: Layout,
}

impl App<'_> {
    pub fn new(prefill_input: Option<Vec<String>>) -> Self {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(2),
                Constraint::Length(3),
                Constraint::Fill(2),
                Constraint::Fill(1),
                Constraint::Length(1),
            ]);
        let mut body = TestInput::new();
        if let Some(input) = prefill_input {
            for line in &input {
                body.textarea.insert_str(line);
                body.textarea.insert_newline();
            }
        }

        Self {
            edit_mode: EditMode::RegexEdit,
            info_mode: InfoMode::Captures,
            regex_input: RegexInput::new(),
            body: TestInput::new(),
            layout,
        }
    }

    pub fn run(
        &mut self,
        term: &mut Terminal<CrosstermBackend<StdoutLock<'static>>>,
    ) -> io::Result<()> {
        loop {
            term.draw(|f| self.draw(f))?;
            match self.handle_input()? {
                Action::Quit => return Ok(()),
                Action::Continue => {}
            }
        }
    }

    fn draw(&self, f: &mut Frame) {
        let chunks = self.layout.split(f.size());
        f.render_widget(header(), chunks[0]);
        f.render_widget(footer(), chunks[4]);

        match self.edit_mode {
            EditMode::RegexEdit => {
                f.render_widget(self.regex_input.textarea.widget(), chunks[1]);
                f.render_widget(
                    self.body.highlighted_body(self.regex_input.current_regex()),
                    chunks[2],
                );
            }
            EditMode::BodyEdit => {
                f.render_widget(self.regex_input.unfocused(), chunks[1]);
                f.render_widget(self.body.textarea.widget(), chunks[2]);
            }
        }

        match self.info_mode {
            InfoMode::QuickReference => f.render_widget(help(), chunks[3]),
            InfoMode::Captures => f.render_widget(
                captures(self.regex_input.current_regex(), self.body.body()),
                chunks[3],
            ),
        }
    }

    fn toggle_edit_mode(&mut self) {
        match self.edit_mode {
            EditMode::RegexEdit => self.edit_mode = EditMode::BodyEdit,
            EditMode::BodyEdit => self.edit_mode = EditMode::RegexEdit,
        }
    }
    fn toggle_info_mode(&mut self) {
        match self.info_mode {
            InfoMode::QuickReference => self.info_mode = InfoMode::Captures,
            InfoMode::Captures => self.info_mode = InfoMode::QuickReference,
        }
    }
    fn handle_input(&mut self) -> io::Result<Action> {
        match (crossterm::event::read()?.into(), self.edit_mode.clone()) {
            (
                Input {
                    key: Key::Char('q'),
                    ctrl: true,
                    ..
                },
                _,
            ) => return Ok(Action::Quit),
            (Input { key: Key::Tab, .. }, _) => self.toggle_edit_mode(),
            (
                Input {
                    key: Key::Char('h'),
                    ctrl: true,
                    ..
                },
                _,
            ) => self.toggle_info_mode(),
            (input, EditMode::BodyEdit) => {
                self.body.textarea.input(input);
            }
            (input, _) => {
                if self.regex_input.textarea.input(input) {
                    self.regex_input.validate()
                }
            }
        }
        Ok(Action::Continue)
    }
}
