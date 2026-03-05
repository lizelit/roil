use crossterm::event::{Event as CEvent, KeyCode, read};

#[derive(Debug)]
pub enum Event {
    MoveUp,
    MoveDown,
    EnterInsert,
    ExitInsert,
    InsertChar(char),
    Save,
    Quit,
    None,
}

pub fn read_event() -> anyhow::Result<Event> {
    match read()? {
        CEvent::Key(key) => match key.code {
            KeyCode::Char('j') => Ok(Event::MoveDown),
            KeyCode::Char('k') => Ok(Event::MoveUp),
            KeyCode::Char('i') => Ok(Event::EnterInsert),
            KeyCode::Esc => Ok(Event::ExitInsert),
            KeyCode::Char('q') => Ok(Event::Quit),
            KeyCode::Char('w') => Ok(Event::Save),
        },
    }
}
