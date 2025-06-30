use crate::{
    App, CurrentScreen, Print,
    ratatui::crossterm::event::{KeyCode, KeyEvent},
};

pub fn match_start_screen(key: &KeyEvent, app: &mut App) -> Option<Result<Print, std::io::Error>> {
    let mut return_value: Option<Result<Print, std::io::Error>> = None;
    match key.code {
        KeyCode::Char(value) => match value {
            'f' => app.current_screen = CurrentScreen::FilePrompt,
            's' => app.current_screen = CurrentScreen::Selection,
            // Force quit app
            'q' => {
                return_value = Some(Ok(Print::None));
            }
            _ => {}
        },
        KeyCode::Enter => app.current_screen = CurrentScreen::Selection,
        _ => {}
    }

    return_value
}
