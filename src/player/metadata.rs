use crate::files::FileEntry;
use id3::{Tag, TagLike};
use std::path::Path;

pub struct Mp3Metadata {
    pub artist: Option<String>,
    pub title: Option<String>,
    pub file_path: String,
}

impl Mp3Metadata {
    fn new(file_name: &str, tag: Tag) -> Self {
        Mp3Metadata {
            artist: tag.artist().map(String::from),
            title: tag.title().map(String::from),
            file_path: String::from(file_name),
        }
    }

}

pub struct MetadataReader;

impl MetadataReader {
    pub fn read_metadata(file_entry: &FileEntry) -> Option<Mp3Metadata> {
        if Path::new(&file_entry.path).is_file() {
            let tag = Tag::read_from_path(&file_entry.path).unwrap_or_else(|_| Tag::new());
            return Some(Mp3Metadata::new(&file_entry.path, tag));
        }
        None
    }
}
