use log::{error, trace};
use ratatui::widgets::ListState;

use crate::{application::actions::Action, files::FileEntry, queue::SongFile};

pub struct QueueView {
    pub state: ListState,
    pub items: Vec<SongFile>,
    pub now_playing: Option<usize>,
}

impl QueueView {
    pub fn new() -> Self {
        QueueView {
            state: ListState::default(),
            items: vec![],
            now_playing: None,
        }
    }

    pub fn do_action(&mut self, action: Action) {
        match action {
            Action::ViewerUp => self.previous(),
            Action::ViewerDown => self.next(),
            Action::DeleteFromQueue => self.remove_selected(),
            Action::PlayNextFromQueue => self.update_now_playing(UpdateDirection::Next),
            Action::PlayPreviousFromQueue => self.update_now_playing(UpdateDirection::Previous),
            _ => error!("Unsupported queue viewer action: {action:?}"),
        }
    }

    pub fn toggle_focus(&mut self) {
        match self.now_playing {
            Some(index) => {
                self.state.select(Some(index));
                trace!("Queue viewer lost focus");
            }
            None => {
                trace!("Queue viewer received focus");
            }
        }
    }

    pub fn add(&mut self, file_entry: &FileEntry) {
        self.items.push(SongFile::new(file_entry));
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => self.get_next_index_for(i),
            None => 0,
        };
        if !self.items.is_empty() {
            self.state.select(Some(i));
        }
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => self.get_previous_index_for(i),
            None => 0,
        };
        if !self.items.is_empty() {
            self.state.select(Some(i));
        }
    }

    fn remove_selected(&mut self) {
        let adjust_now_playing = match (self.now_playing, self.state.selected()) {
            (Some(now_playing_index), Some(selected_index)) => now_playing_index > selected_index,
            (None, None) | (None, Some(_)) | (Some(_), None) => false,
        };
        if let Some(index) = self.state.selected() {
            self.items.remove(index);
            if !self.items.is_empty() && self.items.len() > index {
                self.state.select(Some(index));
            } else {
                self.focus_first_entry_if_available();
            }
        }
        if !adjust_now_playing {
            return;
        }
        if let Some(index) = self.now_playing {
            let adjusted_now_playing_index = self.get_previous_index_for(index);
            self.now_playing = Some(adjusted_now_playing_index);
        }
    }

    pub fn get_selected_file_entry(&self) -> Option<&SongFile> {
        self.state.selected().map(|i| &self.items[i])
    }

    pub fn get_now_playing_entry(&self) -> Option<&SongFile> {
        self.now_playing.map(|i| &self.items[i])
    }

    fn update_now_playing(&mut self, direction: UpdateDirection) {
        if let Some(index) = self.now_playing {
            match direction {
                UpdateDirection::Next => {
                    self.now_playing = Some(self.get_next_index_for(index));
                }
                UpdateDirection::Previous => {
                    self.now_playing = Some(self.get_previous_index_for(index));
                }
            }
            self.state.select(self.now_playing);
        }
    }

    fn focus_first_entry_if_available(&mut self) {
        if !self.items.is_empty() {
            self.state.select(Some(0));
        } else {
            self.state = ListState::default();
        }
    }

    fn get_previous_index_for(&self, index: usize) -> usize {
        if index == 0 {
            self.items.len() - 1
        } else {
            index - 1
        }
    }

    fn get_next_index_for(&self, index: usize) -> usize {
        if index >= self.items.len() - 1 {
            0
        } else {
            index + 1
        }
    }
}

enum UpdateDirection {
    Next,
    Previous,
}
