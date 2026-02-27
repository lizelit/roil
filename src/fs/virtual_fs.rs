use std::collections::HashMap;
use std::ffi::FromBytesUntilNulError;
use std::fs::exists;
use std::path::{Path, PathBuf};

use super::{FileSystem, FsError};
use crate::domain::entry::EntryKind;

pub struct VirtualFs {
    entries: HashMap<PathBuf, EntryKind>,
}

impl VirtualFs {
    pub fn new(initial: Vec<(PathBuf, EntryKind)>) -> Self {
        Self {
            entries: initial.into_iter().collect(),
        }
    }

    pub fn exists(&self, path: &Path) -> bool {
        self.entries.contains_key(path)
    }
}

impl FileSystem for VirtualFs {
    fn rename(&mut self, from: &Path, to: &Path) -> Result<(), FsError> {
        if !self.exists(from) {
            return Err(FsError::NotFound);
        }
        if self.exists(to) {
            return Err(FsError::AlreadyExists);
        }
        let kind = self.entries.remove(from).unwrap();
        self.entries.insert(to.to_path_buf(), kind);
        Ok(())
    }

    fn create(&mut self, path: &Path, kind: EntryKind) -> Result<(), FsError> {
        if self.exists(path) {
            return Err(FsError::AlreadyExists);
        }
        self.entries.insert(path.to_path_buf(), kind);
        Ok(())
    }

    fn delete(&mut self, path: &Path) -> Result<(), FsError> {
        if !self.exists(path) {
            return Err(FsError::NotFound);
        }

        self.entries.remove(path);
        Ok(())
    }
}
