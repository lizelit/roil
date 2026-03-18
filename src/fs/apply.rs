use super::{FileSystem, FsError};
use crate::domain::Diff;

pub fn apply_diff(fs: &mut impl FileSystem, diffs: &[Diff]) -> Result<(), FsError> {
    for diff in diffs {
        match diff {
            Diff::Create { entry } => {
                fs.create(&entry.path(), *entry.kind())?;
            }
            Diff::Delete { entry } => {
                fs.delete(&entry.path())?;
            }
            Diff::Rename { from, to } => {
                fs.rename(from, to)?;
            }
        }
    }
    Ok(())
}
