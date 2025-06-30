use crate::{
    App, CurrentlyEditing,
    ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

pub fn match_null_editing(key: &KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Enter => {
            if let Some(CurrentlyEditing::Key) = &app.currently_editing {
                if !app.key_input.is_empty() {
                    app.save_key_value();
                }
            }
        }
        KeyCode::Backspace => {
            if let Some(CurrentlyEditing::Key) = &app.currently_editing {
                app.del_char();
            }
        }
        KeyCode::BackTab => {
            if let Some(CurrentlyEditing::Key) = &app.currently_editing {
                app.toggle_value_type();
            }
        }
        KeyCode::Esc => {
            app.handle_escape();
        }
        KeyCode::Tab => {
            if let Some(CurrentlyEditing::Key) = &app.currently_editing {
                app.save_key_value();
            }
        }
        KeyCode::Char(value) => {
            if let Some(CurrentlyEditing::Key) = &app.currently_editing {
                // Need this to avoid adding characters when CTRL is pressed
                if !key.modifiers.contains(KeyModifiers::CONTROL) {
                    app.key_input.push(value);
                }
            }
        }
        _ => {}
    }
}
