use crate::ui::ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::App;
use crate::CurrentlyEditing;
use crate::app::CurrentScreen;

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
                            if app.editing_preview.is_empty() {
                                app.editing_preview.new_array(&app.key_input);
                                app.toggle_editing();
                            } else {
                                app.editing_preview.update_key("", &app.key_input);
                                app.toggle_editing();
                            }
                        }
                    }
                    CurrentlyEditing::Value => {
                        if app.value_input.is_empty() {
                            // IF VALUE FIELD IS EMPTY BUT HAS STORED VALUES THEN
                            // SUBMIT -- ELSE DON'T
                            if !app.array_values.is_empty() {
                                app.save_key_value();
                                app.array_values.reset();
                            } else {
                                app.value_input = String::from("cantSubmitNoValue");
                            }
                        } else {
                            app.store_array_values(); // need to store value_input as array value before saving
                            app.save_key_value();
                            app.array_values.reset();
                        }
                    }
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
                        CurrentlyEditing::Value => {
                            app.value_input.push(value);
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
            app.array_values.reset();
            app.editing_preview.reset();
            app.key_input = String::new();
            app.value_input = String::new();
            app.current_screen = CurrentScreen::Main;
            app.currently_editing = None; // exit editing mode
        }
        KeyCode::Tab => {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        // dont toggle if no key or no values
                        if !app.key_input.is_empty() {
                            if app.editing_preview.is_empty() {
                                app.editing_preview.new_array(&app.key_input);
                                app.toggle_editing();
                            } else {
                                app.editing_preview.update_key("", &app.key_input);
                                app.toggle_editing();
                            }
                        }
                    }
                    CurrentlyEditing::Value => {
                        if app.value_input.is_empty() {
                            app.editing_preview.update_key(&app.key_input, "");
                            app.toggle_editing();
                        } else {
                            app.store_array_values(); // need to store value_input as array value
                            app.editing_preview.update_key(&app.key_input, "");
                            app.toggle_editing();
                            app.value_input = String::new();
                        }
                    }
                }
            }
        }
        _ => {}
    }
}
