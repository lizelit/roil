use crate::ui::command::{Command, map_event};
use crate::ui::event::UiEvent;
use crate::ui::mode::Mode;

pub struct Ui {
    mode: Mode,
    cursor: usize,
}
