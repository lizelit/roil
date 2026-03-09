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
#[allow(dead_code)]
pub enum Command {
    Move(Direction, usize),
    InsertChar(char),
    InsertNewLine,
    DeleteChar,
    DeleteEntry(usize),
    Undo,
    Redo,
    Quit,
    Input(char),
    Backspace,
    Execute,
}

pub fn map_event(mode: &CurrentMode, pending_keys: &mut String, event: UiEvent) -> Option<Action> {
    match mode {
        CurrentMode::Normal => normal_mode(pending_keys, event),
        CurrentMode::Insert => insert_mode(event),
        CurrentMode::Command => command_mode(event),
    }
}

fn normal_mode(pending_keys: &mut String, event: UiEvent) -> Option<Action> {
    // let UiEvent::Char(c) = event else {
    //     if event == UiEvent::Esc {
    //         pending_keys.clear();
    //     }
    //     return None;
    // };

    // pending_keys.push(c);
    if let UiEvent::Char(c) = event {
        pending_keys.push(c);
    } else {
        let action = match event {
            UiEvent::Left => Some(Action::Command(Command::Move(Direction::Left, 1))),
            UiEvent::Down => Some(Action::Command(Command::Move(Direction::Down, 1))),
            UiEvent::Up => Some(Action::Command(Command::Move(Direction::Up, 1))),
            UiEvent::Right => Some(Action::Command(Command::Move(Direction::Right, 1))),
            _ => None,
        };

        pending_keys.clear();
        return action;
    };

    let mut count_str = String::new();
    let mut cmd_str = String::new();

    for ch in pending_keys.chars() {
        if cmd_str.is_empty() && ch.is_ascii_digit() {
            count_str.push(ch);
        } else {
            cmd_str.push(ch);
        }
    }

    let count: usize = if count_str.is_empty() {
        1
    } else {
        count_str.parse().unwrap_or(1).max(1)
    };

    match cmd_str.as_str() {
        "h" => consume_and_return(
            pending_keys,
            Action::Command(Command::Move(Direction::Left, count)),
        ),
        "j" => consume_and_return(
            pending_keys,
            Action::Command(Command::Move(Direction::Down, count)),
        ),
        "k" => consume_and_return(
            pending_keys,
            Action::Command(Command::Move(Direction::Up, count)),
        ),
        "l" => consume_and_return(
            pending_keys,
            Action::Command(Command::Move(Direction::Right, count)),
        ),
        "u" => consume_and_return(pending_keys, Action::Command(Command::Undo)),
        "U" => consume_and_return(pending_keys, Action::Command(Command::Redo)),
        "dd" => consume_and_return(pending_keys, Action::Command(Command::DeleteEntry(count))),
        "i" => consume_and_return(
            pending_keys,
            Action::ChangeMode(TargetMode::Insert(InsertKind::BeforeCursor)),
        ),
        "a" => consume_and_return(
            pending_keys,
            Action::ChangeMode(TargetMode::Insert(InsertKind::AfterCursor)),
        ),
        "I" => consume_and_return(
            pending_keys,
            Action::ChangeMode(TargetMode::Insert(InsertKind::LineStart)),
        ),
        "A" => consume_and_return(
            pending_keys,
            Action::ChangeMode(TargetMode::Insert(InsertKind::LineEnd)),
        ),
        "o" => consume_and_return(
            pending_keys,
            Action::ChangeMode(TargetMode::Insert(InsertKind::NewLineBelow)),
        ),
        "O" => consume_and_return(
            pending_keys,
            Action::ChangeMode(TargetMode::Insert(InsertKind::NewLineAbove)),
        ),
        ":" => consume_and_return(pending_keys, Action::ChangeMode(TargetMode::Command)),

        "d" => None,

        _ => {
            if cmd_str.is_empty() {
                None
            } else {
                pending_keys.clear();
                None
            }
        }
    }
}

fn consume_and_return(pending_keys: &mut String, action: Action) -> Option<Action> {
    pending_keys.clear();
    Some(action)
}
fn insert_mode(event: UiEvent) -> Option<Action> {
    match event {
        UiEvent::Esc => Some(Action::ChangeMode(TargetMode::Normal)),
        UiEvent::Enter => Some(Action::Command(Command::InsertNewLine)),
        UiEvent::Char(c) => Some(Action::Command(Command::InsertChar(c))),
        UiEvent::Backspace => Some(Action::Command(Command::DeleteChar)),
        UiEvent::Left => Some(Action::Command(Command::Move(Direction::Left, 1))),
        UiEvent::Down => Some(Action::Command(Command::Move(Direction::Down, 1))),
        UiEvent::Up => Some(Action::Command(Command::Move(Direction::Up, 1))),
        UiEvent::Right => Some(Action::Command(Command::Move(Direction::Right, 1))),
        // _ => None,
    }
}
fn command_mode(event: UiEvent) -> Option<Action> {
    match event {
        UiEvent::Esc => Some(Action::ChangeMode(TargetMode::Normal)),
        UiEvent::Enter => Some(Action::Command(Command::Execute)),
        UiEvent::Backspace => Some(Action::Command(Command::Backspace)),
        UiEvent::Char(c) => Some(Action::Command(Command::Input(c))),
        _ => None,
    }
}
