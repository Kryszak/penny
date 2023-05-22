use std::time::Duration;

use ratatui::widgets::ListState;

use crate::{files::FileEntry, player::SelectedSongFile};

pub struct QueueView {
    pub state: ListState,
    pub items: Vec<SelectedSongFile>,
}

impl QueueView {
    pub fn new() -> Self {
        QueueView {
            state: ListState::default(),
            items: vec![],
        }
    }

    pub fn add(&mut self, file_entry: &FileEntry) {
        self.items
            .push(SelectedSongFile::new(&file_entry, Duration::ZERO));
    }
}
