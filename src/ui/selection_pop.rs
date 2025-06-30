use crate::{
    App, CurrentScreen,
    ratatui::{
        Frame,
        layout::Alignment,
        style::{Style, Stylize},
        text::Text,
        widgets::{Block, HighlightSpacing, List, ListItem, block::BorderType},
    },
    ui::{ColorScheme, components::SelectionPopUp},
};

pub fn render_selection_list(frame: &mut Frame<'_>, app: &mut App) {
    let mut list_items: Vec<ListItem> = Vec::new();

    // Create styled ListItems
    for (i, item) in app.selection_screen.options.iter().enumerate() {
        let text: ListItem = Text::styled(
            format!("[{}]    {}", i + 1, *item),
            Style::new().fg(ColorScheme::Lavender.v()),
        )
        .alignment(Alignment::Left)
        .into();
        list_items.push(text);
    }

    // Create a bordered block for the selection popup
    let list_block = Block::bordered()
        .title_top(" Select ")
        .title_style(Style::new().bold())
        .border_type(BorderType::Rounded)
        .style(
            Style::new()
                .fg(ColorScheme::Flamingo.v())
                .bg(ColorScheme::Mantle.v()),
        );

    // Create the List widget with the items and styles
    let list = List::new(list_items)
        .highlight_style(Style::new().fg(ColorScheme::Green.v()))
        .highlight_symbol("\u{1F836} ")
        .highlight_spacing(HighlightSpacing::Always)
        .repeat_highlight_symbol(true);

    let popup = SelectionPopUp::new(&list, list_block, 15);

    let (_, list_area) = popup.get_list_areas(frame.area());

    // Render the completed selection popup
    if let CurrentScreen::Selection = app.current_screen {
        frame.render_widget(popup, frame.area());
        frame.render_stateful_widget(list, list_area, &mut app.selection_screen.state);
    }
}
