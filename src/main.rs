#[allow(clippy::single_component_path_imports)]
pub(crate) use ratatui;
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{
            self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind,
            KeyModifiers,
        },
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};

use std::{
    error::Error,
    fs::{DirBuilder, File},
    io::{self, BufWriter},
};

mod app;
mod file_state;
mod traits;
mod ui;
use app::{
    App, CurrentScreen, CurrentlyEditing, ValueType,
    screens::{
        match_array_editing, match_bool_editing, match_file_screen, match_num_editing,
        match_object_editing, match_selection_screen, match_start_screen, match_string_editing,
    },
};
use file_state::FileState;
use ui::ui;

use crate::app::screens::match_quitting_screen;

#[allow(unused)]
const TMP_JSON_FILE: &str = "tmp_json_file.json";

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
    let mut file_state = FileState::default();

    let res = run_app(&mut terminal, &mut app, &mut file_state);

    // application post-run steps
    // restore terminal state
    restore_terminal(&mut terminal)?;

    // handle result of run_app
    if let Ok(do_print) = res {
        if do_print {
            // app.print_json()?;
            app.write_file(&mut file_state)?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    file_state: &mut FileState,
) -> io::Result<bool> {
    loop {
        // `draw()` returns a `Frame` which we pass to our `ui` function
        // with an immutable reference to the app to handle rendering
        terminal.draw(|frame| ui(frame, app, file_state))?;

        // polling for keyboard events - could set up a thread to handle this
        if let Event::Key(key) = event::read()? {
            // dbg!(key.code);
            if key.kind == event::KeyEventKind::Release {
                // skips events that are not KeyEventKind::Press
                continue;
            }
            if let CurrentScreen::Editing(_) | CurrentScreen::Main = app.current_screen {
                if key.code == KeyCode::Char('s') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    // `store_array_values` updates the editing_preview
                    if !app.object_values.values.is_empty() {
                        app.editing_object = false;
                        app.value_type = ValueType::Object;
                        app.save_key_value();
                    }
                }
            }
            match app.current_screen {
                CurrentScreen::FilePrompt if key.kind == KeyEventKind::Press => {
                    match_file_screen(&key, app, file_state);
                }
                CurrentScreen::Start if key.kind == KeyEventKind::Press => {
                    match_start_screen(&key, app)
                }
                CurrentScreen::Main if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('s') | KeyCode::Enter => {
                        app.current_screen = CurrentScreen::Selection
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Quitting;
                    }
                    _ => {}
                },
                CurrentScreen::Selection => match_selection_screen(&key, app),
                CurrentScreen::Editing(ValueType::String) if key.kind == KeyEventKind::Press => {
                    match_string_editing(&key, app)
                }
                CurrentScreen::Editing(ValueType::Bool(_)) if key.kind == KeyEventKind::Press => {
                    match_bool_editing(&key, app)
                }
                CurrentScreen::Editing(ValueType::Number) if key.kind == KeyEventKind::Press => {
                    match_num_editing(&key, app)
                }
                CurrentScreen::Editing(ValueType::Array) if key.kind == KeyEventKind::Press => {
                    match_array_editing(&key, app)
                }
                CurrentScreen::Editing(ValueType::Object) if key.kind == KeyEventKind::Press => {
                    match_object_editing(&key, app)
                }
                CurrentScreen::Quitting if key.kind == KeyEventKind::Press => {
                    match match_quitting_screen(&key, app) {
                        Ok(b) => return Ok(b),
                        Err(e) => return Err(e),
                    }
                }
                _ => {}
            }
        }
    }
}

fn restore_terminal<T: Backend + std::io::Write>(
    terminal: &mut Terminal<T>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn _create_tmp_file() -> Option<BufWriter<File>> {
    use std::fs::exists;
    use std::path::PathBuf;

    // Check that this works so we can continue building off of it
    let mut home_dir: PathBuf = PathBuf::from("/home/thomas");
    // let mut home_dir: PathBuf = PathBuf::from("$HOME");
    match exists(&home_dir) {
        Ok(b) => {
            if !b {
                return None;
            }
        }
        Err(_) => return None,
    };
    // Recursively create the cache directory
    // using `expect` because it won't error
    // and we check for existence below
    home_dir.push(".cache/simple_json");
    DirBuilder::new()
        .recursive(true)
        .create(&home_dir)
        .expect("Won't error because of recursive mode");

    if let Ok(b) = exists(&home_dir) {
        match b {
            true => {
                home_dir.push(TMP_JSON_FILE);
                let tmp_file = File::options()
                    .read(true)
                    // .write(true)
                    .append(true)
                    .create(true)
                    // .truncate(true)
                    .open(&home_dir);
                // tmp_file.ok();
                let file: BufWriter<File> = BufWriter::new(tmp_file.unwrap());
                Some(file)
            }
            false => None,
        }
    } else {
        None
    }
}
