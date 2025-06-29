mod bottom_status_bar;
mod components;
mod editing;
mod file_screen;
mod helpers;
mod preview;
mod selection_pop;
mod start;
mod theme;
use components::Header;

use crate::{
    App, CurrentScreen, FileState,
    ratatui::{
        Frame,
        layout::{Constraint, Direction, Layout, Rect},
        style::{Color, Style},
        text::Text,
        widgets::{Block, Borders, Clear, Paragraph, Wrap},
    },
    ui::{
        bottom_status_bar::render_bottom_status_bar, editing::render_editing,
        file_screen::render_file_prompt, preview::render_preview_json,
        selection_pop::render_selection_list, start::render_start_screen, theme::ColorScheme,
    },
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

pub fn ui(frame: &mut Frame, app: &mut App, file_state: &mut FileState) {
    // SET A BACKGROUND COLOR FOR THE ENTIRE FRAME
    let background_color = Block::new().style(Style::new().bg(ColorScheme::Crust.v()));
    frame.render_widget(background_color, frame.area());

    // RENDER START SCREEN
    render_start_screen(frame, app);
    render_file_prompt(frame, app, file_state);

    // MAIN LAYOUT
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // HEADER
    let header_layout = Layout::horizontal(vec![
        Constraint::Min(1),
        Constraint::Min(1),
        Constraint::Min(1),
    ])
    .split(chunks[0]);

    if !file_state.fname_input.is_empty() {
        let header = Header::new(&file_state.fname_input);
        frame.render_widget(Clear, header_layout[1]);
        frame.render_widget(header, header_layout[1]);
    }

    // EDITING/PREVIEW LAYOUT
    let edit_preview_layout =
        Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .margin(2)
            .spacing(2)
            .split(chunks[2]);

    // RENDER LIST OF KEY: VALUE PAIRS - COULD IMPLEMENT SELECTION FUNCTIONALITY HERE
    render_preview_json(frame, app, edit_preview_layout[1]);

    // RENDER BOTTOM STATUS BAR
    render_bottom_status_bar(frame, app, chunks[3]);

    // EDITING POPUP
    if let CurrentScreen::Editing(_) | CurrentScreen::Main | CurrentScreen::Selection =
        &app.current_screen
    {
        render_editing(frame, app, edit_preview_layout[0]);
    }

    // SELECTION POPUP
    render_selection_list(frame, app);

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
