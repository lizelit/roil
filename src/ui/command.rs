use super::event::UiEvent;
use super::mode::Mode;

#[derive(Debug, Clone)]
pub enum Command {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    InsertChar(char),
    DeleteChar,

    EnterInsertMode,
    EnterNormalMode,
    EnterCommandMode,

    DeleteEntry,
    Save,
}

pub fn map_event(mode: Mode, event: UiEvent) -> Option<Command> {
    match mode {
        Mode::Normal => normal_mode(event),
        Mode::Insert => insert_mode(event),
        Mode::Command => command_mode(event),
    }
}

fn normal_mode(event: UiEvent) -> Option<Command> {
    match event {
        UiEvent::Char('h') => Some(Command::MoveLeft),
        UiEvent::Char('j') => Some(Command::MoveDown),
        UiEvent::Char('k') => Some(Command::MoveUp),
        UiEvent::Char('l') => Some(Command::MoveRight),

        UiEvent::Char('i') => Some(Command::EnterInsertMode),
        UiEvent::Char(':') => Some(Command::EnterCommandMode),

        UiEvent::Char('d') => Some(Command::DeleteEntry),

        _ => None,
    }
}
fn insert_mode(event: UiEvent) -> Option<Command> {
    match event {
        UiEvent::Esc => Some(Command::EnterNormalMode),

        UiEvent::Char(c) => Some(Command::InsertChar(c)),

        UiEvent::Backspace => Some(Command::DeleteChar),

        _ => None,
    }
}
fn command_mode(event: UiEvent) -> Option<Command> {
    match event {
        UiEvent::Esc => Some(Command::EnterNormalMode),
        UiEvent::Enter => Some(Command::Save),
        _ => None,
    }
}
