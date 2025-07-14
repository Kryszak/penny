use crate::{
    files::FileEntry,
    player::{metadata::Mp3Metadata, MetadataReader},
};
use std::{path::Path, time::Duration};

/// Information about currently selected song in mp3 player
#[derive(Clone)]
pub struct SongFile {
    pub metadata: Mp3Metadata,
    pub duration: Duration,
    pub file_entry: FileEntry,
}

impl SongFile {
    pub fn new(file_entry: &FileEntry) -> Self {
        let duration =
            mp3_duration::from_path(Path::new(&file_entry.path)).unwrap_or(Duration::ZERO);
        SongFile {
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
            formatted.push(format!("Artist: {a}"));
        }
        let title = match &self.metadata.title {
            Some(t) => t,
            None => &self.metadata.file_path,
        };
        formatted.push(format!("Title : {title}"));

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
