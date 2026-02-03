use crate::file_system::FileItem;
use anyhow::{Context, Result};
use crossterm::execute;
use ratatui_textarea::{CursorMove, Input, Key, TextArea};
use std::{fs, io};

#[derive(Debug, Default, PartialEq, Eq)]
pub enum AppMode {
    #[default]
    Normal,
    Insert,
    Command,
}

pub struct App {
    pub items: Vec<FileItem>,
    pub textarea: TextArea<'static>,
    pub command_line: TextArea<'static>,
    pub mode: AppMode,
    pub exit: bool,
    pub message: Option<String>,
}

impl App {
    pub fn new() -> Result<Self> {
        let entries = fs::read_dir(".").context("failed to load current directory")?;
        let mut items = Vec::new();
        for entry in entries {
            let path = entry?.path();
            if let Ok(item) = FileItem::new(&path) {
                items.push(item);
            }
        }

        items.sort_by(|a, b| a.name.cmp(&b.name));
        let names: Vec<String> = items.iter().map(|i| i.name.clone()).collect();
        let textarea = TextArea::new(names);
        let command_line = TextArea::new(vec![]);

        Ok(Self {
            items,
            textarea,
            command_line,
            mode: AppMode::Normal,
            exit: false,
            message: None,
        })
    }

    pub fn handle_key_event(&mut self, input: impl Into<Input>) {
        let input = input.into();
        match self.mode {
            AppMode::Normal => match input {
                Input {
                    key: Key::Char(':'),
                    ..
                } => self.mode = AppMode::Command,
                Input {
                    key: Key::Char('i'),
                    ..
                } => self.mode = AppMode::Insert,
                Input {
                    key: Key::Char('j'),
                    ..
                } => self.textarea.move_cursor(CursorMove::Down),
                Input {
                    key: Key::Char('k'),
                    ..
                } => self.textarea.move_cursor(CursorMove::Up),
                Input {
                    key: Key::Char('h'),
                    ..
                } => self.textarea.move_cursor(CursorMove::Back),
                Input {
                    key: Key::Char('l'),
                    ..
                } => self.textarea.move_cursor(CursorMove::Forward),
                _ => {}
            },
            AppMode::Insert => match input {
                Input { key: Key::Esc, .. } => self.mode = AppMode::Normal,

                _ => {
                    self.textarea.input(input);
                }
            },
            AppMode::Command => match input {
                Input {
                    key: Key::Enter, ..
                } => {
                    self.execute_command();
                    self.mode = AppMode::Normal;
                }
                _ => {
                    self.command_line.input(input);
                }
            },
        }
    }
    fn execute_command(&mut self) {
        let cmd = self.command_line.lines()[0].trim();

        if cmd.is_empty() {
            return;
        }

        match cmd {
            "q" => {
                self.exit = true;
                self.message = Some("Exiting application...".to_string());
            }
            "help" => {
                self.message = Some("Available commands: q (quit), help (show this)".to_string());
            }
            "clear" => {
                self.message = Some("Screen cleared".to_string());
            }
            _ => {
                self.message = Some(format!(
                    "Unknown command: '{}'. Type 'help' for available commands.",
                    cmd
                ));
            }
        }

        self.command_line = TextArea::new(vec![]);
    }
}
