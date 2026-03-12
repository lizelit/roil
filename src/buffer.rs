mod parse;
mod validation;

pub use validation::ValidationError;
pub use validation::validate;

use std::path::PathBuf;

use crate::domain::{Diff, Entry, EntryId, EntryKind, diff};

#[derive(Clone, Copy, Debug, Default)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

#[derive(Clone, Debug)]
pub struct BufferLine {
    pub id: EntryId,
    pub name: String,
    pub kind: EntryKind,
}

pub struct Buffer {
    parent: PathBuf,
    original: Vec<Entry>,
    lines: Vec<BufferLine>,
    cursor: Cursor,
    undo_stack: Vec<Vec<BufferLine>>,
    redo_stack: Vec<Vec<BufferLine>>,
}

impl Buffer {
    pub fn new(parent: PathBuf, entries: Vec<Entry>) -> Self {
        let lines = entries
            .iter()
            .map(|e| {
                let mut name = e
                    .path
                    .file_name()
                    .expect("entry path must have file name")
                    .to_string_lossy()
                    .to_string();

                if matches!(e.kind, EntryKind::Directory) && !name.ends_with('/') {
                    name.push('/');
                }

                BufferLine {
                    id: e.id,
                    name,
                    kind: e.kind,
                }
            })
            .collect();
        Self {
            parent,
            original: entries,
            lines,
            cursor: Cursor::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn cursor(&self) -> Cursor {
        self.cursor
    }

    pub fn lines(&self) -> &[BufferLine] {
        &self.lines
    }

    pub fn line(&self, row: usize) -> Option<&BufferLine> {
        self.lines.get(row)
    }

    pub fn commit(&mut self) {
        self.original = self.build_current_entries();
    }

    fn snapshot(&mut self) {
        self.undo_stack.push(self.lines.clone());
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(self.lines.clone());
            self.lines = prev;
            self.clamp_cursor();
        }
    }

    pub fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.lines.clone());
            self.lines = next;
            self.clamp_cursor();
        }
    }

    pub fn move_up(&mut self) {
        self.cursor.row = self.cursor.row.saturating_sub(1);
        self.clamp_cursor();
    }

    pub fn move_down(&mut self) {
        self.cursor.row = self.cursor.row.saturating_add(1);
        self.clamp_cursor();
    }

    pub fn move_left(&mut self) {
        self.cursor.col = self.cursor.col.saturating_sub(1);
        self.clamp_cursor();
    }

    pub fn move_right(&mut self) {
        self.cursor.col = self.cursor.col.saturating_add(1);
        self.clamp_cursor();
    }

    pub fn move_to_line_start(&mut self) {
        self.cursor.col = 0;
        self.clamp_cursor();
    }

    pub fn move_to_line_end(&mut self) {
        self.cursor.col = self.current_line_len();
        self.clamp_cursor();
    }

    fn clamp_row(&mut self) {
        if self.lines.is_empty() {
            self.cursor.row = 0;
        } else if self.cursor.row >= self.lines.len() {
            self.cursor.row = self.lines.len() - 1;
        }
    }

    fn clamp_col(&mut self) {
        let len = self.current_line_len();
        if self.cursor.col > len {
            self.cursor.col = len;
        }
    }

    fn current_line(&self) -> Option<&BufferLine> {
        self.lines.get(self.cursor.row)
    }

    fn current_line_mut(&mut self) -> Option<&mut BufferLine> {
        self.lines.get_mut(self.cursor.row)
    }

    fn clamp_cursor(&mut self) {
        self.clamp_row();
        self.clamp_col();
    }

    fn current_line_len(&self) -> usize {
        self.current_line()
            .map(|l| l.name.chars().count())
            .unwrap_or(0)
    }

    pub fn insert_char(&mut self, c: char) {
        let col = self.cursor.col;
        let len = self.current_line_len();

        if col > len {
            return;
        }

        self.snapshot();

        if let Some(line) = self.current_line_mut() {
            let mut chars: Vec<char> = line.name.chars().collect();
            chars.insert(col, c);
            line.name = chars.into_iter().collect();
        }

        self.cursor.col += 1;
    }

    pub fn insert_newline(&mut self, id: EntryId) {
        let row = self.cursor.row;
        let col = self.cursor.col;

        self.snapshot();

        if row >= self.lines.len() {
            // Buffer is completely empty
            self.lines.push(BufferLine {
                id,
                name: String::new(),
                kind: crate::domain::EntryKind::File,
            });
            self.cursor.row = 0;
            self.cursor.col = 0;
            return;
        }

        let kind = self.lines[row].kind;

        let mut chars: Vec<char> = self.lines[row].name.chars().collect();
        let new_chars = chars.split_off(col);

        self.lines[row].name = chars.into_iter().collect();

        self.lines.insert(
            row + 1,
            BufferLine {
                id,
                name: new_chars.into_iter().collect(),
                kind,
            },
        );

        self.cursor.row += 1;
        self.cursor.col = 0;
    }

    pub fn delete_char(&mut self) {
        let col = self.cursor.col;

        if col == 0 {
            if self.cursor.row > 0 && self.current_line_len() == 0 {
                // If it's an empty line, delete the line directly and move up
                self.lines.remove(self.cursor.row);
                // Clamp max row just in case
                if self.cursor.row > self.lines.len() {
                    self.cursor.row = self.lines.len();
                }

                self.cursor.row -= 1;
                self.move_to_line_end();
            }
            return;
        }

        self.snapshot();

        if let Some(line) = self.current_line_mut() {
            let mut chars: Vec<char> = line.name.chars().collect();
            chars.remove(col - 1);
            line.name = chars.into_iter().collect();
        }

        self.cursor.col -= 1;
    }

    pub fn is_dirty(&self) -> bool {
        !self.build_diff().is_empty()
    }

    pub fn delete_line(&mut self) {
        if self.cursor.row >= self.lines.len() {
            return;
        }

        self.snapshot();
        self.lines.remove(self.cursor.row);
        self.clamp_cursor();
    }

    pub fn add_line_below(&mut self, id: EntryId, kind: EntryKind) {
        let insert_pos = self.cursor.row.saturating_add(1).min(self.lines.len());
        self.snapshot();

        self.lines.insert(
            insert_pos,
            BufferLine {
                id,
                name: String::new(),
                kind,
            },
        );

        self.cursor.row = insert_pos;
        self.cursor.col = 0;
        self.clamp_cursor();
    }

    pub fn add_line_above(&mut self, id: EntryId, kind: EntryKind) {
        let insert_pos = self.cursor.row.min(self.lines.len());
        self.snapshot();

        self.lines.insert(
            insert_pos,
            BufferLine {
                id,
                name: String::new(),
                kind,
            },
        );

        self.cursor.row = insert_pos;
        self.cursor.col = 0;
        self.clamp_cursor();
    }

    pub fn build_diff(&self) -> Vec<Diff> {
        let current = self.build_current_entries();
        diff(&self.original, &current)
    }

    pub fn build_current_entries(&self) -> Vec<Entry> {
        self.lines
            .iter()
            .filter(|line| !line.name.trim().is_empty())
            .map(|line| {
                let is_dir = line.name.ends_with('/');
                let kind = if is_dir {
                    EntryKind::Directory
                } else {
                    EntryKind::File
                };
                let strip_name = if is_dir {
                    &line.name[..line.name.len() - 1]
                } else {
                    &line.name
                };

                Entry {
                    id: line.id,
                    path: self.parent.join(strip_name),
                    kind,
                }
            })
            .collect()
    }

    pub fn validate(&self) -> Result<(), Vec<ValidationError>> {
        validate(&self.lines)
    }

    pub fn refresh(&mut self) -> std::io::Result<()> {
        let mut new_entries = Vec::new();

        if let Ok(read_dir) = std::fs::read_dir(&self.parent) {
            for entry in read_dir.flatten() {
                let path = entry.path();
                let kind = if path.is_dir() {
                    EntryKind::Directory
                } else {
                    EntryKind::File
                };
                new_entries.push(Entry {
                    id: EntryId::generate(),
                    path,
                    kind,
                });
            }
        }

        new_entries.sort_by(|a, b| {
            let a_is_dir = matches!(a.kind, EntryKind::Directory);
            let b_is_dir = matches!(b.kind, EntryKind::Directory);

            b_is_dir.cmp(&a_is_dir).then_with(|| {
                let a_name = a
                    .path
                    .file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_default();
                let b_name = b
                    .path
                    .file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_default();

                natord::compare(&a_name, &b_name)
            })
        });

        let new_lines: Vec<BufferLine> = new_entries
            .iter()
            .map(|e| {
                let mut name = e
                    .path
                    .file_name()
                    .expect("entry path must have file name")
                    .to_string_lossy()
                    .to_string();
                if matches!(e.kind, EntryKind::Directory) && !name.ends_with('/') {
                    name.push('/');
                }

                BufferLine {
                    id: e.id,
                    name,
                    kind: e.kind,
                }
            })
            .collect();
        self.original = new_entries;
        self.lines = new_lines;

        self.undo_stack.clear();
        self.redo_stack.clear();

        self.clamp_cursor();

        Ok(())
    }
}
