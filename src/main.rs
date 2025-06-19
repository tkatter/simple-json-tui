use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use std::error::Error;
use std::fs::{DirBuilder, File};
use std::io;

mod app;
mod ui;
use app::{App, CurrentScreen, CurrentlyEditing};
use ui::ui;

use crate::app::{EditingScreens, ValueType};

// Using stderr because stderr is piped differently than stdout
// this allows us to let users pipe the output of our program to a file
// we will render output to stderr and print our completed json to stdout
fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    // state creation and loop starting
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    //create app and run it
    let mut app = App::new();

    // TODO: Create writeable tmp file
    #[allow(unused_variables)]
    let tmp_file = "";

    let res = run_app(&mut terminal, &mut app);

    // application post-run steps
    // restore terminal state
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    // handle result of run_app
    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        // `draw()` returns a `Frame` which we pass to our `ui` function
        // with an immutable reference to the app to handle rendering
        terminal.draw(|frame| ui(frame, app))?;

        // polling for keyboard events - could set up a thread to handle this
        if let Event::Key(key) = event::read()? {
            // dbg!(key.code);
            if key.kind == event::KeyEventKind::Release {
                // skips events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('s') => app.current_screen = CurrentScreen::Selection,
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing(EditingScreens::Default);
                        app.currently_editing = Some(CurrentlyEditing::Key);
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Quitting;
                    }
                    _ => {}
                },
                CurrentScreen::Selection => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.current_screen = CurrentScreen::Main,
                    _ => {}
                },
                CurrentScreen::Quitting => match key.code {
                    KeyCode::Char('y') | KeyCode::Enter | KeyCode::Tab => {
                        return Ok(true); // signal to print JSON
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') | KeyCode::Esc => {
                        return Ok(false); // signal to not print JSON
                    }
                    _ => {}
                },
                CurrentScreen::Editing(EditingScreens::Default)
                    if key.kind == KeyEventKind::Press =>
                {
                    match key.code {
                        KeyCode::Enter => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        app.currently_editing = Some(CurrentlyEditing::Value);
                                    }
                                    CurrentlyEditing::Value => {
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
                            app.current_screen = CurrentScreen::Main;
                            app.currently_editing = None; // exit editing mode
                        }
                        KeyCode::Tab => {
                            app.toggle_editing();
                        }
                        KeyCode::Char(value) => {
                            if let Some(editing) = &app.currently_editing {
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
                        _ => {}
                    }
                }
                CurrentScreen::Editing(EditingScreens::Array)
                    if key.kind == KeyEventKind::Press =>
                {
                    // KEYMAP TO ADD ANOTHER ITEM
                    if key.code == KeyCode::Char('n')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
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
                CurrentScreen::Editing(EditingScreens::Object)
                    if key.kind == KeyEventKind::Press =>
                {
                    match key.code {
                        KeyCode::Enter => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        app.currently_editing = Some(CurrentlyEditing::Value);
                                    }
                                    CurrentlyEditing::Value => {
                                        app.save_key_value();
                                        app.current_screen = CurrentScreen::Main;
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
                        KeyCode::Char(value) => {
                            if let Some(editing) = &app.currently_editing {
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
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

#[allow(dead_code)]
fn create_tmp_file() -> Option<File> {
    use std::fs::exists;
    use std::path::PathBuf;

    let home_dir: PathBuf = PathBuf::from("~/");

    let cache_dir = match exists(home_dir) {
        Ok(b) => {
            if b {
                let builder = DirBuilder::new()
                    .recursive(true)
                    .create(home_dir.join(".cache").join("ratatui-json-editor"));
                match builder {
                    Ok(()) => Some(home_dir.join(".cache").join("simple-json")),
                    _ => None,
                };
            } else {
                None;
            }
        }
        Err(e) => None,
    };
}
