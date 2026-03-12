use crate::buffer::Buffer;
use crate::buffer::ValidationError;
use crate::fs::{FsError, RealFs, VirtualFs, apply_diff};
use crate::ui::{Command, Direction};

pub struct App {
    pub scroll_offset: usize,
    pub view_height: usize,
    pub buffer: Buffer,
    virtual_fs: VirtualFs,
    real_fs: RealFs,
    state: AppState,
    error_message: Option<String>,
    pub command_buffer: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppState {
    Running,
    Error,
    Exiting,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SaveError {
    Validation(Vec<ValidationError>),
    Fs(FsError),
}

impl App {
    pub fn new(buffer: Buffer, virtual_fs: VirtualFs, real_fs: RealFs) -> Self {
        Self {
            scroll_offset: 0,
            view_height: 0,
            buffer,
            virtual_fs,
            real_fs,
            state: AppState::Running,
            error_message: None,
            command_buffer: String::new(),
        }
    }

    pub fn state(&self) -> AppState {
        self.state
    }

    pub fn update_scroll(&mut self) {
        let cursor_row = self.buffer.cursor().row;

        if cursor_row < self.scroll_offset {
            self.scroll_offset = cursor_row;
        }

        if self.view_height > 0 && cursor_row >= self.scroll_offset + self.view_height {
            self.scroll_offset = cursor_row - self.view_height + 1;
        }
    }

    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    pub fn request_exit(&mut self, force: bool) {
        if self.buffer.is_dirty() && !force {
            self.error_message = Some("No write since last change (add ! to override)".into());
            self.state = AppState::Error;
        } else {
            self.state = AppState::Exiting;
        }
    }

    pub fn handle_insert_kind(&mut self, kind: crate::ui::InsertKind) {
        use crate::ui::InsertKind;
        match kind {
            InsertKind::BeforeCursor => {}
            InsertKind::AfterCursor => self.buffer.move_right(),
            InsertKind::LineStart => self.buffer.move_to_line_start(),
            InsertKind::LineEnd => self.buffer.move_to_line_end(),
            InsertKind::NewLineBelow => self.buffer.add_line_below(
                crate::domain::EntryId::generate(),
                crate::domain::EntryKind::File,
            ),
            InsertKind::NewLineAbove => self.buffer.add_line_above(
                crate::domain::EntryId::generate(),
                crate::domain::EntryKind::File,
            ),
        }
    }

    pub fn set_error(&mut self, msg: String) {
        self.error_message = Some(msg);
        self.state = AppState::Error;
    }

    pub fn save(&mut self) -> Result<(), SaveError> {
        if let Err(errors) = self.buffer.validate() {
            self.error_message = Some("validation error".into());
            self.state = AppState::Error;
            return Err(SaveError::Validation(errors));
        }

        let diffs = self.buffer.build_diff();

        if diffs.is_empty() {
            return Ok(());
        }

        apply_diff(&mut self.virtual_fs, &diffs).map_err(SaveError::Fs)?;
        apply_diff(&mut self.real_fs, &diffs).map_err(SaveError::Fs)?;

        self.buffer.commit();

        self.state = AppState::Running;
        self.error_message = None;

        Ok(())
    }

    pub fn execute_cmd(&mut self) {
        let cmd = self.command_buffer.trim().to_string();
        self.command_buffer.clear();

        match cmd.as_str() {
            "w" => {
                if let Err(e) = self.save() {
                    self.set_error(format!("{:?}", e));
                }
            }
            "q" => {
                self.request_exit(false);
            }
            "q!" => {
                self.request_exit(true);
            }
            "wq" => {
                if let Err(e) = self.save() {
                    self.set_error(format!("{:?}", e));
                } else {
                    self.request_exit(false);
                }
            }
            "wq!" => {
                if let Err(e) = self.save() {
                    self.set_error(format!("{:?}", e));
                } else {
                    self.request_exit(true);
                }
            }
            "" => {}
            _ => {
                self.set_error(format!("Unknown command: {}", cmd));
            }
        }
    }

    pub fn execute(&mut self, cmd: Command) {
        match cmd {
            Command::Move(dir, count) => {
                for _ in 0..count {
                    match dir {
                        Direction::Left => self.buffer.move_left(),
                        Direction::Down => self.buffer.move_down(),
                        Direction::Up => self.buffer.move_up(),
                        Direction::Right => self.buffer.move_right(),
                    }
                }
            }

            Command::InsertChar(c) => self.buffer.insert_char(c),
            Command::InsertNewLine => self
                .buffer
                .insert_newline(crate::domain::EntryId::generate()),
            Command::DeleteChar => self.buffer.delete_char(),
            Command::DeleteEntry(count) => {
                for _ in 0..count {
                    self.buffer.delete_line();
                }
            }

            Command::Undo => self.buffer.undo(),
            Command::Redo => self.buffer.redo(),

            Command::Input(c) => {
                self.command_buffer.push(c);
            }
            Command::Backspace => {
                self.command_buffer.pop();
            }

            Command::Execute => unreachable!(),
            Command::Quit => self.request_exit(false),
        }
    }
}
