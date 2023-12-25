use std::error::Error;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind};
use crossterm::{event, execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;
use crate::app::{App, CurrentScreen};
use crate::sqlite::FlashCard;
use crate::ui::controller_ui::ui;

pub fn start() -> Result<(), Box<dyn Error>>{
    FlashCard::initialize_db()?;

    // setup terminal
    enable_raw_mode()?;
    let mut stderr = std::io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // run app
    let mut app = App::new();
    let result = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = result {
        // if do_print {
        //     app.print_json()?;
        // }
    } else if let Err(err) = result {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> std::io::Result<bool> {
    let mut fc = FlashCard::empty();

    loop {
        terminal.draw(|f| ui(f, app, &mut fc))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Enter => {
                        app.current_screen = CurrentScreen::Reveal;
                    },
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    },
                    KeyCode::Char('a') => {
                        app.current_screen = CurrentScreen::Adding
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}