use ratatui::{
    layout::Alignment,
    widgets::{List, ListItem, Paragraph},
};

use crate::ui::ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders},
};

use crate::{
    app::{App, CurrentlyEditing, ValueType},
    ui::theme::ColorScheme,
};
const BORDERSTYLE: ColorScheme = ColorScheme::Sky;
pub fn render_editing(frame: &mut Frame<'_>, app: &mut App, area: Rect) {
    // LAYOUT
    let default_editing_layout =
        Layout::vertical(vec![Constraint::Length(3), Constraint::Min(1)]).split(area);

    let object_kv_area =
        Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(default_editing_layout[0]);

    let parsed_preview = &app.editing_preview.parse();
    let preview_list = render_editing_preview(parsed_preview);

    // DRAW EDITING SCREEN BASED ON APP STATE
    if app.editing_object {
        frame.render_widget(
            input_box(CurrentlyEditing::Key, &app.value_type, &app.key_input),
            object_kv_area[0],
        );
        frame.render_widget(
            input_box(CurrentlyEditing::Value, &app.value_type, &app.value_input),
            object_kv_area[1],
        );
        frame.render_widget(preview_list, default_editing_layout[1]);
    } else {
        default_editing_screen(frame, app, default_editing_layout, &preview_list);
    }
}

fn default_editing_screen(
    frame: &mut Frame<'_>,
    app: &mut App,
    default_editing_layout: std::rc::Rc<[Rect]>,
    preview_list: &List<'_>,
) {
    if let Some(state) = &app.currently_editing {
        match state {
            CurrentlyEditing::Key => {
                frame.render_widget(
                    input_box(CurrentlyEditing::Key, &app.value_type, &app.key_input),
                    default_editing_layout[0],
                );
            }
            CurrentlyEditing::Value => {
                frame.render_widget(
                    input_box(CurrentlyEditing::Value, &app.value_type, &app.value_input),
                    default_editing_layout[0],
                );
            }
        }
        frame.render_widget(preview_list, default_editing_layout[1]);
    } else if !app.pairs.is_empty() {
        frame.render_widget(
            input_box(CurrentlyEditing::default(), &ValueType::default(), ""),
            default_editing_layout[0],
        );
        frame.render_widget(preview_list, default_editing_layout[1]);
    }
}

fn render_editing_preview(parsed: &str) -> List<'_> {
    // PREVIEW BLOCK
    let input_preview = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(BORDERSTYLE.v()))
        .style(Style::new().fg(ColorScheme::Green.v()));

    // CREATE PREVIEW FROM `serde_json::to_string_pretty()`
    let mut list_items = Vec::<ListItem>::new();
    let parsed_string = parsed;
    for (i, line) in parsed_string.lines().enumerate() {
        // Create line numbers
        let mut line_num: String = format!("{}", i + 1);
        if i < 9 {
            line_num = format!(" {}", i + 1);
        }

        // Create line from preview key/value
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

    List::new(list_items).block(input_preview)
}

fn input_box<'a>(
    cur_editing: CurrentlyEditing,
    cur_type: &ValueType,
    text: &'a str,
) -> Paragraph<'a> {
    let key_block = Block::new()
        .title(" Key ")
        .bold()
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(BORDERSTYLE.v()))
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
        .border_style(Style::new().fg(BORDERSTYLE.v()))
        .style(Style::new().fg(ColorScheme::Green.v()));

    let input_val = Paragraph::new(text);
    match cur_editing {
        CurrentlyEditing::Key => input_val.block(key_block),
        CurrentlyEditing::Value => input_val.block(value_block),
    }
}
