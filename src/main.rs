use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind,
};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use std::{
    error::Error,
    fs::{DirBuilder, File},
    io::{self, BufWriter},
};

mod app;
mod ui;
use app::{
    App, CurrentScreen, CurrentlyEditing, ValueType,
    screens::{
        match_array_editing, match_object_editing, match_selection_screen, match_string_editing,
    },
};
use ui::ui;

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

    // TODO: Create writeable tmp file
    // if create_tmp_file().is_none() {
    //     return restore_terminal(&mut terminal);
    // }
    //
    // #[allow(unused_variables, unused_mut)]
    // let mut tmp_file: BufWriter<File> = create_tmp_file().unwrap();
    // tmp_file.write_all(b"tedt").unwrap();
    // tmp_file.flush().unwrap();
    // let mut tmp_file = match create_tmp_file() {
    //     Some(file) => file,
    //     None => {
    //         println!("Failed to create temporary file for JSON storage");
    //         restore_terminal(terminal)?;
    //     }
    // };

    let res = run_app(&mut terminal, &mut app);

    // application post-run steps
    // restore terminal state
    restore_terminal(&mut terminal)?;

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
                CurrentScreen::Main | CurrentScreen::Start => match key.code {
                    KeyCode::Char('s') | KeyCode::Enter => {
                        app.current_screen = CurrentScreen::Selection
                    }
                    // KeyCode::Char('e') => {
                    //     app.current_screen = CurrentScreen::Editing(ValueType::default());
                    //     app.currently_editing = Some(CurrentlyEditing::Key);
                    // }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Quitting;
                    }
                    _ => {}
                },
                CurrentScreen::Selection => match_selection_screen(&key, app),
                CurrentScreen::Editing(ValueType::String) if key.kind == KeyEventKind::Press => {
                    match_string_editing(&key, app)
                }
                CurrentScreen::Editing(ValueType::Bool) if key.kind == KeyEventKind::Press => {
                    match_string_editing(&key, app)
                }
                CurrentScreen::Editing(ValueType::Number) if key.kind == KeyEventKind::Press => {
                    match_string_editing(&key, app)
                }
                CurrentScreen::Editing(ValueType::Array) if key.kind == KeyEventKind::Press => {
                    match_array_editing(&key, app)
                }
                CurrentScreen::Editing(ValueType::Object) if key.kind == KeyEventKind::Press => {
                    match_object_editing(&key, app)
                }
                CurrentScreen::Quitting => match key.code {
                    KeyCode::Char('y') | KeyCode::Enter | KeyCode::Tab => {
                        return Ok(true); // signal to print JSON
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') | KeyCode::Esc => {
                        return Ok(false); // signal to not print JSON
                    }
                    _ => {}
                },
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
