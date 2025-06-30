use crate::{
    App, FileState,
    ratatui::crossterm::event::{KeyCode, KeyEvent},
};

pub fn match_file_screen(key: &KeyEvent, app: &mut App, file_state: &mut FileState) {
    match key.code {
        KeyCode::Esc => {
            app.handle_escape();
        }
        KeyCode::Enter => {
            file_state.create_file_buf();
            app.current_screen = file_state.next_screen;
        }
        KeyCode::Char(value) => {
            file_state.fname_input.push(value);
        }
        KeyCode::Backspace => {
            file_state.fname_input.pop();
        }

        _ => {}
    }
}
