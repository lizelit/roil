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

    let buffer = Buffer::new(parent)?;

    let initial_vfs = buffer
        .build_current_entries()
        .into_iter()
        .map(|e| (e.path, e.kind))
        .collect();

    let vfs = VirtualFs::new(initial_vfs);
    let rfs = RealFs::new();

    let mut app = App::new(buffer, vfs, rfs);

    let result = ui.run(&mut app);

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    result
}
