#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiMode {
    Normal,
    Insert,
    Command { input: String },
}
