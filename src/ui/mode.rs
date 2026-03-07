#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentMode {
    Normal,
    Insert,
    Command,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetMode {
    Normal,
    Insert(InsertKind),
    Command,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertKind {
    BeforeCursor,
    AfterCursor,
    LineStart,
    LineEnd,
    NewLineBelow,
    NewLineAbove,
}
