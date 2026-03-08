use crossterm::event::{self, Event, KeyCode, KeyEventKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiEvent {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Esc,
    Char(char),
    Backspace,
}

pub fn from_crossterm(event: Event) -> Option<UiEvent> {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Up => Some(UiEvent::Up),
            KeyCode::Down => Some(UiEvent::Down),
            KeyCode::Left => Some(UiEvent::Left),
            KeyCode::Right => Some(UiEvent::Right),
            KeyCode::Enter => Some(UiEvent::Enter),
            KeyCode::Esc => Some(UiEvent::Esc),
            KeyCode::Backspace => Some(UiEvent::Backspace),
            KeyCode::Char(c) => Some(UiEvent::Char(c)),
            _ => None,
        },
        _ => None,
    }
}

pub fn read_event() -> Option<UiEvent> {
    let event = event::read().ok()?;
    from_crossterm(event)
}
