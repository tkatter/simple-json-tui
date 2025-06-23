use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget},
};

use crate::{
    app::{App, CurrentlyEditing, ValueType},
    ui::theme::ColorScheme,
};

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
        // let header_layout = Layout::horizontal(vec![
        //     Constraint::Min(1),
        //     Constraint::Min(1),
        //     Constraint::Min(1),
        // ])
        // .split(area);

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

pub enum EditingId {
    Key,
    Value,
}

pub struct EditingBox {
    id: EditingId,
    input_text: String,
    state: Option<CurrentlyEditing>,
    value_type: ValueType,
}

impl EditingBox {
    pub fn new(id: EditingId, app: &App, editing: &CurrentlyEditing) -> EditingBox {
        let state = match editing {
            CurrentlyEditing::Key => Some(CurrentlyEditing::Key),
            CurrentlyEditing::Value => Some(CurrentlyEditing::Value),
        };

        let input_text = match id {
            EditingId::Key => app.key_input.clone(),
            EditingId::Value => app.value_input.clone(),
        };

        let value_type = app.value_type.clone();

        EditingBox {
            id,
            input_text,
            state,
            value_type,
        }
    }
}

impl Widget for EditingBox {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut block = Block::bordered()
            .title(match self.id {
                EditingId::Key => Line::from("Key").centered(),
                EditingId::Value => Line::from(vec![
                    Span::styled("Value", Style::default()),
                    Span::styled(" - ", Style::default()),
                    Span::styled(
                        match self.value_type {
                            ValueType::String => "string",
                            ValueType::Number => "number",
                            ValueType::Bool => "boolean",
                            ValueType::Object => "object",
                            ValueType::Array => "array",
                        },
                        Style::new().fg(match self.state {
                            Some(CurrentlyEditing::Key) => ColorScheme::Red.v(),
                            Some(CurrentlyEditing::Value) => ColorScheme::Red.v(),
                            _ => ColorScheme::Red.v(),
                        }),
                    ),
                ])
                .centered(),
            })
            .borders(Borders::ALL)
            .border_type(BorderType::Thick);

        let active_style = Style::default().fg(ColorScheme::Peach.v());

        match self.state {
            Some(CurrentlyEditing::Key) => match self.id {
                EditingId::Key => block = block.style(active_style).border_type(BorderType::Double),
                _ => block = block.style(Style::default()),
            },
            Some(CurrentlyEditing::Value) => match self.id {
                EditingId::Value => {
                    block = block.style(active_style).border_type(BorderType::Double)
                }
                _ => block = block.style(Style::default()),
            },
            None => block = block.style(Style::default()),
        };

        let text = Paragraph::new(self.input_text.to_string()).block(block);

        text.render(area, buf);
    }
}

// pub struct PairsList {}
