use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use unicode_width::UnicodeWidthChar;

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App, mode: &crate::ui::mode::CurrentMode) {
    let area = frame.size();

    let [main, status] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .areas(area);

    frame.render_widget(buffer_widget(app), main);

    status_widget(frame, status, app, mode);

    draw_cursor(frame, main, app);
}

fn buffer_widget(app: &App) -> Paragraph<'_> {
    use crate::ui::{classify::classify, icon::icon};

    let lines: Vec<Line> = app
        .buffer()
        .lines()
        .iter()
        .skip(app.scroll_offset())
        .take(app.view_height())
        .map(|line| {
            let entry = line.to_entry(&app.buffer().parent());
            let kind = classify(&entry);
            let icon = icon(kind);

            Line::from(vec![
                Span::raw(icon),
                Span::raw(" "),
                Span::raw(line.name()),
            ])
        })
        .collect();

    Paragraph::new(lines).block(Block::default().borders(Borders::ALL))
}

fn status_widget<'a>(frame: &mut Frame, area: Rect, app: &'a App, mode: &super::mode::CurrentMode) {
    use super::mode::CurrentMode;
    let [left, right] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Length(20)])
        .areas(area);

    let left_content = match mode {
        CurrentMode::Command => format!(":{} ", app.command_buffer()),
        _ => format!(" -- {:?} -- ", mode).to_ascii_uppercase(),
    };
    frame.render_widget(Paragraph::new(left_content), left);

    let cursor = app.buffer().cursor();
    let right_content = format!("{}:{}", cursor.row() + 1, cursor.col() + 1);
    frame.render_widget(
        Paragraph::new(right_content).alignment(Alignment::Right),
        right,
    );
}

fn draw_cursor(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let inner_area = area.inner(&ratatui::layout::Margin {
        horizontal: 1,
        vertical: 1,
    });

    let cursor = app.buffer().cursor();
    let relative_row = cursor.row().saturating_sub(app.scroll_offset());

    if relative_row >= app.view_height() {
        return;
    }

    let Some(line) = app.buffer().line(cursor.row()) else {
        return;
    };

    const ICON_WIDTH: usize = 2;
    let text_column: usize = line
        .name()
        .chars()
        .take(cursor.col())
        .filter_map(|c| c.width())
        .sum();
    let total_col_offset = (text_column + ICON_WIDTH) as u16;

    frame.set_cursor(
        inner_area.x + total_col_offset,
        inner_area.y + relative_row as u16,
    );
}
