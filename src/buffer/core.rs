use std::path::PathBuf;

use crate::{
    buffer::validation::{self, ValidationError},
    domain::{
        diff::{Diff, diff},
        entry::{Entry, EntryKind},
        id::EntryId,
    },
};

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
    pub parent: PathBuf,
    original: Vec<Entry>,
    pub lines: Vec<BufferLine>,
    pub cursor: Cursor,
    undo_stack: Vec<Vec<BufferLine>>,
    redo_stack: Vec<Vec<BufferLine>>,
}

impl Buffer {
    pub fn new(parent: PathBuf, entries: Vec<Entry>) -> Self {
        let lines = entries
            .iter()
            .map(|e| BufferLine {
                id: e.id,
                name: e
                    .path
                    .file_name()
                    .expect("entry path must have file name")
                    .to_string_lossy()
                    .to_string(),
                kind: e.kind,
            })
            .collect();
        Self {
            parent: parent,
            original: entries,
            lines: lines,
            cursor: Cursor::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
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

    pub fn delete_char(&mut self) {
        let col = self.cursor.col;

        if col == 0 {
            return;
        }

        let len = self.current_line_len();

        if col - 1 >= len {
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
                kind: kind,
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
                kind: kind,
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
            .map(|line| Entry {
                id: line.id,
                path: self.parent.join(&line.name),
                kind: line.kind,
            })
            .collect()
    }

    pub fn validate(&self) -> Result<(), Vec<ValidationError>> {
        validation::validate(&self.lines)
    }
}
