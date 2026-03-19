use super::file_kind::FileKind;
use crate::domain::{Entry, EntryKind};
use std::path::Path;

pub fn classify(entry: &Entry) -> FileKind {
    match entry.kind() {
        EntryKind::Directory => FileKind::Directory,
        EntryKind::File => classify_file(&entry.path()),
    }
}

fn classify_file(path: &Path) -> FileKind {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    match ext.as_str() {
        "rs" => FileKind::Rust,
        "java" => FileKind::Java,
        "py" => FileKind::Python,
        "js" => FileKind::JavaScript,
        "ts" => FileKind::TypeScript,
        "lua" => FileKind::Lua,
        "swift" => FileKind::Swift,
        "kt" => FileKind::Kotlin,
        "r" => FileKind::R,
        "jl" => FileKind::Julia,
        "hs" => FileKind::Haskell,

        "sh" | "bash" | "zsh" => FileKind::Shell,
        "yml" | "yaml" => FileKind::Yaml,
        "html" | "htm" => FileKind::Html,
        "css" => FileKind::Css,

        "md" => FileKind::Markdown,
        "toml" => FileKind::Toml,
        "json" => FileKind::Json,

        "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" => FileKind::Image,

        "pdf" => FileKind::Pdf,

        "mp4" | "mkv" | "avi" | "mov" => FileKind::Video,
        "mp3" | "wav" | "flac" | "ogg" => FileKind::Audio,

        "zip" | "tar" | "gz" | "xz" | "bz2" | "7z" => FileKind::Archive,

        "" => FileKind::Other,

        _ => FileKind::Other,
    }
}
