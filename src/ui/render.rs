use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};

use crate::app::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let size = frame.size();

    let items: Vec<ListItem> = app
        .buffer
        .lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let prefix = if i == app.buffer.cursor.row {
                "> "
            } else {
                "  "
            };

            ListItem::new(format!("{}{}", prefix, line.name))
        })
        .collect();

    let list = List::new(items).block(Block::default().title("Files").borders(Borders::ALL));

    frame.render_widget(list, size);
}
