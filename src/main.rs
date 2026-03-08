mod app;
mod buffer;
mod domain;
mod fs;
mod ui;

use std::{io, path::PathBuf};

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{
    app::App,
    buffer::Buffer,
    domain::{Entry, EntryId, EntryKind},
    fs::{RealFs, VirtualFs},
    ui::Ui,
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut ui = Ui::new(terminal);
    let parent = PathBuf::from(".");
    let mut entries = Vec::new();

    if let Ok(read_dir) = std::fs::read_dir(&parent) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            let kind = if path.is_dir() {
                EntryKind::Directory
            } else {
                EntryKind::File
            };
            entries.push(Entry {
                id: EntryId::generate(),
                path,
                kind,
            });
        }
    }

    let initial_vfs = entries
        .iter()
        .map(|e| (e.path.clone(), e.kind))
        .collect();

    let buffer = Buffer::new(parent, entries);
    let vfs = VirtualFs::new(initial_vfs);
    let rfs = RealFs::new();

    let mut app = App::new(buffer, vfs, rfs);

    let result = ui.run(&mut app);

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    // We do not need terminal.clear() here because LeaveAlternateScreen restores the screen
    // but just making sure disable_raw_mode goes through cleanly.
    
    result
}
