pub mod apply;
pub mod real_fs;
pub mod virtual_fs;

use crate::domain::entry::EntryKind;
use std::path::Path;

#[derive(Debug)]
pub enum FsError {
    NotFound,
    AlreadyExists,
    NotEmpty,
    Io(std::io::Error),
}

impl From<std::io::Error> for FsError {
    fn from(e: std::io::Error) -> Self {
        FsError::Io(e)
    }
}

pub trait FileSystem {
    fn rename(&mut self, from: &Path, to: &Path) -> Result<(), FsError>;
    fn create(&mut self, path: &Path, kind: EntryKind) -> Result<(), FsError>;
    fn delete(&mut self, path: &Path) -> Result<(), FsError>;
}
