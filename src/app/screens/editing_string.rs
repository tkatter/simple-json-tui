use crate::ui::ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::App;
use crate::CurrentScreen;
use crate::CurrentlyEditing;

pub fn match_string_editing(key: &KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Enter => {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        // If input is not empty, push value to preview and
                        // switch focus to value_input
                        if !app.key_input.is_empty() {
                            app.editing_preview.new_string(app.key_input.to_owned());
                            app.currently_editing = Some(CurrentlyEditing::Value);
                        }
                    }
                    CurrentlyEditing::Value => {
                        // Restrict what happens when Enter is pressed
                        // and focused on value_input
                        // TODO: Handle this better
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
            // Reset state and return to main screen
            app.editing_preview.reset();
            app.currently_editing = None;
            app.current_screen = CurrentScreen::Main;
        }
        KeyCode::Tab => {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        // Push key_input to editing preview and toggle
                        // focus to value_input if not empty
                        if !app.key_input.is_empty() {
                            app.editing_preview.new_string(app.key_input.to_owned());
                            app.toggle_editing();
                        }
                    }
                    CurrentlyEditing::Value => {
                        // If editing_preview has value, clear it before
                        // toggling focus
                        if !app.editing_preview.is_empty() {
                            app.editing_preview.reset();
                        }
                        app.toggle_editing();
                    }
                }
            }
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
