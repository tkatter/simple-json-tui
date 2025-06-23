use crate::ui::{
    ratatui::{
        Frame,
        layout::Rect,
        style::Style,
        text::{Line, Span},
        widgets::{List, ListItem},
    },
    theme::ColorScheme,
};

use crate::app::App;

pub fn render_preview_json(frame: &mut Frame<'_>, app: &mut App, chunks: &std::rc::Rc<[Rect]>) {
    let mut list_items = Vec::<ListItem>::new();

    let serialized_preview = serde_json::to_string_pretty(&app.pairs).unwrap();

    for (i, line) in serialized_preview.lines().enumerate() {
        let mut line_num: String = format!("{}", i + 1);
        if i < 9 {
            line_num = format!(" {}", i + 1);
        }
        list_items.push(ListItem::new(Line::from(vec![
            Span::styled(
                line_num,
                Style::new()
                    .fg(ColorScheme::Surface1.v())
                    .bg(ColorScheme::Mantle.v()),
            ),
            Span::raw("  "),
            Span::styled(line, Style::new().fg(ColorScheme::Peach.v())),
        ])));
    }

    let list = List::new(list_items);

    frame.render_widget(list, chunks[1]);
}
