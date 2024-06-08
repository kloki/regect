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
    body::{captures, substitution, TestInput},
    regex_input::{RegexInput, SubstitutionInput},
};

#[derive(Clone, Copy)]
enum Mode {
    Match,
    Substitution,
}

#[derive(Clone, Copy)]
enum EditMode {
    Regex,
    Substitution,
    Body,
}

enum InfoMode {
    QuickReference,
    Captures,
}

enum Action {
    Continue,
    Quit,
    ReturnValue(String),
}

pub struct App<'a> {
    mode: Mode,
    edit_mode: EditMode,
    info_mode: InfoMode,
    regex_input: RegexInput<'a>,
    sub_input: SubstitutionInput<'a>,
    body: TestInput<'a>,
}

impl App<'_> {
    pub fn new(prefill_input: Option<Vec<String>>) -> Self {
        let mut body = TestInput::new();
        if let Some(input) = prefill_input {
            for line in &input {
                body.textarea.insert_str(line);
                body.textarea.insert_newline();
            }
        }

        Self {
            mode: Mode::Match,
            edit_mode: EditMode::Regex,
            info_mode: InfoMode::Captures,
            regex_input: RegexInput::new(),
            sub_input: SubstitutionInput::new(),
            body: TestInput::new(),
        }
    }

    pub fn run(
        &mut self,
        term: &mut Terminal<CrosstermBackend<StdoutLock<'static>>>,
    ) -> io::Result<Option<String>> {
        loop {
            term.draw(|f| self.draw(f))?;
            match self.handle_input()? {
                Action::Quit => return Ok(None),
                Action::ReturnValue(s) => return Ok(Some(s)),
                Action::Continue => {}
            }
        }
    }
    fn draw(&self, f: &mut Frame) {
        match self.mode {
            Mode::Match => self.draw_match(f),
            Mode::Substitution => self.draw_substitution(f),
        }
    }

    fn draw_match(&self, f: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(2),
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .split(f.size());
        f.render_widget(header(), layout[0]);
        f.render_widget(footer(), layout[4]);

        match self.edit_mode {
            EditMode::Body => {
                f.render_widget(self.regex_input.unfocused(), layout[1]);
                f.render_widget(self.body.textarea.widget(), layout[2]);
            }
            _ => {
                f.render_widget(self.regex_input.textarea.widget(), layout[1]);
                f.render_widget(
                    self.body.highlighted_body(self.regex_input.current_regex()),
                    layout[2],
                );
            }
        }

        match self.info_mode {
            InfoMode::QuickReference => f.render_widget(help(), layout[3]),
            InfoMode::Captures => f.render_widget(
                captures(self.regex_input.current_regex(), self.body.body()),
                layout[3],
            ),
        }
    }

    fn draw_substitution(&self, f: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(2),
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .split(f.size());

        let input_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
            .split(layout[1]);
        f.render_widget(header(), layout[0]);
        f.render_widget(footer(), layout[5]);

        match self.edit_mode {
            EditMode::Body => {
                f.render_widget(self.regex_input.unfocused(), input_layout[0]);
                f.render_widget(self.sub_input.unfocused(), input_layout[1]);
                f.render_widget(self.body.textarea.widget(), layout[2]);
            }
            EditMode::Regex => {
                f.render_widget(self.regex_input.textarea.widget(), input_layout[0]);
                f.render_widget(self.sub_input.unfocused(), input_layout[1]);
                f.render_widget(
                    self.body.highlighted_body(self.regex_input.current_regex()),
                    layout[2],
                );
            }
            EditMode::Substitution => {
                f.render_widget(self.regex_input.unfocused(), input_layout[0]);
                f.render_widget(self.sub_input.textarea.widget(), input_layout[1]);
                f.render_widget(
                    self.body.highlighted_body(self.regex_input.current_regex()),
                    layout[2],
                );
            }
        }
        f.render_widget(
            substitution(
                self.body.body(),
                self.regex_input.current_regex(),
                self.sub_input.current_substitution(),
            ),
            layout[3],
        );

        match self.info_mode {
            InfoMode::QuickReference => f.render_widget(help(), layout[4]),
            InfoMode::Captures => f.render_widget(
                captures(self.regex_input.current_regex(), self.body.body()),
                layout[4],
            ),
        }
    }

    fn toggle_edit_mode(&mut self) {
        match (self.edit_mode, self.mode) {
            (EditMode::Regex, Mode::Match) => self.edit_mode = EditMode::Body,
            (EditMode::Regex, Mode::Substitution) => self.edit_mode = EditMode::Substitution,
            (EditMode::Substitution, _) => self.edit_mode = EditMode::Body,
            (EditMode::Body, _) => self.edit_mode = EditMode::Regex,
        }
    }
    fn toggle_info_mode(&mut self) {
        match self.info_mode {
            InfoMode::QuickReference => self.info_mode = InfoMode::Captures,
            InfoMode::Captures => self.info_mode = InfoMode::QuickReference,
        }
    }

    fn toggle_mode(&mut self) {
        match self.mode {
            Mode::Match => self.mode = Mode::Substitution,
            Mode::Substitution => {
                self.mode = Mode::Match;
                if let EditMode::Substitution = self.edit_mode {
                    self.edit_mode = EditMode::Regex;
                }
            }
        }
    }
    fn handle_input(&mut self) -> io::Result<Action> {
        match (crossterm::event::read()?.into(), self.edit_mode) {
            (
                Input {
                    key: Key::Char('q'),
                    ctrl: true,
                    ..
                },
                _,
            ) => return Ok(Action::Quit),

            (
                Input {
                    key: Key::Char('e'),
                    ctrl: true,
                    ..
                },
                _,
            ) => return Ok(Action::ReturnValue(self.regex_input.current_regex_str())),
            (
                Input {
                    key: Key::Char('o'),
                    ctrl: true,
                    ..
                },
                _,
            ) => match self.regex_input.current_regex() {
                None => return Ok(Action::ReturnValue(self.body.body())),
                Some(reg) => {
                    return Ok(Action::ReturnValue(
                        reg.replace_all(
                            &self.body.body(),
                            self.sub_input.current_substitution().to_string(),
                        )
                        .to_string(),
                    ))
                }
            },
            (Input { key: Key::Tab, .. }, _) => self.toggle_edit_mode(),
            (
                Input {
                    key: Key::Char('h'),
                    ctrl: true,
                    ..
                },
                _,
            ) => self.toggle_info_mode(),
            (
                Input {
                    key: Key::Char('x'),
                    ctrl: true,
                    ..
                },
                _,
            ) => self.toggle_mode(),
            (input, EditMode::Body) => {
                self.body.textarea.input(input);
            }
            (input, EditMode::Regex) => {
                if self.regex_input.textarea.input(input) {
                    self.regex_input.validate()
                }
            }
            (input, EditMode::Substitution) => {
                self.sub_input.textarea.input(input);
            }
        }
        Ok(Action::Continue)
    }
}
