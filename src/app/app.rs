use crate::buffer::buffer::Buffer;
use crate::buffer::validation::ValidationError;
use crate::fs::{real_fs::RealFs, virtual_fs::VirtualFs};

pub struct App {
    pub buffer: Buffer,
    virtual_fs: VirtualFs,
    real_fs: RealFs,
    state: AppState,
    error_message: Option<String>,
}

#[derive(Clone, Copy)]
pub enum AppState {
    Running,
    ConfirmExit,
    Saving,
    Error,
    Exiting,
}

impl App {
    pub fn state(&self) -> AppState {
        self.state
    }

    pub fn request_exit(&mut self) {
        self.state = AppState::Exiting;
    }
}
