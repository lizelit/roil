use crossterm::cursor::MoveDown;

use crate::buffer::Buffer;
use crate::buffer::ValidationError;
use crate::fs::{FsError, RealFs, VirtualFs, apply_diff};
use crate::ui::{Command, Direction};

pub struct App {
    pub buffer: Buffer,
    virtual_fs: VirtualFs,
    real_fs: RealFs,
    state: AppState,
    error_message: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppState {
    Running,
    ConfirmExit,
    Saving,
    Error,
    Exiting,
}

#[derive(Debug)]
pub enum SaveError {
    Validation(Vec<ValidationError>),
    Fs(FsError),
}

impl App {
    pub fn new(buffer: Buffer, virtual_fs: VirtualFs, real_fs: RealFs) -> Self {
        Self {
            buffer,
            virtual_fs,
            real_fs,
            state: AppState::Running,
            error_message: None,
        }
    }

    pub fn state(&self) -> AppState {
        self.state
    }

    pub fn request_exit(&mut self) {
        if self.buffer.is_dirty() {
            self.state = AppState::ConfirmExit;
        } else {
            self.state = AppState::Exiting;
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

    pub fn execute(&mut self, cmd: Command) {
        match cmd {
            Command::Move(Direction::Left) => self.buffer.move_left(),
            Command::Move(Direction::Down) => self.buffer.move_down(),
            Command::Move(Direction::Up) => self.buffer.move_up(),
            Command::Move(Direction::Right) => self.buffer.move_right(),

            Command::InsertChar(c) => self.buffer.insert_char(c),
            Command::DeleteChar => self.buffer.delete_char(),
            Command::DeleteEntry => self.buffer.delete_line(),

            Command::Undo => self.buffer.undo(),
            Command::Redo => self.buffer.redo(),

            Command::Save => {
                if let Err(e) = self.save() {
                    self.set_error(format!("{:?}", e));
                }
            }

            Command::Quit => self.request_exit(),
        }
    }
}
