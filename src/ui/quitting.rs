use crate::{
    App, CurrentScreen,
    file_state::FileState,
    ratatui::{
        Frame,
        style::Style,
        text::{Line, Span},
        widgets::{Block, BorderType, HighlightSpacing, List, ListItem},
    },
    ui::{components::SelectionPopUp, theme::ColorScheme},
};

pub fn render_quitting_screen(frame: &mut Frame<'_>, app: &mut App, file_state: &FileState) {
    let mut list_items: Vec<ListItem> = Vec::new();

    let save_option: Vec<Line> = if !file_state.fname_input.is_empty() {
        vec![
            Line::from(vec![
                Span::from("Y"),
                Span::raw("  "),
                Span::from(format!("Quit and save to {}", file_state.fname_input)),
            ])
            .style(Style::new().fg(ColorScheme::Green.v())),
            Line::default(),
            Line::from(vec![
                Span::from("N"),
                Span::raw("  "),
                Span::from("Quit without saving"),
            ])
            .style(Style::new().fg(ColorScheme::Red.v())),
        ]
    } else {
        vec![
            Line::from(vec![
                Span::from("Y"),
                Span::raw("  "),
                Span::from("Quit and print buffer as stdout"),
            ])
            .style(Style::new().fg(ColorScheme::Green.v())),
            Line::default(),
            Line::from(vec![
                Span::from("F"),
                Span::raw("  "),
                Span::from("Save to a file"),
            ])
            .style(Style::new().fg(ColorScheme::Yellow.v())),
            Line::default(),
            Line::from(vec![
                Span::from("N"),
                Span::raw("  "),
                Span::from("Quit without saving"),
            ])
            .style(Style::new().fg(ColorScheme::Red.v())),
        ]
    };

    // Create list items dependent on whether the user
    // created a file in the beginning or not
    for item in save_option.into_iter() {
        let list_item: ListItem = ListItem::new(item);

        list_items.push(list_item);
    }

    let popup_outer_block = Block::bordered().border_type(BorderType::Rounded);

    let list = List::new(list_items)
        .highlight_style(Style::new().fg(ColorScheme::Green.v()))
        .highlight_symbol("\u{1F836} ")
        .highlight_spacing(HighlightSpacing::Always)
        .repeat_highlight_symbol(true);

    let popup = SelectionPopUp::new(&list, popup_outer_block, 20);

    let (_, list_area) = popup.get_list_areas(frame.area());

    if let CurrentScreen::Quitting = app.current_screen {
        frame.render_widget(popup, frame.area());
        frame.render_stateful_widget(list, list_area, &mut app.quitting_screen.state);
    }
}
