#[cfg(test)]
mod tests {
    use std::fs::File;

    use id3::{Tag, TagLike};
    use tempdir::TempDir;

    use crate::{files::FileEntry, queue::song_file::SongFile};

    #[test]
    fn should_format_with_artist_and_title() -> Result<(), Box<dyn std::error::Error>> {
        // given
        let tmp_dir = TempDir::new("metadata_with_tag")?;
        let dummy_mp3_file_path = tmp_dir.path().join("song_2.mp3");
        prepare_dummy_mp3(&dummy_mp3_file_path.to_string_lossy())?;
        let file_entry = FileEntry::new(&dummy_mp3_file_path);

        // when
        let result = SongFile::new(&file_entry);

        // then
        assert_eq!(
            result.display(),
            vec![
                String::from("Artist: Rockers"),
                String::from("Title : My song 2")
            ]
        );

        Ok(())
    }

    #[test]
    fn should_format_with_filename() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = TempDir::new("metadata_without_tags")?;
        let dummy_mp3_file_path = tmp_dir.path().join("song_tagless.mp3");
        File::create(&dummy_mp3_file_path)?;
        let file_entry = FileEntry::new(&dummy_mp3_file_path);

        // when
        let result = SongFile::new(&file_entry);

        // then
        assert_eq!(
            result.display(),
            vec![format!("Title : {}", dummy_mp3_file_path.to_string_lossy())]
        );

        Ok(())
    }

    fn prepare_dummy_mp3(path: &str) -> Result<(), Box<dyn std::error::Error>> {
        File::create(path)?;

        let mut tag = Tag::new();
        tag.set_title("My song 2");
        tag.set_artist("Rockers");

        tag.write_to_path(path, id3::Version::Id3v24)?;

        Ok(())
    }
}
