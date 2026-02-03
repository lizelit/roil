use crate::app::{App, AppMode};
use ratatui::widgets::block::Block;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Borders, Paragraph, Widget},
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(area);
        let body_area = main_chunks[0];
        let footer_area = main_chunks[1];

        let mut file_list = self.textarea.clone();
        file_list.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("File Explorer"),
        );
        let file_list_widget = file_list.widget();
        file_list_widget.render(body_area, buf);

        match self.mode {
            AppMode::Command => {
                let mut command_line = self.command_line.clone();
                command_line.set_block(Block::default().borders(Borders::ALL).title("Command"));
                let command_line_widget = command_line.widget();
                command_line_widget.render(footer_area, buf);
            }
            _ => {
                let mut status_text = format!(
                    "Mode: {:?} | Press ':' for command, 'i' for insert",
                    self.mode
                );

                if let Some(ref msg) = self.message {
                    status_text = format!("{} | {}", status_text, msg);
                }

                let status_style = Style::default().fg(match self.mode {
                    AppMode::Normal => Color::Green,
                    AppMode::Insert => Color::Yellow,
                    AppMode::Command => Color::Cyan,
                });

                let status_paragraph = Paragraph::new(status_text)
                    .style(status_style)
                    .block(Block::default().borders(Borders::ALL));
                status_paragraph.render(footer_area, buf);
            }
        }
    }
}
