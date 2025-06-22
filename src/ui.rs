mod components;
mod selection_pop;
mod theme;
use components::AppTitle;
use theme::ColorScheme;

use crate::{
    app::{App, CurrentScreen, CurrentlyEditing},
    ui::{
        components::{EditingBox, EditingId},
        selection_pop::render_selection_list,
    },
};

#[allow(clippy::single_component_path_imports)]
pub(crate) use ratatui;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // cut the given rect into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // cut the middle piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn ui(frame: &mut Frame, app: &mut App) {
    // SET A BACKGROUND COLOR FOR THE ENTIRE FRAME
    let background_color = Block::new().style(Style::new().bg(ColorScheme::Crust.v()));
    frame.render_widget(background_color, frame.area());

    // MAIN LAYOUT
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title = AppTitle::new("Create new JSON".to_string());
    frame.render_widget(title, chunks[0]);

    // RENDER LIST OF KEY: VALUE PAIRS - COULD IMPLEMENT SELECTION FUNCTIONALITY HERE
    let mut list_items = Vec::<ListItem>::new();

    for key in app.pairs.keys() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} : {}", key, app.pairs.get(key).unwrap()),
            Style::default().fg(Color::Yellow),
        ))));
    }

    let list = List::new(list_items);

    frame.render_widget(list, chunks[1]);

    // BOTTOM NAV BAR
    let current_navigation_text = vec![
        // first half of the text
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Selection => {
                Span::styled("Select type", Style::default().fg(Color::Green))
            }
            CurrentScreen::Editing(_) => {
                Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Quitting => {
                Span::styled("Exiting", Style::default().fg(Color::LightRed))
            }
        }
        .to_owned(),
        // white divider
        Span::styled(" | ", Style::default().fg(Color::White)),
        // second half of the text with hints on what the user is editing
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        Span::styled("Editing Key", Style::default().fg(Color::Green))
                    }
                    CurrentlyEditing::Value => {
                        Span::styled("Editing Value", Style::default().fg(Color::LightGreen))
                    }
                }
            } else {
                Span::styled("Not Editing", Style::default().fg(Color::DarkGray))
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    //render keybinding hints
    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "q to quit / e to make new pair",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Selection => Span::styled(
                "<esc> to close / [#] to select",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Editing(_) => Span::styled(
                "<esc> to cancel / <tab> to switch boxes / <enter> to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Quitting => Span::styled(
                "q to quit / s to select value type / e to make new pair",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    // create layout for the bottom section
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);

    // SELECTION POPUP
    render_selection_list(frame, app);

    // EDITING POPUP
    if let Some(editing) = &app.currently_editing {
        let popup_block = Block::bordered()
            .title(Line::from("Enter a new key-value pair").centered())
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Double)
            .style(Style::new().bg(ColorScheme::Base.v()))
            .bold();

        // Size of the popup
        let area = centered_rect(60, 15, frame.area());
        frame.render_widget(popup_block, area);

        // popup layout
        let popup_chunks =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .margin(2)
                .spacing(1)
                .split(area);

        // render blocks for key and value pairs
        let key_box = EditingBox::new(EditingId::Key, app, editing);
        frame.render_widget(key_box, popup_chunks[0]);
        let value_box = EditingBox::new(EditingId::Value, app, editing);
        frame.render_widget(value_box, popup_chunks[1]);
    }

    // EXIT POPUP
    if let CurrentScreen::Quitting = app.current_screen {
        // clear frame
        frame.render_widget(Clear, frame.area());

        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to output the buffer as JSON?",
            Style::default().fg(Color::Red),
        );

        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
    }
}
