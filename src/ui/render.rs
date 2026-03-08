use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};
use unicode_width::UnicodeWidthStr;

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.size();

    let [main, status] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .areas(area);

    frame.render_widget(buffer_widget(app), main);
    frame.render_widget(status_widget(app), status);

    draw_cursor(frame, main, app);
}

fn buffer_widget(app: &App) -> Paragraph {
    let text = app
        .buffer
        .lines()
        .iter()
        .map(|l| l.name.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    Paragraph::new(text).block(Block::default().borders(Borders::ALL))
}

fn status_widget(app: &App) -> Paragraph {
    let c = app.buffer.cursor();
    let text = format!(" row:{} col:{} ", c.row + 1, c.col + 1);

    Paragraph::new(text).block(Block::default().borders(Borders::TOP))
}

fn draw_cursor(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let cursor = app.buffer.cursor();

    let Some(line) = app.buffer.line(cursor.row) else {
        return;
    };

    let col = line
        .name
        .chars()
        .take(cursor.col)
        .map(|c| c.to_string().width())
        .sum::<usize>();

    frame.set_cursor(area.x + 1 + col as u16, area.y + 1 + cursor.row as u16);
}
