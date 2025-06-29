use crate::{
    App, CurrentScreen, CurrentlyEditing,
    ratatui::{
        Frame,
        layout::{Constraint, Direction, Layout, Rect},
        style::{Color, Style},
        text::{Line, Span},
        widgets::{Block, Borders, Paragraph},
    },
    ui::ColorScheme,
};

pub fn render_bottom_status_bar(frame: &mut Frame<'_>, app: &mut App, area: Rect) {
    let current_navigation_text = vec![
        // first half of the text
        match app.current_screen {
            CurrentScreen::Start => Span::default(),
            CurrentScreen::FilePrompt => Span::default(),
            CurrentScreen::Main => {
                Span::styled("Normal Mode", Style::default().fg(ColorScheme::Green.v()))
            }
            CurrentScreen::Selection => {
                Span::styled("Select type", Style::default().fg(ColorScheme::Green.v()))
            }
            CurrentScreen::Editing(_) => {
                Span::styled("Editing Mode", Style::default().fg(ColorScheme::Green.v()))
            }
            CurrentScreen::Quitting => {
                Span::styled("Exiting", Style::default().fg(ColorScheme::Green.v()))
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
            CurrentScreen::Start => Span::default(),
            CurrentScreen::FilePrompt => Span::default(),
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
        .split(area);

    // show only if not on the start screen
    match app.current_screen {
        CurrentScreen::Start => {}
        _ => {
            frame.render_widget(mode_footer, footer_chunks[0]);
            frame.render_widget(key_notes_footer, footer_chunks[1]);
        }
    }
}
