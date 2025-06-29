use crate::{
    App, CurrentlyEditing, ValueType,
    ratatui::{
        Frame,
        layout::{Alignment, Constraint, Layout, Rect},
        style::{Style, Stylize},
        text::{Line, Span},
        widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    },
    traits::UpdateMap,
    ui::ColorScheme,
};

const DEFAULT_BORDERSTYLE: ColorScheme = ColorScheme::Sky;
const ACTIVE_BORDERSTYLE: ColorScheme = ColorScheme::Yellow;

pub fn render_editing(frame: &mut Frame<'_>, app: &mut App, area: Rect) {
    // LAYOUT
    let main_editing_layout =
        Layout::vertical(vec![Constraint::Length(3), Constraint::Min(1)]).split(area);

    let kv_area = Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_editing_layout[0]);

    let parsed_preview = &app.editing_preview.parse();
    let preview_list = render_editing_preview(parsed_preview);

    // DRAW EDITING SCREEN BASED ON APP STATE
    frame.render_widget(
        input_box(
            CurrentlyEditing::Key,
            &app.currently_editing,
            &app.value_type,
            &app.key_input,
        ),
        kv_area[0],
    );
    frame.render_widget(
        input_box(
            CurrentlyEditing::Value,
            &app.currently_editing,
            &app.value_type,
            &app.value_input,
        ),
        kv_area[1],
    );
    frame.render_widget(preview_list, main_editing_layout[1]);
}

fn render_editing_preview(parsed: &str) -> List<'_> {
    // PREVIEW BLOCK
    let input_preview = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(DEFAULT_BORDERSTYLE.v()))
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
    active: &Option<CurrentlyEditing>,
    cur_type: &ValueType,
    text: &'a str,
) -> Paragraph<'a> {
    let (value_input, value_color) = if let ValueType::Bool(x) = cur_type {
        match x {
            true => ("true", ColorScheme::Blue.v()),
            false => ("false", ColorScheme::Red.v()),
        }
    } else {
        (text, ColorScheme::Green.v())
    };
    let key_block = Block::new()
        .title(Line::from(vec![
            Span::styled(" Key ", Style::default()).bold(),
        ]))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(match active {
            Some(CurrentlyEditing::Key) => ACTIVE_BORDERSTYLE.v(),
            _ => DEFAULT_BORDERSTYLE.v(),
        }))
        .style(Style::new().fg(ColorScheme::Green.v()));

    let value_block = Block::new()
        .title(Line::from(vec![
            Span::styled(" Value", Style::default()).bold(),
            Span::styled(" - ", Style::default()),
            Span::styled(
                match cur_type {
                    ValueType::String => "String ",
                    ValueType::Number => "Number ",
                    ValueType::Bool(_) => "Boolean ",
                    ValueType::Object => "Object ",
                    ValueType::Array => "Array ",
                },
                Style::new().fg(ColorScheme::Red.v()),
            ),
        ]))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(match active {
            Some(CurrentlyEditing::Value) => ACTIVE_BORDERSTYLE.v(),
            _ => DEFAULT_BORDERSTYLE.v(),
        }))
        .style(Style::new().fg(value_color));

    match cur_editing {
        CurrentlyEditing::Key => Paragraph::new(text).block(key_block),
        CurrentlyEditing::Value => Paragraph::new(value_input).block(value_block),
    }
}
