use super::{metadata::Mp3Metadata, MetadataReader};
use crate::files::FileEntry;
use std::time::Duration;

/// Information about currently selected song in mp3 player
pub struct SelectedSongFile {
    pub metadata: Mp3Metadata,
    pub duration: Duration,
    file_entry: FileEntry,
}

impl SelectedSongFile {
    pub fn new(file_entry: &FileEntry, duration: Duration) -> Self {
        SelectedSongFile {
            metadata: MetadataReader::read_metadata(file_entry).unwrap(),
            duration,
            file_entry: file_entry.clone(),
        }
    }

    /// Returns vector of information to be displayed about selected song
    /// Informations are strings in format `<label> : <value>`
    pub fn display(&self) -> Vec<String> {
        let mut formatted = vec![];
        if let Some(a) = &self.metadata.artist {
            formatted.push(format!("Artist: {}", a));
        }
        let title = match &self.metadata.title {
            Some(t) => t,
            None => &self.metadata.file_path,
        };
        formatted.push(format!("Title : {}", title));

        formatted
    }

    pub fn display_short(&self) -> String {
        let mut formatted = String::new();
        if let Some(a) = &self.metadata.artist {
            formatted.push_str(a);
        }
        let title = match &self.metadata.title {
            Some(t) => t,
            None => &self.metadata.file_path,
        };
        if formatted.is_empty() {
            formatted.push_str(title);
        } else {
            formatted.push_str(" - ");
            formatted.push_str(title);
        }

        formatted
    }
}
