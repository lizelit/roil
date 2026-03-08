mod apply;
mod real_fs;
mod virtual_fs;

use crate::domain::EntryKind;
use std::path::Path;

pub use apply::apply_diff;
pub use real_fs::RealFs;
pub use virtual_fs::VirtualFs;

#[allow(dead_code)]
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
