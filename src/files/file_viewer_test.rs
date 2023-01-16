#[cfg(test)]
mod tests {
    use std::{fs::File, io};

    use tempdir::TempDir;

    use crate::files::{FileEntry, FileViewerList};

    #[test]
    fn should_create_file_viewer_for_given_dir() -> Result<(), io::Error> {
        // given
        let directory = TempDir::new("music")?;

        // when
        let result = FileViewerList::with_directory(directory.path().to_str().unwrap());

        // then
        assert!(result.is_some());

        Ok(())
    }

    #[test]
    fn should_list_music_files() -> Result<(), io::Error> {
        // given
        let directory = TempDir::new("music")?;
        let mp3_file_path = directory.path().join("song.mp3");
        File::create(&mp3_file_path)?;
        File::create(directory.path().join("random_file.txt"))?;

        // when
        let result = FileViewerList::with_directory(directory.path().to_str().unwrap()).unwrap();

        // then
        assert_eq!(result.items.len(), 1);
        assert!(result.items.contains(&FileEntry::new(&mp3_file_path)));

        Ok(())
    }
}
