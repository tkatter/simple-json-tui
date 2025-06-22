use super::ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, Borders, Clear, HighlightSpacing, List, ListItem, block::BorderType},
};
use crate::{App, app::CurrentScreen, ui::ColorScheme};

pub(crate) fn render_popup(frame: &mut Frame<'_>, app: &mut App) {
    let mut list_items: Vec<ListItem> = Vec::new();

    for (i, item) in app.selection_screen.options.iter().enumerate() {
        let text: ListItem = Text::styled(
            format!("{{{}}}    {}", i + 1, *item),
            Style::new().fg(ColorScheme::Lavender.v()),
        )
        .alignment(Alignment::Left)
        .into();
        list_items.push(text);
    }

    let list_block = Block::bordered()
        .title_top(" Select ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(
            Style::new()
                .fg(ColorScheme::Flamingo.v())
                .bg(ColorScheme::Mantle.v()),
        );

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

    let select_block_area = center(
        frame.area(),
        Constraint::Percentage(15),
        // Constraint::Length(1),
        Constraint::Length((&list.len() + 4) as u16),
    );

    let list_area = center(
        select_block_area,
        // Constraint::Length(select_block_area.width),
        Constraint::Percentage(50),
        Constraint::Length(list.len() as u16),
    );

    if let CurrentScreen::Selection = app.current_screen {
        frame.render_widget(Clear, select_block_area);
        frame.render_widget(list_block, select_block_area);
        frame.render_stateful_widget(list, list_area, &mut app.selection_screen.state);
    }
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

#[allow(dead_code)]
fn render_main_blocks(frame: &mut Frame<'_>, main_layout: std::rc::Rc<[ratatui::prelude::Rect]>) {
    let block1 = Block::bordered()
        .title(" Left Side ")
        .title_alignment(Alignment::Center)
        .bold()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .style(Style::new().fg(ColorScheme::Peach.v()));
    let block2 = Block::bordered()
        .title(" Right Side ")
        .title_alignment(Alignment::Center)
        .bold()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .style(Style::new().fg(ColorScheme::Peach.v()));

    frame.render_widget(block1, main_layout[0]);
    frame.render_widget(block2, main_layout[2]);
}
