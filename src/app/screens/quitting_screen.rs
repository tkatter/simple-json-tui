#![allow(unused)]

use crate::{
    App, CurrentScreen, CurrentlyEditing, Print, ValueType,
    app::SelectionScreen,
    file_state::FileState,
    ratatui::crossterm::event::{KeyCode, KeyEvent},
};

pub fn match_quitting_screen(
    key: &KeyEvent,
    app: &mut App,
    file_state: &mut FileState,
) -> Option<Result<Print, std::io::Error>> {
    let mut return_value: Option<Result<Print, std::io::Error>> = None;
    match key.code {
        KeyCode::Char('y') => {
            if file_state.fname_input.is_empty() {
                return_value = Some(Ok(Print::Stdout));
            } else {
                return_value = Some(Ok(Print::File));
            }
        }
        KeyCode::Char('n') => {
            if !file_state.fname_input.is_empty() {
                file_state.remove_file();
            }

            return_value = Some(Ok(Print::None));
        }
        // Create a file if one is not already created
        KeyCode::Char('f') => {
            if file_state.fname_input.is_empty() {
                file_state.next_screen = CurrentScreen::Quitting;
                app.current_screen = CurrentScreen::FilePrompt;
            }
        }
        // Go back to main screen (cancel)
        KeyCode::Char('q') | KeyCode::Char('h') | KeyCode::Esc | KeyCode::Left => {
            app.current_screen = CurrentScreen::Main
        }
        KeyCode::Enter | KeyCode::Char('l') | KeyCode::Right => {
            if let Some(selected_idx) = app.quitting_screen.state.selected() {
                if file_state.fname_input.is_empty() {
                    match selected_idx {
                        // Save to stdout
                        0 => return_value = Some(Ok(Print::Stdout)),
                        // Create a file if one is not already create
                        1 => {
                            file_state.next_screen = CurrentScreen::Quitting;
                            app.current_screen = CurrentScreen::FilePrompt;
                        }
                        // Quit without saving
                        2 => return_value = Some(Ok(Print::None)),
                        _ => {}
                    }
                } else {
                    match selected_idx {
                        // Save to file
                        0 => return_value = Some(Ok(Print::File)),
                        // Quit without saving
                        1 => {
                            file_state.remove_file();
                            return_value = Some(Ok(Print::None))
                        }
                        _ => {}
                    }
                }
            }
        }
        KeyCode::Char('j') | KeyCode::Down => {
            let list_size = app.quitting_screen.list_size;
            if app.quitting_screen.state.selected().is_some() {
                let mut next = app
                    .quitting_screen
                    .state
                    .selected()
                    .expect("Verified that something is selected")
                    + 1;
                if next >= list_size {
                    next = 0
                }
                app.quitting_screen.state.select(Some(next));
            } else {
                app.quitting_screen.state.select(Some(0));
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            let list_size = app.quitting_screen.list_size;
            if app.quitting_screen.state.selected().is_some() {
                if app
                    .quitting_screen
                    .state
                    .selected()
                    .expect("Verified that something is selected")
                    == 0
                {
                    let next = list_size;
                    app.quitting_screen.state.select(Some(next));
                } else {
                    let next = app.quitting_screen.state.selected().unwrap() - 1;
                    app.quitting_screen.state.select(Some(next));
                }
            } else {
                app.quitting_screen.state.select(Some(list_size));
            }
        }
        _ => {}
    }

    return_value
}
