use std::{io, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{info, LevelFilter};
use tui::{backend::CrosstermBackend, Terminal};

use crate::{app_state::AppState, ui};

pub fn run_app(app_state: &mut AppState) -> io::Result<()> {
    let stdout = io::stdout();
    enable_raw_mode()?;
    // Configure log
    tui_logger::init_logger(LevelFilter::Debug).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Debug);

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    execute!(
        terminal.backend_mut(),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;

    terminal.clear()?;
    terminal.hide_cursor()?;
    info!("Welcome to penny!");

    loop {
        terminal.draw(|f| ui::ui(f, app_state))?;

        if crossterm::event::poll(Duration::from_millis(200)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('h') => app_state.help_visible = !app_state.help_visible,
                    KeyCode::Char('l') => app_state.logs_visible = !app_state.logs_visible,
                    KeyCode::Left => app_state.file_list.go_directory_up(),
                    KeyCode::Down => app_state.file_list.next(),
                    KeyCode::Up => app_state.file_list.previous(),
                    KeyCode::Right => app_state.file_list.enter_directory(),
                    KeyCode::Char('f') => app_state.file_list.focus(),
                    _ => {}
                }
            }
        }
    }

    terminal.clear()?;
    terminal.show_cursor()?;
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}
