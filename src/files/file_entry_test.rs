#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::files::FileEntry;

    #[test]
    fn should_create_file_entry() {
        // given
        let path = Path::new("src/files/file_entry_test.rs");

        // when
        let result = FileEntry::new(path);

        // then
        assert_eq!(result.path, "src/files/file_entry_test.rs");
        assert_eq!(result.name, "file_entry_test.rs");
        assert_eq!(result.is_file, true);
    }
}
