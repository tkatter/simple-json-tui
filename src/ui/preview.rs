use ratatui::{
    layout::Alignment,
    widgets::{Block, BorderType, Borders, Padding},
};

use crate::{
    app::CurrentScreen,
    ui::{
        ratatui::{
            Frame,
            layout::Rect,
            style::Style,
            text::{Line, Span},
            widgets::{List, ListItem},
        },
        theme::ColorScheme,
    },
};

use crate::app::App;

pub fn render_preview_json(frame: &mut Frame<'_>, app: &mut App, area: Rect) {
    let mut list_items = Vec::<ListItem>::new();

    let serialized_preview = serde_json::to_string_pretty(&app.pairs).unwrap();

    // Iterate through each line of the Serialized JSON
    // to create a ratatui::ListItem
    for (i, line) in serialized_preview.lines().enumerate() {
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

    let preview_block = Block::bordered()
        .title_top(" Preview ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(ColorScheme::Sky.v()))
        .padding(Padding::left(1));

    let list = List::new(list_items).block(preview_block);

    match app.current_screen {
        CurrentScreen::Start => {}
        _ => {
            if !app.pairs.is_empty() {
                frame.render_widget(list, area);
            }
        }
    }
}
