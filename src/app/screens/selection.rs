use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    widgets::ListState,
};

use crate::app::{App, CurrentScreen, CurrentlyEditing, ValueType};

#[derive(Debug)]
pub struct SelectionScreen {
    pub options: Vec<&'static str>,
    pub state: ListState,
}

impl Default for SelectionScreen {
    fn default() -> Self {
        Self {
            options: vec!["String", "Number", "Bool", "Object", "Array"],
            state: ListState::default(),
        }
    }
}

pub fn match_selection_screen(key: &KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Enter | KeyCode::Char('l') | KeyCode::Right => {
            if let Some(selected_idx) = app.selection_screen.state.selected() {
                let selected_value = match selected_idx {
                    0 => ValueType::String,
                    1 => ValueType::Number,
                    2 => ValueType::Bool(true),
                    3 => ValueType::Object,
                    4 => ValueType::Array,
                    _ => ValueType::default(),
                };
                app.value_type = selected_value.to_owned();
                app.currently_editing = Some(CurrentlyEditing::Key);
                app.current_screen = CurrentScreen::Editing(selected_value);
                app.selection_screen.state.select(None);
            } else {
                // app.value_type = ValueType::default();
                app.currently_editing = Some(CurrentlyEditing::Key);
                app.current_screen = CurrentScreen::Editing(app.value_type.to_owned());
                app.selection_screen.state.select(None);
            }
        }
        KeyCode::Char('q') | KeyCode::Char('h') | KeyCode::Esc | KeyCode::Left => {
            // Don't clear object values or pairs
            app.key_input = String::new();
            app.value_input = String::new();
            app.value_type = ValueType::default();
            app.current_screen = CurrentScreen::default();
            app.currently_editing = None;
            app.selection_screen = SelectionScreen::default();
        }
        KeyCode::Char('1') => app.selection_screen.state.select(Some(0)),
        KeyCode::Char('2') => app.selection_screen.state.select(Some(1)),
        KeyCode::Char('3') => app.selection_screen.state.select(Some(2)),
        KeyCode::Char('4') => app.selection_screen.state.select(Some(3)),
        KeyCode::Char('5') => app.selection_screen.state.select(Some(4)),
        KeyCode::Char('j') | KeyCode::Down => {
            if app.selection_screen.state.selected().is_some() {
                let mut next = app.selection_screen.state.selected().unwrap() + 1;
                if next >= 5 {
                    next = 0
                }
                app.selection_screen.state.select(Some(next));
            } else {
                app.selection_screen.state.select(Some(0));
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            if app.selection_screen.state.selected().is_some() {
                if app.selection_screen.state.selected().unwrap() == 0 {
                    let next = 4;
                    app.selection_screen.state.select(Some(next));
                } else {
                    let next = app.selection_screen.state.selected().unwrap() - 1;
                    app.selection_screen.state.select(Some(next));
                }
            } else {
                app.selection_screen.state.select(Some(4));
            }
        }
        _ => {}
    }
}
