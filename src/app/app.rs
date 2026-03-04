use crate::buffer::buffer::Buffer;
use crate::buffer::validation::ValidationError;
use crate::fs::apply::apply_diff;
use crate::fs::{FileSystem, FsError};

pub struct App<FV, FR>
where
    FV: FileSystem,
    FR: FileSystem,
{
    pub buffer: Buffer,
    virtual_fs: FV,
    real_fs: FR,
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

impl<FV, FR> App<FV, FR>
where
    FV: FileSystem,
    FR: FileSystem,
{
    pub fn new(buffer: Buffer, virtual_fs: FV, real_fs: FR) -> Self {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entry::{Entry, EntryKind};
    use crate::domain::id::EntryId;
    use crate::fs::virtual_fs::VirtualFs;
    use std::path::PathBuf;

    fn setup_app() -> App<VirtualFs, VirtualFs> {
        let parent = PathBuf::from("/tmp");

        let entries = vec![Entry {
            id: EntryId::new(1),
            path: parent.join("a.txt"),
            kind: EntryKind::File,
        }];

        let buffer = Buffer::new(parent.clone(), entries.clone());

        let initial_state: Vec<_> = entries.iter().map(|e| (e.path.clone(), e.kind)).collect();

        let virtual_fs = VirtualFs::new(initial_state.clone());
        let real_fs = VirtualFs::new(initial_state);

        App::new(buffer, virtual_fs, real_fs)
    }

    #[test]
    fn save_does_nothing_when_not_dirty() {
        let mut app = setup_app();
        assert!(!app.buffer.is_dirty());
        let result = app.save();
        assert!(result.is_ok());
        assert!(!app.buffer.is_dirty());
    }

    #[test]
    fn save_applies_rename() {
        let mut app = setup_app();
        app.buffer.lines[0].name = "b.txt".into();

        assert!(app.buffer.is_dirty());
        let result = app.save();

        assert!(result.is_ok());
        assert!(!app.buffer.is_dirty());

        let current = app.buffer.build_current_entries();
        assert_eq!(current[0].path.file_name().unwrap(), "b.txt");
    }

    #[test]
    fn save_fails_on_validation_error() {
        let mut app = setup_app();

        app.buffer.lines[0].name = "".into();
        let result = app.save();

        assert!(result.is_err());
        assert_eq!(app.state(), AppState::Error);
        assert!(app.buffer.is_dirty());
    }
}
