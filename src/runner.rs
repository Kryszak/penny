use crate::{
    application::{
        actions::{Action, Actions},
        ui, App,
        AppActionResult::Exit,
    },
    input::{AppEvent, EventBus},
};
use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, SetTitle,
    },
    ExecutableCommand,
};
use log::{info, LevelFilter};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io,
    sync::{Arc, Mutex},
};

/// Application runner handling terminal setup as well as managing app lifetime
pub fn run_app(app: &mut App, events: Arc<Mutex<EventBus>>) -> io::Result<()> {
    let stdout = io::stdout();
    enable_raw_mode()?;

    tui_logger::init_logger(LevelFilter::Trace).unwrap();
    tui_logger::set_default_level(app.state.log_level);

    let mut backend = CrosstermBackend::new(stdout);
    backend.execute(SetTitle("penny"))?;
    let mut terminal = Terminal::new(backend)?;
    execute!(
        terminal.backend_mut(),
        EnterAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.clear()?;
    terminal.hide_cursor()?;
    info!("Welcome to penny!");

    loop {
        terminal.draw(|f| ui(f, app))?;

        let mut events_ref = events.lock().unwrap();

        match events_ref.next() {
            AppEvent::Input(key_code) => {
                if let Some(action) = Actions::from(key_code) {
                    if let Exit = app.do_action(action) {
                        app.do_action(Action::StopPlayback);
                        events_ref.close();
                        break;
                    }
                }
            }
            AppEvent::Tick => {}
            AppEvent::Playback(event) => {
                app.do_action(Actions::from_event(event));
            }
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
