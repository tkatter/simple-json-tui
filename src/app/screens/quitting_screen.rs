#![allow(unused)]

use crate::{
    App, CurrentScreen, CurrentlyEditing, ValueType,
    app::SelectionScreen,
    ratatui::crossterm::event::{KeyCode, KeyEvent},
};

pub fn match_quitting_screen(key: &KeyEvent, app: &mut App) -> Result<bool, std::io::Error> {
    match key.code {
        KeyCode::Char('y') | KeyCode::Enter | KeyCode::Tab => {
            Ok(true) // signal to print JSON
        }
        KeyCode::Char('n') | KeyCode::Char('q') | KeyCode::Esc => {
            Ok(false) // signal to not print JSON
        }
        _ => Ok(false),
    }
}
