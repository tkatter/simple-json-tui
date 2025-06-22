use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::App;
use crate::CurrentlyEditing;
use crate::app::{CurrentScreen, ValueType};

pub fn match_array_editing(key: &KeyEvent, app: &mut App) {
    // KEYMAP TO ADD ANOTHER ITEM
    if key.code == KeyCode::Char('n') && key.modifiers.contains(KeyModifiers::CONTROL) {
        if let Some(editing) = &app.currently_editing {
            match editing {
                CurrentlyEditing::Key => {}
                CurrentlyEditing::Value => {
                    app.store_array_values();
                    app.value_input = String::new();
                }
            }
        }
    }

    match key.code {
        KeyCode::Enter => {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        app.currently_editing = Some(CurrentlyEditing::Value);
                    }
                    CurrentlyEditing::Value => {
                        if app.key_input.is_empty() {
                            // IF KEY FIELD IS EMPTY -- DONT SUBMIT
                            app.key_input = String::from("cantSubmitNoKey");
                            app.currently_editing = Some(CurrentlyEditing::Key); // reset to Key
                        } else if app.value_input.is_empty() {
                            // IF VALUE FIELD IS EMPTY BUT HAS STORED VALUES THEN
                            // SUBMIT -- ELSE DON'T
                            if !app.array_values.values.is_empty() {
                                app.save_key_value();
                                app.array_values.reset();
                                app.value_type = ValueType::String; // reset value type
                                app.current_screen = CurrentScreen::Main;
                            } else {
                                app.value_input = String::from("cantSubmitNoValue");
                            }
                        } else {
                            app.store_array_values();
                            app.save_key_value();
                            app.array_values.reset();
                            app.value_type = ValueType::String; // reset value type
                            app.current_screen = CurrentScreen::Main;
                        }
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
            app.current_screen = CurrentScreen::Main;
            app.currently_editing = None; // exit editing mode
        }
        KeyCode::Tab => {
            app.toggle_editing();
        }
        _ => {}
    }
}
