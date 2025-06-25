use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget},
};

use crate::ui::theme::ColorScheme;

pub struct Header {
    content: String,
}

impl Header {
    pub fn new(content: String) -> Header {
        Header { content }
    }
}

impl Widget for Header {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header_block = Block::default()
            .padding(Padding::horizontal(4))
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .style(
                Style::new()
                    .bg(ColorScheme::Crust.v())
                    .fg(ColorScheme::Pink.v()),
            );

        let header_line = Line::styled(
            &self.content,
            Style::new()
                .fg(ColorScheme::Mauve.v())
                .add_modifier(Modifier::BOLD),
        )
        .centered();

        let header = Paragraph::new(Text::from(header_line)).block(header_block);

        header.render(area, buf);
    }
}
