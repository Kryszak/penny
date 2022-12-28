use std::{io, time::Duration};

use crossterm::event::{self, KeyCode};
use log::info;
use tui::{backend::Backend, Terminal};

use crate::{ui, app_state::FileViewerList};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app_state = crate::app_state::AppState {
        help_visible: true,
        logs_visible: false,
        file_list: FileViewerList::with_directory("/home/kryszak/storage"),
    };

    info!("Welcome to penny!");

    loop {
        terminal.draw(|f| ui::ui(f, &mut app_state))?;

        if crossterm::event::poll(Duration::from_millis(200)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
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
}
