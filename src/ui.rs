mod command;
mod event;
mod mode;
mod render;

use command::{Action, map_event};
use event::read_event;
use mode::{CurrentMode, InsertKind, TargetMode};
use ratatui::{Terminal, backend::CrosstermBackend};
use render::render;

use std::io::Stdout;

pub use command::{Command, Direction};

use crate::app::App;

pub struct Ui {
    mode: CurrentMode,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Ui {
    pub fn new(terminal: Terminal<CrosstermBackend<Stdout>>) -> Self {
        Self {
            mode: CurrentMode::Normal,
            terminal,
        }
    }

    pub fn run(&mut self, app: &mut App) -> std::io::Result<()> {
        loop {
            self.terminal.draw(|frame| {
                render(frame, app);
            })?;

            if let Some(event) = read_event() {
                if let Some(action) = map_event(&self.mode, event) {
                    if !self.dispatch(app, action) {
                        break;
                    }
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
            _ => {
                app.execute(cmd);
                true
            }
        }
    }

    fn change_mode(&mut self, _app: &mut App, target: TargetMode) {
        match target {
            TargetMode::Normal => {
                self.mode = CurrentMode::Normal;
            }
            TargetMode::Insert(kind) => match kind {
                InsertKind::AfterCursor => unimplemented!(),
                // same above
                _ => unimplemented!(),
            },
            TargetMode::Command => {
                self.mode = CurrentMode::Command;
            }
        }
    }
    pub fn mode(&self) -> CurrentMode {
        self.mode
    }
}
