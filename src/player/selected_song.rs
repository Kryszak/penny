use super::{
    duration_formatter::{DurationFormat, DurationFormatter},
    metadata::Mp3Metadata,
    MetadataReader,
};
use crate::files::FileEntry;
use std::time::Duration;

pub struct SelectedSongFile {
    pub metadata: Mp3Metadata,
    pub duration: Duration,
}

impl SelectedSongFile {
    pub fn new(file_entry: &FileEntry, duration: Duration) -> Self {
        SelectedSongFile {
            metadata: MetadataReader::read_metadata(file_entry).unwrap(),
            duration,
        }
    }

    pub fn display(&self) -> Vec<String> {
        let mut formatted = vec![];
        if let Some(a) = &self.metadata.artist {
            formatted.push(format!("Artist: {}", a));
        }
        match &self.metadata.title {
            Some(t) => formatted.push(format!("Title : {}", t)),
            None => formatted.push(format!("Title : {}", self.metadata.file_path)),
        }
        formatted.push(format!(
            "Duration: {}",
            self.duration.format(DurationFormat::MmSs)
        ));

        formatted
    }
}
