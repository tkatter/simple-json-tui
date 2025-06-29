use crate::{
    CurrentScreen,
    ratatui::{
        Frame,
        layout::{Alignment, Rect},
        style::{Color, Style},
        text::{Line, Span},
        widgets::{Block, BorderType, Borders, List, ListItem, Padding},
    },
    ui::ColorScheme,
};

use crate::app::App;

const PRIMARY: ColorScheme = ColorScheme::Peach;
const SECONDARY: ColorScheme = ColorScheme::Mauve;

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

        let special_chars = ['{', '}', '[', ']'];
        let mut color: Color = PRIMARY.v();

        let styled_line: Vec<Span> = if let Some((key, value)) = line.split_once(':') {
            for c in special_chars.iter() {
                if value.contains(*c) {
                    color = SECONDARY.v();
                }
            }
            vec![
                Span::styled(
                    line_num,
                    Style::new()
                        .fg(ColorScheme::Overlay1.v())
                        .bg(ColorScheme::Mantle.v()),
                ),
                Span::raw("  "),
                Span::styled(key, Style::new().fg(ColorScheme::Lavender.v())),
                Span::raw(": "),
                Span::styled(value, Style::new().fg(color)),
            ]
        } else {
            for c in special_chars.iter() {
                if line.contains(*c) {
                    color = SECONDARY.v();
                }
            }
            vec![
                Span::styled(
                    line_num,
                    Style::new()
                        .fg(ColorScheme::Overlay1.v())
                        .bg(ColorScheme::Mantle.v()),
                ),
                Span::raw("  "),
                Span::styled(line, Style::new().fg(color)),
            ]
        };

        list_items.push(ListItem::new(Line::from(styled_line)));
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
