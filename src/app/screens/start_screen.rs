use crate::{
    App, CurrentScreen,
    ratatui::crossterm::event::{KeyCode, KeyEvent},
};

pub fn match_start_screen(key: &KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char(value) => match value {
            'f' => app.current_screen = CurrentScreen::FilePrompt,
            's' => app.current_screen = CurrentScreen::Selection,
            'q' => app.current_screen = CurrentScreen::Quitting,
            _ => {}
        },
        KeyCode::Enter => app.current_screen = CurrentScreen::Selection,
        _ => {}
    }
}
