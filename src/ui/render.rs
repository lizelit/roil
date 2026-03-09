use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};
use unicode_width::UnicodeWidthStr;

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App, mode: &crate::ui::mode::CurrentMode) {
    let area = frame.size();

    let [main, status] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .areas(area);

    frame.render_widget(buffer_widget(app), main);
    frame.render_widget(status_widget(app, mode), status);

    draw_cursor(frame, main, app);
}

fn buffer_widget(app: &App) -> Paragraph<'_> {
    let text = app
        .buffer
        .lines()
        .iter()
        .map(|l| l.name.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    Paragraph::new(text).block(Block::default().borders(Borders::ALL))
}

fn status_widget<'a>(app: &'a App, mode: &crate::ui::mode::CurrentMode) -> Paragraph<'a> {
    let mut text = String::new();

    if *mode == crate::ui::mode::CurrentMode::Command {
        text.push_str(&format!(":{} ", app.command_buffer));
    } else {
        let mode_str = match mode {
            crate::ui::mode::CurrentMode::Normal => "-- NORMAL --",
            crate::ui::mode::CurrentMode::Insert => "-- INSERT --",
            _ => unreachable!(),
        };
        let c = app.buffer.cursor();
        text.push_str(&format!(" {}:{} ", c.row + 1, c.col + 1));
        text.push_str(mode_str);
    }

    if app.state() == crate::app::AppState::Error
        && let Some(msg) = app.error_message()
    {
        text.push_str(&format!("| ERROR: {} ", msg));
    }

    Paragraph::new(text)
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
