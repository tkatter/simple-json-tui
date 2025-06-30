use crate::{
    ratatui::{
        buffer::Buffer,
        layout::{Constraint, Rect},
        style::{Modifier, Style},
        text::{Line, Text},
        widgets::{Block, BorderType, Borders, Clear, List, Padding, Paragraph, Widget},
    },
    ui::{ColorScheme, helpers::center},
};

pub struct Header {
    content: String,
}

impl Header {
    pub fn new(content: &str) -> Header {
        Header {
            content: content.to_string(),
        }
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

/// Renders an empty popup block
/// in the center of the entire terminal frame.
///
/// Provides a padded area within the empty area
/// for a list to be rendered.
pub struct SelectionPopUp<'a> {
    list: &'a List<'a>,
    list_block: Block<'a>,
    percentage: u16,
}

impl<'a> SelectionPopUp<'a> {
    pub fn new(list: &'a List, list_block: Block<'a>, x_percent: u16) -> SelectionPopUp<'a> {
        SelectionPopUp {
            list,
            list_block,
            percentage: x_percent,
        }
    }

    pub fn get_list_areas(&self, area: Rect) -> (Rect, Rect) {
        let popup_outer_area = center(
            area,
            Constraint::Percentage(self.percentage),
            Constraint::Length((self.list.len() + 4) as u16),
        );

        let list_area = center(
            popup_outer_area,
            Constraint::Percentage(50),
            Constraint::Length(self.list.len() as u16),
        );

        (popup_outer_area, list_area)
    }
}

impl Widget for SelectionPopUp<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let (popup_outer_area, _) = self.get_list_areas(area);

        Clear.render(popup_outer_area, buf);
        self.list_block.render(popup_outer_area, buf);
    }
}
