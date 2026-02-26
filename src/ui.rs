use crate::app::{App, AppMode};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(area);

        let body_area = main_chunks[0];
        let footer_area = main_chunks[1];

        self.render_file_list(body_area, buf);

        self.render_footer(footer_area, buf);
    }
}

impl App {
    fn render_file_list(&self, area: Rect, buf: &mut Buffer) {
        let list_block = Block::default()
            .borders(Borders::ALL)
            .title(" File Explorer (Batch Rename) ");

        let inner_area = list_block.inner(area);
        list_block.render(area, buf);

        for (i, item) in self.current_items.iter().enumerate() {
            if i as u16 >= inner_area.height {
                break;
            }

            let row_rect = Rect::new(inner_area.x, inner_area.y + i as u16, inner_area.width, 1);

            let row_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(row_rect);

            let icon_style = if i == self.cursor_index {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Cyan)
            };

            Paragraph::new(format!(" {} ", item.kind.icon()))
                .style(icon_style)
                .render(row_chunks[0], buf);

            self.edit_buffers[i].widget().render(row_chunks[1], buf);
        }
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        match self.mode {
            AppMode::Command => {
                let mut cmd = self.command_line.clone();
                cmd.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Cyan))
                        .title(" Command (e.g. :w, :q) "),
                );
                cmd.widget().render(area, buf);
            }
            _ => {
                let mode_name = format!("{:?}", self.mode).to_uppercase();
                let status_text = format!(
                    " MODE: {} | [j/k] Move | [i/Enter] Edit | [:] Cmd ",
                    mode_name
                );

                let status_style = match self.mode {
                    AppMode::Insert => Style::default().fg(Color::Yellow),
                    _ => Style::default().fg(Color::Green),
                };

                Paragraph::new(status_text)
                    .style(status_style)
                    .block(Block::default().borders(Borders::ALL).title(" Status "))
                    .render(area, buf);
            }
        }
    }
}
