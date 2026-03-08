pub mod command;
mod event;
pub mod mode;
mod render;

use command::{Action, map_event};
use event::read_event;
use mode::{CurrentMode, TargetMode};
use ratatui::{Terminal, backend::CrosstermBackend};
use render::render;

use std::io::Stdout;

pub use self::command::{Command, Direction};
pub use mode::InsertKind;

use crate::app::App;

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

            self.terminal.draw(|frame| {
                render(frame, app, &self.mode);
            })?;

            if let Some(event) = read_event() {
                let Some(action) = map_event(&self.mode, &mut self.pending_keys, event) else {
                    continue;
                };
                if !self.dispatch(app, action) {
                    break;
                }
            }
        }
        Ok(())
    }

    fn dispatch(&mut self, app: &mut App, action: Action) -> bool {
        match action {
            Action::Command(cmd) => self.handle_command(app, cmd),
            Action::ChangeMode(target) => {
                self.change_mode(app, target);
                true
            }
        }
    }

    fn handle_command(&mut self, app: &mut App, cmd: Command) -> bool {
        match cmd {
            Command::Quit => false,
            Command::Execute => {
                app.execute_cmd();
                if app.state() != crate::app::AppState::Exiting {
                    self.change_mode(app, TargetMode::Normal);
                }
                true
            }
            _ => {
                app.execute(cmd);
                true
            }
        }
    }

    fn change_mode(&mut self, app: &mut App, target: TargetMode) {
        if matches!(target, TargetMode::Normal) {
            app.command_buffer.clear();
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
}
