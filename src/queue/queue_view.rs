use log::{error, trace};
use ratatui::widgets::ListState;

use crate::{application::actions::Action, files::FileEntry, queue::SongFile};

pub struct QueueView {
    pub state: ListState,
    pub items: Vec<SongFile>,
}

impl QueueView {
    pub fn new() -> Self {
        QueueView {
            state: ListState::default(),
            items: vec![],
        }
    }

    pub fn do_action(&mut self, action: Action) {
        match action {
            Action::ViewerUp => self.previous(),
            Action::ViewerDown => self.next(),
            Action::DeleteFromQueue => self.remove_current(),
            _ => error!("Unsupported queue viewer action: {:?}", action),
        }
    }

    pub fn toggle_focus(&mut self) {
        match self.state.selected() {
            Some(_) => {
                trace!("Queue viewer lost focus");
            }
            None => {
                self.focus_first_entry_if_available();
                trace!("Queue viewer received focus");
            }
        };
    }

    pub fn add(&mut self, file_entry: &FileEntry) {
        self.items.push(SongFile::new(file_entry));
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

    fn remove_current(&mut self) {
        if let Some(index) = self.state.selected() {
            self.items.remove(index);
            if !self.items.is_empty() && self.items.len() > index {
                self.state.select(Some(index));
            } else {
                self.focus_first_entry_if_available();
            }
        }
    }

    pub fn get_selected_file_entry(&self) -> Option<&SongFile> {
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
