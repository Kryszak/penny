#[cfg(test)]
mod tests {
    use std::fs::File;

    use id3::{Tag, TagLike};
    use tempdir::TempDir;

    use crate::{files::FileEntry, player::MetadataReader};

    #[test]
    fn should_extract_mp3_metadata() -> Result<(), Box<dyn std::error::Error>> {
        // given
        let tmp_dir = TempDir::new("metadata")?;
        let dummy_mp3_file_path = tmp_dir.path().join("song.mp3");
        prepare_dummy_mp3(&dummy_mp3_file_path.to_string_lossy())?;
        let file_entry = FileEntry::new(&dummy_mp3_file_path);

        // when
        let result = MetadataReader::read_metadata(&file_entry);

        // then
        assert!(result.is_some());
        let metadata = result.unwrap();
        assert_eq!(metadata.title, Some(String::from("Awesome medley")));
        assert_eq!(metadata.artist, Some(String::from("Rockers")));

        Ok(())
    }

    fn prepare_dummy_mp3(path: &str) -> Result<(), Box<dyn std::error::Error>> {
        File::create(path)?;

        let mut tag = Tag::new();
        tag.set_title("Awesome medley");
        tag.set_artist("Rockers");

        tag.write_to_path(path, id3::Version::Id3v24)?;

        Ok(())
    }
}
