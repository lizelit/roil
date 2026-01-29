use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub enum FileKind {
    EmptyDirectory,
    Directory,
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Markdown,
    Toml,
    Unknown,
}

pub struct FileItem {
    pub name: String,
    pub file_kind: FileKind,
}

impl FileItem {
    pub fn new(path: &Path) -> Result<Self> {
        let name = path
            .file_name()
            .context("Invalid file name")?
            .to_string_lossy()
            .into_owned();
        let metadata = fs::metadata(path).ok();
        let is_dir = metadata.map(|m| m.is_dir()).unwrap_or(false);

        let file_kind = if is_dir {
            let is_empty = fs::read_dir(path)
                .map(|mut rd| rd.next().is_none())
                .unwrap_or(true);
            if is_empty {
                FileKind::EmptyDirectory
            } else {
                FileKind::Directory
            }
        } else {
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            match ext.to_lowercase().as_str() {
                "rs" => FileKind::Rust,
                "py" => FileKind::Python,
                "js" => FileKind::JavaScript,
                "ts" => FileKind::TypeScript,
                "toml" => FileKind::Toml,
                "md" => FileKind::Markdown,
                _ => FileKind::Unknown,
            }
        };
        Ok(Self { name, file_kind })
    }
}
