use crate::files::FileEntry;

use super::{metadata::Mp3Metadata, MetadataReader};

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

pub struct Mp3Player {
    pub song: Option<SelectedSongFile>,
}

impl Mp3Player {
    pub fn new() -> Self {
        Mp3Player { song: None }
    }

    pub fn set_song_file(&mut self, file_entry: &FileEntry) {
        self.song = Some(SelectedSongFile::new(file_entry));
    }
}
