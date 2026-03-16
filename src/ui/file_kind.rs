#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    EmptyDirectory,
    Directory,

    Rust,
    Java,
    Python,
    JavaScript,
    TypeScript,
    Lua,
    Swift,
    Kotlin,
    R,
    Julia,
    Haskell,

    Shell,
    Yaml,
    Html,
    Css,

    Markdown,
    Toml,
    Json,

    Image,
    Binary,

    Pdf,
    Video,
    Audio,
    Archive,

    Other,
}
