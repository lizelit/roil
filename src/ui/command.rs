use super::mode::TargetMode;

use super::event::UiEvent;
use super::mode::{CurrentMode, InsertKind};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub enum Action {
    Command(Command),
    ChangeMode(TargetMode),
}

#[derive(Debug, Clone)]
pub enum Command {
    Move(Direction),
    InsertChar(char),
    DeleteChar,
    DeleteEntry,
    Undo,
    Redo,
    Save,
    Quit,
}

pub fn map_event(mode: &CurrentMode, event: UiEvent) -> Option<Action> {
    match mode {
        CurrentMode::Normal => normal_mode(event),
        CurrentMode::Insert => insert_mode(event),
        CurrentMode::Command => command_mode(event),
    }
}

fn normal_mode(event: UiEvent) -> Option<Action> {
    match event {
        UiEvent::Char('h') => Some(Action::Command(Command::Move(Direction::Left))),
        UiEvent::Char('j') => Some(Action::Command(Command::Move(Direction::Down))),
        UiEvent::Char('k') => Some(Action::Command(Command::Move(Direction::Up))),
        UiEvent::Char('l') => Some(Action::Command(Command::Move(Direction::Right))),
        UiEvent::Char('u') => Some(Action::Command(Command::Undo)),
        UiEvent::Char('U') => Some(Action::Command(Command::Redo)),
        UiEvent::Char('d') => Some(Action::Command(Command::DeleteEntry)),
        UiEvent::Char('i') => Some(Action::ChangeMode(TargetMode::Insert(
            InsertKind::BeforeCursor,
        ))),
        UiEvent::Char(':') => Some(Action::ChangeMode(TargetMode::Command)),

        _ => None,
    }
}
fn insert_mode(event: UiEvent) -> Option<Action> {
    match event {
        UiEvent::Esc => Some(Action::ChangeMode(TargetMode::Normal)),
        UiEvent::Char(c) => Some(Action::Command(Command::InsertChar(c))),
        UiEvent::Backspace => Some(Action::Command(Command::DeleteChar)),
        _ => None,
    }
}
fn command_mode(event: UiEvent) -> Option<Action> {
    match event {
        UiEvent::Esc => Some(Action::ChangeMode(TargetMode::Normal)),
        UiEvent::Enter => Some(Action::Command(Command::Save)),
        _ => None,
    }
}
