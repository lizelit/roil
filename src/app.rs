use crate::file_system::FileItem;
use anyhow::{Context, Result, bail};
use crossterm::execute;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders};
use ratatui_textarea::{CursorMove, Input, Key, TextArea};
use std::path::{Path, PathBuf};
use std::{fs, io, mem::uninitialized};
use crate::file_system::{FileItem, FileKind};

#[derive(Debug, Default, PartialEq, Eq)]
pub enum AppMode {
    #[default]
    Normal,
    Insert,
    Command,
    Wating(char),
}

pub struct App {
    pub current_items: Vec<FileItem>,
    pub edit_buffers: Vec<TextArea<'static>>,
    pub command_line: TextArea<'static>,
    pub cursor_index: usize,
    pub mode: AppMode,
    pub exit: bool,
}

pub enum FileAction {
    Rename { from: PathBuf, to: PathBuf },
    Delete { path: PathBuf },
    CreateFile { path: PathBuf },
    CreateDir { path: PathBuf },
}

impl App {
    pub fn new() -> Result<Self> {
        let entries = fs::read_dir(".").context("failed to load current directory")?;
        let mut current_items = Vec::new();
        let mut edit_buffers = Vec::new();

        for entry in entries {
            let path = entry?.path();
            if let Ok(item) = FileItem::new(&path) {
                let mut ta = TextArea::from(vec![item.name.clone()]);
                ta.set_block(ratatui::widgets::Block::default());
                ta.set_cursor_line_style(ratatui::style::Style::default());

                edit_buffers.push(ta);
                current_items.push(item);
            }
        }
        Ok(Self {
            current_items,
            edit_buffers,
            command_line: TextArea::default(),
            cursor_index: 0,
            mode: AppMode::Normal,
            exit: false,
        })
    }

    pub fn handle_input(&mut self, input: Input) {
        match self.mode {
            AppMode::Normal => self.handle_normal_input(input),
            AppMode::Insert => self.handle_insert_input(input),
            AppMode::Command => self.handle_command_input(input),
            AppMode::Wating(_) => self.handle_waiting_input(input),
        }
    }

    fn handle_normal_input(&mut self, input: Input) {
        match input.key {
            Key::Char('i') => self.mode = AppMode::Insert,
            Key::Char('j') => {
                if self.cursor_index < self.edit_buffers.len() - 1 {
                    self.cursor_index += 1;
                }
            }
            Key::Char('k') => {
                if self.cursor_index > 0 {
                    self.cursor_index -= 1;
                }
            }
            Key::Char('h') => {
                self.edit_buffers[self.cursor_index]
                    .move_cursor(ratatui_textarea::CursorMove::Back);
            }
            Key::Char('l') => {
                self.edit_buffers[self.cursor_index]
                    .move_cursor(ratatui_textarea::CursorMove::Forward);
            }
            Key::Char('d') => {}
            Key::Char('o') => {
                let new_item = FileItem {
                    name: String::new(),
                    kind: FileKind::Unknown,
                    id: None,
                    path: self.path.clone()
                }
            }
            Key::Char(':') => {
                self.mode = AppMode::Command;
                self.command_line.insert_char(':');
            }
            _ => {}
        }
    }

    fn handle_insert_input(&mut self, input: Input) {
        match input.key {
            Key::Esc => self.mode = AppMode::Normal,
            _ => {
                self.edit_buffers[self.cursor_index].input(input);
            }
        }
    }

    fn handle_command_input(&mut self, input: Input) {
        match input.key {
            Key::Esc => {
                self.mode = AppMode::Normal;
                self.command_line = TextArea::default();
            }
            Key::Enter => {
                self.mode = AppMode::Normal;
                let cmd = self.command_line.lines()[0].clone();
                self.execute_command(&cmd);
            }
            _ => {
                self.command_line.input(input);
            }
        }
    }

    fn handle_waiting_input(&mut self, input: Input) {
        match self.mode {
            AppMode::Wating(d) => match input.key {
                Key::Char('d') => unimplemented!(),
                _ => self.mode = AppMode::Normal,
            },
            _ => unreachable!(),
        }
    }

    fn execute_command(&mut self, cmd: &str) {
        match cmd {
            ":w" | ":write" => {
                if let Ok(_) = self.save_changes() {
                    self.command_line = TextArea::from(vec!["Saved!"]);
                }
            }
            ":q" | ":quit" => {
                self.exit = true;
            }
            ":wq" => {
                let _ = self.save_changes();
                self.exit = true;
            }
            _ => {
                self.command_line = TextArea::from(vec!["Unknown command"]);
            }
        }
    }
    pub fn update_styles(&mut self) {
        for (i, textarea) in self.edit_buffers.iter_mut().enumerate() {
            if i == self.cursor_index {
                match self.mode {
                    AppMode::Insert => {
                        textarea
                            .set_cursor_style(Style::default().bg(Color::White).fg(Color::Black));
                        textarea.set_style(Style::default().fg(Color::Yellow));
                    }
                    AppMode::Normal => {
                        textarea
                            .set_cursor_style(Style::default().bg(Color::Gray).fg(Color::Black));
                        textarea.set_style(Style::default().bg(Color::Indexed(237))); // 濃いグレー
                    }
                    _ => {
                        textarea.set_cursor_style(Style::default());
                        textarea.set_style(Style::default());
                    }
                }
            } else {
                textarea.set_cursor_style(Style::default());
                textarea.set_style(Style::default());
            }
        }
    }

    pub fn plan_changes(&self) -> Result<Vec<FileAction>> {
        let mut plans = Vec::new();
        let mut target_names = std::collections::HashSet::new();
        for (item, textarea) in self.current_items.iter().zip(&self.edit_buffers) {
            let new_name = textarea.lines()[0].trim();
            if new_name.is_empty() {
                bail!("err:Name is empty: {}", item.name);
            }

            if target_names.contains(new_name) {
                bail!("err:Already exist that name: {}", new_name);
            }
            target_names.insert(new_name.to_string());

            if item.name != new_name {
                let mut to_path = item.path.clone();
                to_path.set_file_name(new_name);

                plans.push(FileAction::Rename {
                    from: item.path.clone(),
                    to: to_path,
                });
            }
        }
        Ok(plans)
    }

    pub fn execute_operations(acts: Vec<FileAction>) -> Result<()> {
        for act in acts {
            match act {
                FileAction::Rename { from, to } => {
                    std::fs::rename(from, to)?;
                }
                FileAction::Delete { path } => {
                    std::fs::remove_dir(path)?;
                }
                _ => {}
            }
        }
        Ok(())
    }
}
