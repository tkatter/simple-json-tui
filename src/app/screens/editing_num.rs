use crate::{
    App, CurrentlyEditing, ValueType,
    ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    traits::UpdateMap,
};

pub fn match_num_editing(key: &KeyEvent, app: &mut App) {
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
                                    && app.object_values.values.get("").unwrap().is_number()
                                {
                                    let new_key = app.key_input.clone();
                                    app.update_object_key("", &new_key);
                                } else {
                                    app.add_object_value(None, Some(ValueType::Number));
                                }
                            } else if app.editing_preview.values.contains_key("")
                                && app.editing_preview.values.get("").unwrap().is_number()
                            {
                                app.editing_preview.update_key("", &app.key_input);
                            } else {
                                app.editing_preview.new_number(&app.key_input, true);
                            }
                            app.toggle_editing();
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
                        if !app.key_input.is_empty() {
                            if app.editing_object {
                                if app.object_values.values.contains_key("")
                                    && app.object_values.values.get("").unwrap().is_number()
                                {
                                    let new_key = app.key_input.clone();
                                    app.update_object_key("", &new_key);
                                } else {
                                    app.add_object_value(None, Some(ValueType::Number));
                                }
                            } else if app.editing_preview.values.contains_key("")
                                && app.editing_preview.values.get("").unwrap().is_number()
                            {
                                app.editing_preview.update_key("", &app.key_input);
                            } else {
                                app.editing_preview.new_number(&app.key_input, true);
                            }
                            app.toggle_editing();
                        }
                    }

                    CurrentlyEditing::Value => {
                        if app.editing_object {
                            if !app.value_input.is_empty() {
                                app.object_values.push(
                                    &app.key_input,
                                    serde_json::Value::Number(
                                        app.value_input.clone().parse().unwrap(),
                                    ),
                                );
                            }
                            let key = app.key_input.clone();
                            app.update_object_key(&key, "");
                        } else if !app.editing_preview.is_empty() {
                            if !app.value_input.is_empty() {
                                app.editing_preview.push(
                                    &app.key_input,
                                    serde_json::Value::Number(
                                        app.value_input.clone().parse().unwrap(),
                                    ),
                                );
                            }
                            app.editing_preview.update_key(&app.key_input, "");
                        }
                        app.toggle_editing();
                    }
                }
            }
        }
        KeyCode::Char(value) => {
            if let Some(editing) = &app.currently_editing {
                if !key.modifiers.contains(KeyModifiers::CONTROL) {
                    match editing {
                        CurrentlyEditing::Key => {
                            app.key_input.push(value);
                        }
                        CurrentlyEditing::Value => match value {
                            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                                app.push_char(key, value)
                            }
                            _ => {}
                        },
                    }
                }
            }
        }
        _ => {}
    }
}
