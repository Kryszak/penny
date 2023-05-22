use std::time::Duration;

use log::{error, trace};
use ratatui::widgets::ListState;

use crate::{application::actions::Action, files::FileEntry, player::SelectedSongFile};

pub struct QueueView {
    pub state: ListState,
    pub items: Vec<SelectedSongFile>,
    previously_selected_index: Option<usize>,
}

impl QueueView {
    pub fn new() -> Self {
        QueueView {
            state: ListState::default(),
            items: vec![],
            previously_selected_index: None,
        }
    }

    pub fn do_action(&mut self, action: Action) {
        match action {
            Action::ViewerUp => self.previous(),
            Action::ViewerDown => self.next(),
            _ => error!("Unsupported queue viewer action: {:?}", action),
        }
    }

    pub fn toggle_focus(&mut self) {
        match self.state.selected() {
            Some(_) => {
                self.previously_selected_index = self.state.selected();
                self.state = ListState::default();
                trace!("Queue viewer lost focus");
            }
            None => {
                match self.previously_selected_index {
                    Some(_) => self.state.select(self.previously_selected_index),
                    None => self.focus_first_entry_if_available(),
                }
                trace!("Queue viewer received focus");
            }
        };
    }

    pub fn add(&mut self, file_entry: &FileEntry) {
        self.items
            .push(SelectedSongFile::new(&file_entry, Duration::ZERO));
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        if !self.items.is_empty() {
            self.state.select(Some(i));
        }
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        if !self.items.is_empty() {
            self.state.select(Some(i));
        }
    }

    pub fn get_selected_file_entry(&self) -> Option<&SelectedSongFile> {
        self.state.selected().map(|i| &self.items[i])
    }

    fn focus_first_entry_if_available(&mut self) {
        if !self.items.is_empty() {
            self.state.select(Some(0));
        } else {
            self.state = ListState::default();
        }
    }
}
