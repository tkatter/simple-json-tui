use crate::ui::ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::App;
use crate::CurrentlyEditing;
use crate::app::{CurrentScreen, UpdateMap, ValueType};

pub fn match_object_editing(key: &KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Enter => {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        // If `key_input` is not empty && there is not an
                        // object in progress, create a new object
                        if !app.key_input.is_empty() {
                            if app.object_values.key.is_empty() {
                                app.object_values.add_key(&app.key_input);
                                app.editing_object = true;
                                app.editing_preview.new_object(&app.object_values.key, true);
                                app.key_input = String::new();
                                app.value_type = ValueType::String;
                                app.current_screen = CurrentScreen::Editing(ValueType::String);
                            } else {
                                app.editing_preview.new_object(&app.key_input, true);
                                app.toggle_editing();
                            }
                        }
                    }
                    CurrentlyEditing::Value => {
                        app.save_key_value();
                        app.current_screen = CurrentScreen::Main;
                    }
                }
            }
        }
        KeyCode::Backspace => {
            app.del_char();
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
            app.handle_escape();
        }
        KeyCode::Tab => {
            app.toggle_editing();
        }
        KeyCode::Char(value) => {
            app.push_char(key, value);
        }
        _ => {}
    }
}
