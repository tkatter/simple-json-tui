use crate::{
    App, CurrentlyEditing, ValueType,
    app::UpdateMap,
    ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

pub fn match_array_editing(key: &KeyEvent, app: &mut App) {
    // KEYMAP TO ADD ANOTHER ITEM
    if key.code == KeyCode::Char('n') && key.modifiers.contains(KeyModifiers::CONTROL) {
        if let Some(editing) = &app.currently_editing {
            match editing {
                CurrentlyEditing::Key => {}
                CurrentlyEditing::Value => {
                    // `store_array_values` updates the editing_preview
                    if !app.value_input.is_empty() {
                        app.store_array_values();
                        app.value_input = String::new();
                    }
                }
            }
        }
    }

    match key.code {
        KeyCode::Enter => {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        // If input is not empty, push value to preview and
                        // switch focus to value_input
                        if !app.key_input.is_empty() {
                            // FOR ARRAYS IN OBJECTS
                            if app.editing_object {
                                if app.object_values.values.contains_key("") {
                                    let is_array = app.object_values.values.get("").unwrap();
                                    if is_array.is_array() {
                                        let key = app.key_input.clone();
                                        app.update_object_key("", &key);
                                    }
                                } else {
                                    // Push visual queue that user is editing an array
                                    app.add_object_value(None, Some(ValueType::Array));
                                }

                            // FOR REGULAR ARRAYS
                            } else if app.editing_preview.is_empty() {
                                app.editing_preview.new_array(&app.key_input, true);
                            } else {
                                app.editing_preview.update_key("", &app.key_input);
                            }

                            app.toggle_editing();
                        }
                    }
                    CurrentlyEditing::Value => {
                        if app.value_input.is_empty() {
                            // IF VALUE FIELD IS EMPTY BUT HAS STORED VALUES THEN
                            // SUBMIT -- ELSE DON'T
                            if !app.array_values.is_empty() {
                                app.save_key_value();
                            }
                        } else {
                            app.store_array_values(); // need to store value_input as array value before saving
                            app.save_key_value();
                        }
                    }
                }
            }
        }
        KeyCode::Char(value) => {
            app.push_char(key, value);
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
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        // dont toggle if no key or no values
                        if !app.key_input.is_empty() {
                            if app.editing_object && app.object_values.values.contains_key("") {
                                let is_array = app.object_values.values.get("").unwrap();
                                if is_array.is_array() {
                                    let key = app.key_input.clone();
                                    app.update_object_key("", &key);
                                }
                            } else if app.editing_object {
                                // Push visual queue that user is editing an array
                                app.add_object_value(None, Some(ValueType::Array));
                            } else if app.editing_preview.is_empty() {
                                app.editing_preview.new_array(&app.key_input, true);
                            } else {
                                app.editing_preview.update_key("", &app.key_input);
                            }
                            app.toggle_editing();
                        }
                    }
                    CurrentlyEditing::Value => {
                        if app.value_input.is_empty() {
                            if app.editing_object {
                                let key = app.key_input.clone();
                                app.update_object_key(&key, "");
                            } else {
                                app.editing_preview.update_key(&app.key_input, "");
                            }
                        } else if app.editing_object {
                            let key = app.key_input.clone();
                            app.store_array_values();
                            app.update_object_key(&key, "");
                            app.value_input = String::new();
                        } else {
                            app.store_array_values(); // need to store value_input as array value
                            app.editing_preview.update_key(&app.key_input, "");
                            app.value_input = String::new();
                        }

                        app.toggle_editing();
                    }
                }
            }
        }
        _ => {}
    }
}
