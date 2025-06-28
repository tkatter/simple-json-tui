use crate::App;
use crate::CurrentlyEditing;
use crate::app::{UpdateMap, ValueType};
use crate::ui::ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn match_bool_editing(key: &KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Enter => {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        // If input is not empty, push value to preview and
                        // switch focus to value_input
                        if !app.key_input.is_empty() {
                            if app.editing_object {
                                if app.object_values.values.contains_key("")
                                    && app.object_values.values.get("").unwrap().is_boolean()
                                {
                                    let new_key = app.key_input.clone();
                                    app.update_object_key("", &new_key);
                                } else {
                                    app.add_object_value(None, Some(ValueType::Bool(true)));
                                }
                            } else if app.editing_preview.values.contains_key("")
                                && app.editing_preview.values.get("").unwrap().is_boolean()
                            {
                                app.editing_preview.update_key("", &app.key_input);
                            } else {
                                app.editing_preview.new_bool(&app.key_input, true);
                            }
                            app.toggle_editing();
                        }
                    }
                    CurrentlyEditing::Value => {
                        app.save_key_value();
                    }
                }
            }
        }
        KeyCode::Backspace => {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => app.del_char(),
                    CurrentlyEditing::Value => {}
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
                                if app.object_values.values.contains_key("")
                                    && app.object_values.values.get("").unwrap().is_boolean()
                                {
                                    let new_key = app.key_input.clone();
                                    app.update_object_key("", &new_key);
                                } else {
                                    app.add_object_value(None, Some(ValueType::Bool(true)));
                                }
                            } else if app.editing_preview.values.contains_key("")
                                && app.editing_preview.values.get("").unwrap().is_boolean()
                            {
                                app.editing_preview.update_key("", &app.key_input);
                            } else {
                                app.editing_preview.new_bool(&app.key_input, true);
                            }
                            app.toggle_editing();
                        }
                    }
                    CurrentlyEditing::Value => {
                        // If editing_preview has value, clear it before
                        // toggling focus
                        if app.editing_object {
                            let key = app.key_input.clone();
                            app.update_object_key(&key, "");
                        } else if !app.editing_preview.is_empty() {
                            app.editing_preview.update_key(&app.key_input, "");
                        }
                        app.toggle_editing();
                    }
                }
            }
        }
        KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {}
                    CurrentlyEditing::Value => app.toggle_bool(),
                }
            }
        }
        KeyCode::Char(value) => {
            if let Some(editing) = &app.currently_editing {
                // Need this to avoid adding characters when CTRL is pressed
                if !key.modifiers.contains(KeyModifiers::CONTROL) {
                    match editing {
                        CurrentlyEditing::Key => {
                            app.key_input.push(value);
                        }
                        CurrentlyEditing::Value => match value {
                            'h' | 'j' | 'k' | 'l' => app.toggle_bool(),
                            _ => {}
                        },
                    }
                }
            }
        }
        _ => {}
    }
}
