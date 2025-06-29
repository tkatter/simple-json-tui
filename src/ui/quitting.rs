use ratatui::style::Style;

use crate::{
    App, CurrentScreen,
    file_state::FileState,
    ratatui::{
        Frame,
        layout::Constraint,
        text::{Line, Span, Text},
        widgets::{Block, BorderType, Clear, Paragraph},
    },
    ui::{helpers::center, theme::ColorScheme},
};

pub fn render_quitting_screen(frame: &mut Frame<'_>, app: &mut App, file_state: &FileState) {
    let popup_area = center(
        frame.area(),
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    );

    let popup_block = Block::bordered().border_type(BorderType::Rounded);

    let save_option: Span = if !file_state.fname_input.is_empty() {
        Span::from(format!(
            "Would you like to save to {}?",
            file_state.fname_input
        ))
    } else {
        Span::from("Would you like to print the buffer as stdout?")
    };

    let popup_content = Paragraph::new(Text::from(vec![
        Line::from(vec![Span::from("Y"), Span::raw("  "), save_option])
            .style(Style::new().fg(ColorScheme::Green.v())),
        if file_state.fname_input.is_empty() {
            Line::from(vec![
                Span::from("F"),
                Span::raw("  "),
                Span::from("Save to file."),
            ])
            .style(Style::new().fg(ColorScheme::Yellow.v()))
        } else {
            Line::raw("")
        },
        Line::from(vec![
            Span::from("N"),
            Span::raw("  "),
            Span::from("Quit without saving"),
        ])
        .style(Style::new().fg(ColorScheme::Red.v())),
    ]));

    if let CurrentScreen::Quitting = app.current_screen {
        frame.render_widget(Clear, popup_area);
        frame.render_widget(popup_content.block(popup_block), popup_area);
    }
}
