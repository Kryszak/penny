use super::{AppState, FileViewerList};
use crate::input::events::KeyPress;
use crossterm::event::KeyCode;
use std::env;

pub enum AppActionResult {
    Continue,
    Exit,
}

pub struct App {
    pub state: AppState,
    pub file_list: FileViewerList,
}

impl App {
    pub fn new() -> Self {
        App {
            state: AppState {
                help_visible: true,
                logs_visible: true,
            },
            file_list: FileViewerList::with_directory(&env::var("HOME").unwrap()),
        }
    }

    pub fn do_action(&mut self, key_press: KeyPress) -> AppActionResult {
        match key_press.key {
            KeyCode::Char('q') => return AppActionResult::Exit,
            KeyCode::Char('h') => self.state.help_visible = !self.state.help_visible,
            KeyCode::Char('l') => self.state.logs_visible = !self.state.logs_visible,
            KeyCode::Left => self.file_list.go_directory_up(),
            KeyCode::Down => self.file_list.next(),
            KeyCode::Up => self.file_list.previous(),
            KeyCode::Right => self.file_list.enter_directory(),
            KeyCode::Char('f') => self.file_list.focus(),
            _ => {}
        };

        AppActionResult::Continue
    }
}
