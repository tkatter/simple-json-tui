use crate::{
    App, CurrentScreen,
    ratatui::{
        Frame,
        layout::{Alignment, Constraint, Flex, Layout, Rect},
        style::{Style, Stylize},
        text::Text,
        widgets::{Block, Borders, Clear, HighlightSpacing, List, ListItem, block::BorderType},
    },
    ui::ColorScheme,
};

pub(crate) fn render_selection_list(frame: &mut Frame<'_>, app: &mut App) {
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
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(
            Style::new()
                .fg(ColorScheme::Flamingo.v())
                .bg(ColorScheme::Mantle.v()),
        );

    // Create the List widget with the items and styles
    let list = List::new(list_items)
        .highlight_style(
            Style::new()
                // .bg(ColorScheme::Flamingo.v())
                .fg(ColorScheme::Green.v()),
        )
        // .highlight_symbol(">> ")
        .highlight_symbol("\u{1F836} ")
        .highlight_spacing(HighlightSpacing::Always)
        .repeat_highlight_symbol(true);

    // Create the popup area centered in the frame
    let select_block_area = center(
        frame.area(),
        Constraint::Percentage(15),
        Constraint::Length((&list.len() + 4) as u16),
    );

    // Create the selection area centered in the popup area
    let list_area = center(
        select_block_area,
        // Constraint::Length(select_block_area.width),
        Constraint::Percentage(50),
        Constraint::Length(list.len() as u16),
    );

    // Render the completed selection popup
    if let CurrentScreen::Selection = app.current_screen {
        frame.render_widget(Clear, select_block_area);
        frame.render_widget(list_block, select_block_area);
        frame.render_stateful_widget(list, list_area, &mut app.selection_screen.state);
    }
}

// Helper function to center a Rect within a given Rect/area
fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
