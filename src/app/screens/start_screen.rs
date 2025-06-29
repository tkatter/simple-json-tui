use crate::{
    App, CurrentScreen,
    ratatui::crossterm::event::{KeyCode, KeyEvent},
};

pub fn match_start_screen(key: &KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char(value) => match value {
            'f' => app.current_screen = CurrentScreen::FilePrompt,
            _ => {}
        },
        _ => {}
    }
}
