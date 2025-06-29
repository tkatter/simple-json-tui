use crate::{
    App, CurrentScreen,
    ratatui::{
        Frame,
        layout::{Constraint, Flex, Layout, Rect},
        style::Style,
        text::{Line, Text},
        widgets::Paragraph,
        widgets::{Block, BorderType, Borders},
    },
    ui::ColorScheme,
};

pub fn render_start_screen(frame: &mut Frame<'_>, app: &mut App) {
    let block1 = Block::bordered()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::new().fg(ColorScheme::Peach.v()));

    let start_text = Text::from(vec![
        Line::from("Welcome to the App!"),
        Line::from("Press 's' to get started"),
    ])
    .style(Style::new().fg(ColorScheme::Peach.v()));

    let start_paragraph = Paragraph::new(start_text).centered();

    let p_area = center(
        frame.area(),
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    );

    if let CurrentScreen::Start = app.current_screen {
        frame.render_widget(block1, frame.area());
        frame.render_widget(start_paragraph, p_area);
    };
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
