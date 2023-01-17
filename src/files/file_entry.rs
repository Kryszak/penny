use std::path::Path;

/// Structure representing file in file viewer.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FileEntry {
    /// Full path of given file
    pub path: String,
    /// File name for display
    pub name: String,
    /// Indicator for file/directory recognition
    pub is_file: bool,
}

impl FileEntry {
    pub fn new(path: &Path) -> Self {
        let path_string = path.to_string_lossy().to_string();
        let fallback_file_name = path_string.clone();
        FileEntry {
            path: path_string,
            name: path
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or(fallback_file_name),
            is_file: path.is_file(),
        }
    }
}
