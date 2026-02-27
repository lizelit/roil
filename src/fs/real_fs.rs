use super::{FileSystem, FsError};
use crate::domain::entry::EntryKind;
use std::fs;
use std::path::Path;

pub struct RealFs;

impl FileSystem for RealFs {
    fn rename(&mut self, from: &Path, to: &Path) -> Result<(), FsError> {
        fs::rename(from, to)?;
        Ok(())
    }

    fn create(&mut self, path: &Path, kind: EntryKind) -> Result<(), FsError> {
        match kind {
            EntryKind::File => {
                fs::File::create(path)?;
            }
            EntryKind::Directory => {
                fs::create_dir(path)?;
            }
        }
        Ok(())
    }

    fn delete(&mut self, path: &Path) -> Result<(), FsError> {
        if path.is_dir() {
            fs::remove_dir(path)?;
        } else {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}
