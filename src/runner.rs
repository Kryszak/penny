use crate::{
    application::{actions::Actions, ui, App, AppActionResult::Exit},
    input::{Events, InputEvent},
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{info, LevelFilter};
use std::{io, sync::Arc, time::Duration};
use tokio::sync::Mutex;
use tui::{backend::CrosstermBackend, Terminal};

pub async fn run_app(app_state: &Arc<Mutex<App>>) -> io::Result<()> {
    let stdout = io::stdout();
    enable_raw_mode()?;

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
    let mut events = Events::new(tick_rate);

    loop {
        let mut app = app_state.lock().await;
        terminal.draw(|f| ui(f, &mut app))?;

        match events.next().await {
            InputEvent::Input(key_code) => {
                if let Some(action) = Actions::from(key_code) {
                    if let Exit = app.do_action(action) {
                        events.close();
                        break;
                    }
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
