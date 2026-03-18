use crate::domain::id::EntryId;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    id: EntryId,
    path: PathBuf,
    kind: EntryKind,
}

impl Entry {
    pub fn new(id: EntryId, path: PathBuf, kind: EntryKind) -> Self {
        Self { id, path, kind }
    }

    pub fn id(&self) -> &EntryId {
        &self.id
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn kind(&self) -> &EntryKind {
        &self.kind
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Directory,
}
