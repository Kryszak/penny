use std::{io, time::Duration};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{info, LevelFilter};
use tui::{backend::CrosstermBackend, Terminal};

use crate::{
    app::{ui, App, AppActionResult::Exit},
    input::{Events, InputEvent},
};

pub fn run_app(app_state: &mut App) -> io::Result<()> {
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

    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);

    loop {
        terminal.draw(|f| ui(f, app_state))?;

        match events.next().unwrap() {
            InputEvent::Input(key_code) => {
                if let Exit = app_state.do_action(key_code) {
                    break;
                }
            }
            InputEvent::Tick => {}
        };
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
