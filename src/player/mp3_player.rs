use super::{metadata::Mp3Metadata, MetadataReader};
use crate::{application::actions::Action, files::FileEntry};
use log::{debug, error};

pub struct SelectedSongFile {
    pub metadata: Mp3Metadata,
}

impl SelectedSongFile {
    fn new(file_entry: &FileEntry) -> Self {
        SelectedSongFile {
            metadata: MetadataReader::read_metadata(file_entry).unwrap(),
        }
    }
}

#[derive(PartialEq)]
enum PlayerState {
    New,
    SongSelected,
    Playing,
    Paused,
}

impl PlayerState {
    fn can_start_playback(&self) -> bool {
        *self == PlayerState::SongSelected || *self == PlayerState::Paused
    }
}

pub struct Mp3Player {
    pub song: Option<SelectedSongFile>,
    state: PlayerState,
}

impl Mp3Player {
    pub fn new() -> Self {
        Mp3Player {
            song: None,
            state: PlayerState::New,
        }
    }

    pub fn set_song_file(&mut self, file_entry: &FileEntry) {
        self.song = Some(SelectedSongFile::new(file_entry));
        self.state = PlayerState::SongSelected;
        self.toggle_playback();
    }

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::TogglePlayback => self.toggle_playback(),
            _ => error!("Action {:?} is not supported for Mp3Player!", action),
        }
    }

    fn toggle_playback(&mut self) {
        if self.state.can_start_playback() {
            debug!(
                "Starting playback of {:?}",
                self.song.as_ref().unwrap().metadata.display()
            );
            self.state = PlayerState::Playing;
        } else if self.state == PlayerState::Playing {
            self.state = PlayerState::Paused;
            debug!("Paused playback");
        }
    }
}
