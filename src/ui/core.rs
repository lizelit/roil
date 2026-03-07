use crate::ui::CurrentMode;

pub struct Ui {
    mode: CurrentMode,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            mode: CurrentMode::Normal,
        }
    }

    pub fn mode(&self) -> CurrentMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: CurrentMode) {
        self.mode = mode;
    }
}
