use super::file_kind::FileKind;

pub fn icon(kind: FileKind) -> &'static str {
    match kind {
        FileKind::EmptyDirectory => "",
        FileKind::Directory => "",

        FileKind::Rust => "",
        FileKind::Java => "",
        FileKind::Python => "",
        FileKind::JavaScript => "",
        FileKind::TypeScript => "",
        FileKind::Lua => "",
        FileKind::Swift => "",
        FileKind::Kotlin => "",
        FileKind::R => "󰟔",
        FileKind::Julia => "",
        FileKind::Haskell => "",

        FileKind::Shell => "",
        FileKind::Yaml => "",
        FileKind::Html => "",
        FileKind::Css => "",

        FileKind::Markdown => "",
        FileKind::Toml => "",
        FileKind::Json => "",

        FileKind::Image => "",
        FileKind::Pdf => "",

        FileKind::Video => "",
        FileKind::Audio => "",

        FileKind::Archive => "",

        FileKind::Binary => "",

        FileKind::Other => "",
    }
}
