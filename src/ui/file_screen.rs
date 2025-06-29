use ratatui::style::Modifier;

use crate::{
    App, CurrentScreen, FileState,
    ratatui::{
        Frame,
        layout::{Alignment, Constraint},
        style::Style,
        text::{Span, Text},
        widgets::{Block, BorderType, Clear, Paragraph},
    },
    ui::{helpers::center, theme::ColorScheme},
};

pub fn render_file_prompt(frame: &mut Frame<'_>, app: &mut App, file_state: &mut FileState) {
    let file_prompt_area = center(
        frame.area(),
        Constraint::Percentage(50),
        Constraint::Length(3),
    );

    let file_prompt_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title_top(Span::from(" File Name "))
        .title_alignment(Alignment::Left)
        .style(
            Style::new()
                .fg(ColorScheme::Peach.v())
                .bg(ColorScheme::Mantle.v())
                .add_modifier(Modifier::BOLD),
        );

    let file_prompt = Paragraph::new(Text::from(Span::from(&file_state.fname_input)))
        .block(file_prompt_block)
        .style(Style::new().fg(ColorScheme::Green.v()));

    if let CurrentScreen::FilePrompt = app.current_screen {
        frame.render_widget(Clear, file_prompt_area);
        frame.render_widget(file_prompt, file_prompt_area);
    };
}
