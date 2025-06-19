use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::{
    app::{App, CurrentlyEditing, ValueType},
    ui::theme::ColorScheme,
};

pub struct AppTitle {
    content: String,
}

impl AppTitle {
    pub fn new(content: String) -> AppTitle {
        AppTitle { content }
    }
}

impl Widget for AppTitle {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title_block = Block::default()
            .borders(Borders::BOTTOM)
            .style(Style::new().bg(ColorScheme::Base.v()));

        let title_line = Line::styled(
            &self.content,
            Style::new()
                .fg(ColorScheme::Peach.v())
                .add_modifier(Modifier::BOLD),
        )
        .centered();

        let title = Paragraph::new(Text::from(vec![
            Line::default(),
            title_line,
            Line::default(),
        ]))
        .block(title_block);

        title.render(area, buf);
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
