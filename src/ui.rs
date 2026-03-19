mod classify;
pub mod command;
mod event;
mod file_kind;
mod icon;
pub mod mode;
mod render;

use command::{Action, map_event};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use event::read_event;
use mode::{CurrentMode, TargetMode};
use ratatui::{Terminal, backend::CrosstermBackend};
use render::render;

use std::{io::Stdout, path::PathBuf};

pub use self::command::{Command, Direction};
pub use mode::InsertKind;

use crate::app::{App, AppEffect};

pub struct Ui {
    mode: CurrentMode,
    pending_keys: String,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Ui {
    pub fn new(terminal: Terminal<CrosstermBackend<Stdout>>) -> Self {
        Self {
            mode: CurrentMode::Normal,
            pending_keys: String::new(),
            terminal,
        }
    }

    pub fn run(&mut self, app: &mut App) -> std::io::Result<()> {
        loop {
            if app.state() == crate::app::AppState::Exiting {
                break;
            }

            let size = self.terminal.size()?;

            use ratatui::layout::{Constraint, Direction, Layout};
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(size);

            let main_area = chunks[0];

            app.update_layout(main_area);

            self.terminal.draw(|frame| {
                render(frame, app, &self.mode);
            })?;

            if let Some(event) = read_event() {
                let Some(action) = map_event(&self.mode, &mut self.pending_keys, event) else {
                    continue;
                };
                if let Some(effect) = self.dispatch(app, action) {
                    self.handle_effect(app, effect)?;
                }
            }
        }
        Ok(())
    }

    fn handle_effect(&mut self, app: &mut App, effect: AppEffect) -> std::io::Result<()> {
        match effect {
            AppEffect::OpenEditor(path) => {
                self.open_editor(path)?;
                app.refresh_buffer()?;
            }
        }
        Ok(())
    }

    fn dispatch(&mut self, app: &mut App, action: Action) -> Option<AppEffect> {
        match action {
            Action::Command(cmd) => self.handle_command(app, cmd),
            Action::ChangeMode(target) => {
                self.change_mode(app, target);
                None
            }
        }
    }

    fn handle_command(&mut self, app: &mut App, cmd: Command) -> Option<AppEffect> {
        match cmd {
            Command::Quit => {
                app.request_exit(false);
                None
            }

            Command::Execute => {
                app.execute_cmd();

                if app.state() != crate::app::AppState::Exiting {
                    self.change_mode(app, TargetMode::Normal);
                }

                None
            }

            _ => app.execute(cmd),
        }
    }

    fn change_mode(&mut self, app: &mut App, target: TargetMode) {
        if matches!(target, TargetMode::Normal) {
            app.clear_command();
        }

        match target {
            TargetMode::Normal => {
                self.mode = CurrentMode::Normal;
            }
            TargetMode::Insert(kind) => {
                self.mode = CurrentMode::Insert;
                app.handle_insert_kind(kind);
            }
            TargetMode::Command => {
                self.mode = CurrentMode::Command;
            }
        }
    }

    fn suspend_terminal(&mut self) -> std::io::Result<()> {
        use crossterm::terminal::LeaveAlternateScreen;
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    fn resume_terminal(&mut self) -> std::io::Result<()> {
        use crossterm::terminal::EnterAlternateScreen;
        enable_raw_mode()?;
        execute!(self.terminal.backend_mut(), EnterAlternateScreen)?;
        self.terminal.clear()?;
        Ok(())
    }

    fn open_editor(&mut self, path: PathBuf) -> std::io::Result<()> {
        self.suspend_terminal()?;
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

        std::process::Command::new(editor).arg(path).status()?;

        self.resume_terminal()?;
        Ok(())
    }
}
