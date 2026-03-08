#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurrentMode {
    Normal,
    Insert,
    Command,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetMode {
    Normal,
    Insert(crate::ui::mode::InsertKind),
    Command,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsertKind {
    BeforeCursor,
    AfterCursor,
    LineStart,
    LineEnd,
    NewLineBelow,
    NewLineAbove,
}
