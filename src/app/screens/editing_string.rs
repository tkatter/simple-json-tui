use crate::app::{UpdateMap, ValueType};
use crate::ui::ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::App;
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
                            if app.editing_object {
                                app.add_object_value(None, Some(ValueType::String));
                            } else {
                                app.editing_preview.new_string(&app.key_input, true);
                            }
                            app.currently_editing = Some(CurrentlyEditing::Value);
                        }
                    }
                    CurrentlyEditing::Value => {
                        if !app.value_input.is_empty() {
                            app.save_key_value();
                        }
                    }
                }
            }
        }
        KeyCode::Backspace => app.del_char(),
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
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        // Push key_input to editing preview and toggle
                        // focus to value_input if not empty
                        if !app.key_input.is_empty() {
                            if app.editing_object {
                                app.add_object_value(None, Some(ValueType::String));
                            } else {
                                app.editing_preview.new_string(&app.key_input, true);
                            }

                            app.toggle_editing();
                        }
                    }
                    CurrentlyEditing::Value => {
                        // If editing_preview has value, clear it before
                        // toggling focus
                        if app.editing_object {
                            let key = app.key_input.clone();
                            app.remove_object_value(&key);
                        } else if !app.editing_preview.is_empty() {
                            app.editing_preview.reset();
                        }
                        app.toggle_editing();
                    }
                }
            }
        }
        KeyCode::Char(value) => {
            app.push_char(key, value);
        }
        _ => {}
    }
}
