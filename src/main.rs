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
    let terminal = Terminal::new(backend)?;

    let mut ui = Ui::new(terminal);
    let parent = PathBuf::from(".");
    let entries: Vec<Entry> = vec![Entry {
        id: EntryId::new(1),
        path: parent.join("test.txt"),
        kind: EntryKind::File,
    }];

    let buffer = Buffer::new(parent, entries);
    let vfs = VirtualFs::new(Vec::new());
    let rfs = RealFs::new();

    let mut app = App::new(buffer, vfs, rfs);

    let result = ui.run(&mut app);

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    result
}
