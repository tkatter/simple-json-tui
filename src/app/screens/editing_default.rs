use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::App;
use crate::CurrentlyEditing;
use crate::app::CurrentScreen;

pub fn match_default_editing(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Enter => {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        app.currently_editing = Some(CurrentlyEditing::Value);
                    }
                    CurrentlyEditing::Value => {
                        if app.key_input.is_empty() | app.value_input.is_empty() {
                            app.key_input = String::from("cantSubmitNoKey");
                            app.currently_editing = Some(CurrentlyEditing::Key); // reset to Key
                        } else if !app.value_input.is_empty() {
                            app.save_key_value();
                            app.current_screen = CurrentScreen::Main;
                        }
                    }
                }
            }
        }
        KeyCode::Backspace => {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        app.key_input.pop();
                    }
                    CurrentlyEditing::Value => {
                        app.value_input.pop();
                    }
                }
            }
        }
        KeyCode::BackTab => {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {}
                    CurrentlyEditing::Value => app.toggle_value_type(),
                }
            }
        }
        KeyCode::Esc => {
            app.current_screen = CurrentScreen::Main;
            app.currently_editing = None; // exit editing mode
        }
        KeyCode::Tab => {
            app.toggle_editing();
        }
        KeyCode::Char(value) => {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        app.key_input.push(value);
                    }
                    CurrentlyEditing::Value => {
                        app.value_input.push(value);
                    }
                }
            }
        }
        _ => {}
    }
}
