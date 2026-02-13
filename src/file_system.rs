use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileItem {
    pub name: String,
    pub kind: FileKind,
    pub id: Option<Uuid>,
    pub path: PathBuf,
}

impl FileKind {
    pub fn from_path(path: &Path) -> Self {
        if path.is_dir() {
            let is_empty = fs::read_dir(path)
                .map(|mut rd| rd.next().is_none())
                .unwrap_or(false);
            if is_empty {
                Self::EmptyDirectory
            } else {
                Self::Directory
            }
        } else {
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            match ext.to_lowercase().as_str() {
                "rs" => Self::Rust,
                "py" => Self::Python,
                "js" => Self::JavaScript,
                "ts" => Self::TypeScript,
                "toml" => Self::Toml,
                "md" => Self::Markdown,
                _ => Self::Unknown,
            }
        }
    }

    pub fn icon(&self) -> char {
        match self {
            Self::EmptyDirectory => '\u{f115}',
            Self::Directory => '\u{f07b}',
            Self::Rust => '\u{e7a8}',
            Self::Python => '\u{e73c}',
            Self::JavaScript => '\u{e781}',
            Self::TypeScript => '\u{e8ca}',
            Self::Markdown => '\u{e73e}',
            Self::Toml => '\u{e6b2}',
            Self::Unknown => '\u{f15b}',
        }
    }
}

impl FileItem {
    pub fn new(path: &Path) -> Result<Self> {
        let name = path
            .file_name()
            .context("Invalid file name")?
            .to_string_lossy()
            .into_owned();

        let kind = FileKind::from_path(path);

        Ok(Self {
            name,
            kind,
            id: Some(Uuid::new_v4()),
            path: path.to_path_buf(),
        })
    }

    pub fn execute_rename(&self, new_name: &str) -> Result<()> {
        let mut new_path = self.path.clone();
        new_path.set_file_name(new_name);
        fs::rename(&self.path, new_path)?;
        Ok(())
    }

    pub fn execute_delete(&self) -> Result<()> {
        if self.path.is_dir() {
            fs::remove_dir_all(&self.path)?;
        } else {
            fs::remove_file(&self.path)?;
        }
        Ok(())
    }
}
