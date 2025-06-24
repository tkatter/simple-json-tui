use ratatui::{
    layout::Alignment,
    widgets::{List, ListItem, Paragraph},
};

use crate::ui::ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders},
};

use crate::{
    app::{App, CurrentlyEditing, ValueType},
    ui::theme::ColorScheme,
};

pub fn render_editing(frame: &mut Frame<'_>, app: &mut App, area: Rect) {
    let editing_layout =
        Layout::vertical(vec![Constraint::Length(3), Constraint::Min(1)]).split(area);

    let input_preview = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(Color::White))
        .style(Style::new().fg(ColorScheme::Green.v()));

    let mut list_items = Vec::<ListItem>::new();
    let parsed_string = app.editing_preview.parse();
    for (i, line) in parsed_string.lines().enumerate() {
        let mut line_num: String = format!("{}", i + 1);
        if i < 9 {
            line_num = format!(" {}", i + 1);
        }
        list_items.push(ListItem::new(Line::from(vec![
            Span::styled(
                line_num,
                Style::new()
                    .fg(ColorScheme::Overlay1.v())
                    .bg(ColorScheme::Mantle.v()),
            ),
            Span::raw("  "),
            Span::styled(line, Style::new().fg(ColorScheme::Peach.v())),
        ])));
    }

    let list = List::new(list_items).block(input_preview);

    if let Some(state) = &app.currently_editing {
        match state {
            CurrentlyEditing::Key => {
                frame.render_widget(
                    input_box(CurrentlyEditing::Key, &app.value_type, &app.key_input),
                    editing_layout[0],
                );
            }
            CurrentlyEditing::Value => {
                frame.render_widget(
                    input_box(CurrentlyEditing::Value, &app.value_type, &app.value_input),
                    editing_layout[0],
                );
            }
        }
        frame.render_widget(list, editing_layout[1]);
    } else if !app.pairs.is_empty() {
        frame.render_widget(
            input_box(
                CurrentlyEditing::default(),
                &ValueType::default(),
                &String::from(""),
            ),
            editing_layout[0],
        );
        frame.render_widget(list, editing_layout[1]);
    }
}
fn input_box<'a>(
    cur_editing: CurrentlyEditing,
    cur_type: &ValueType,
    text: &String,
) -> Paragraph<'a> {
    let key_block = Block::new()
        .title(" Key ")
        .bold()
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(Color::White))
        .style(Style::new().fg(ColorScheme::Green.v()));

    let value_block = Block::new()
        .title(Line::from(vec![
            Span::styled(" Value", Style::default()),
            Span::styled(" - ", Style::default()),
            Span::styled(
                match cur_type {
                    ValueType::String => "String ",
                    ValueType::Number => "Number ",
                    ValueType::Bool => "Boolean ",
                    ValueType::Object => "Object ",
                    ValueType::Array => "Array ",
                },
                Style::new().fg(ColorScheme::Red.v()),
            ),
        ]))
        .bold()
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(Color::White))
        .style(Style::new().fg(ColorScheme::Green.v()));

    let input_val = Paragraph::new(text.to_owned());
    match cur_editing {
        CurrentlyEditing::Key => input_val.block(key_block),
        CurrentlyEditing::Value => input_val.block(value_block),
    }
}
