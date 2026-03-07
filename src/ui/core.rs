use std::io;

use crossterm::{
    event, execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{Terminal, backend::CrosstermBackend};

use crate::app::app::{App, AppState};

use super::{command::map_event, event::from_crossterm, mode::Mode, render::render};

pub struct Ui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    mode: Mode,
}

impl Ui {
    pub fn new() -> anyhow::Result<Self> {
        enable_raw_mode()?;

        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            terminal,
            mode: Mode::Normal,
        })
    }

    pub fn run<FV, FR>(&mut self, app: &mut App<FV, FR>) -> anyhow::Result<()> {
        loop {
            self.terminal.draw(|f| render(f, app))?;

            if let event::Event::Key(key) = event::read()? {
                if let Some(ev) = from_crossterm(event::Event::Key(key)) {
                    let cmd = map_event(self.mode, ev);

                    self.mode = app.handle_command(cmd, self.mode);
                }
            }

            if matches!(app.state(), AppState::Exiting) {
                break;
            }
        }

        Ok(())
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
    }
}
