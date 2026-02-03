mod app;
mod file_system;
mod ui;

use crate::app::{App, AppMode};
use anyhow::Result;
use crossterm::{
    cursor::SetCursorStyle,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    style::SetBackgroundColor,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self, Write};

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new()?;

    loop {
        terminal.draw(|f| {
            f.render_widget(&app, f.size());
        })?;

        match app.mode {
            AppMode::Insert => {
                execute!(io::stdout(), SetCursorStyle::BlinkingBar)?;
            }
            AppMode::Normal | AppMode::Command => {
                execute!(io::stdout(), SetCursorStyle::BlinkingBlock)?;
            }
        }

        io::stdout().flush()?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(':') => app.mode = AppMode::Command,
                KeyCode::Esc => app.mode = AppMode::Normal,
                KeyCode::Char('i') if app.mode == AppMode::Normal => app.mode = AppMode::Insert,
                _ => app.handle_key_event(key),
            }

            if app.exit {
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
