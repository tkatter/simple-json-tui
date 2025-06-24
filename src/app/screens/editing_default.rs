use crate::ui::ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::App;
use crate::CurrentScreen;
use crate::CurrentlyEditing;

pub fn match_default_editing(key: &KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Enter => {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        if !app.key_input.is_empty() {
                            app.editing_preview
                                .push(app.key_input.to_owned(), serde_json::to_value("").unwrap());
                            app.currently_editing = Some(CurrentlyEditing::Value);
                        }
                    }
                    CurrentlyEditing::Value => {
                        app.editing_preview.push(
                            app.key_input.to_owned(),
                            serde_json::to_value(app.value_input.to_owned()).unwrap(),
                        );
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
            app.editing_preview.reset();
            app.current_screen = CurrentScreen::Main;
            app.currently_editing = None; // exit editing mode
        }
        KeyCode::Tab => {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        if !app.key_input.is_empty() {
                            app.toggle_editing();
                        }
                    }
                    CurrentlyEditing::Value => app.toggle_editing(),
                }
            }
        }
        KeyCode::Char(value) => {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        app.key_input.push(value);
                        app.editing_preview
                            .push(value.into(), serde_json::to_value("").unwrap());

                        if !app.key_input.is_empty() {
                            app.editing_preview
                                .push(value.into(), serde_json::to_value("").unwrap());
                        }
                    }
                    CurrentlyEditing::Value => {
                        app.value_input.push(value);
                        app.editing_preview.push(
                            app.key_input.to_owned(),
                            serde_json::to_value(value).unwrap(),
                        );
                    }
                }
            }
        }
        _ => {}
    }
}
