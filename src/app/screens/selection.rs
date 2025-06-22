use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    widgets::ListState,
};

use crate::app::{App, CurrentScreen};

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
            app.current_screen = CurrentScreen::Main;
            app.selection_screen.state.select(None);
        }
        KeyCode::Char('q') | KeyCode::Char('h') | KeyCode::Esc | KeyCode::Left => {
            app.current_screen = CurrentScreen::Main;
            app.selection_screen.state.select(None);
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
