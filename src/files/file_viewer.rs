use log::{error, trace};
use ratatui::widgets::ListState;
use std::{fs, io, path::Path};

use crate::application::actions::Action;

use super::FileEntry;

/// File viewer for traversing filesystem and selecting mp3 files
/// for playback
pub struct FileViewerList {
    /// State of file viewer, also used for correct rendering
    pub state: ListState,
    /// Current directory contents listed
    /// Only contains directories and mp3 files
    pub items: Vec<FileEntry>,
    pub current_directory: String,
    /// Contains index of item selected before file viewer
    /// lost it's focus
    previously_selected_index: Option<usize>,
    /// Contains index of selected item in parent directory.
    /// Used to focus item when going to parent dir
    parent_selected_index: Option<usize>,
}

impl FileViewerList {
    /// Creates new File Viewer for given directory
    pub fn with_directory(dir_name: &str) -> Option<Self> {
        FileViewerList::list_directory_content(dir_name)
            .map(|entries| FileViewerList {
                items: entries,
                state: ListState::default(),
                current_directory: dir_name.to_string(),
                previously_selected_index: None,
                parent_selected_index: None,
            })
            .map(|mut viewer| {
                viewer.focus_first_entry_if_available();
                viewer
            })
            .ok()
    }

    pub fn do_action(&mut self, action: Action) {
        match action {
            Action::ViewerUp => self.previous(),
            Action::ViewerDown => self.next(),
            Action::FileViewerDirUp => self.go_directory_up(),
            Action::FileViewerEnterDir => self.enter_directory(),
            _ => error!("Unsupported file viewer action: {:?}", action),
        }
    }

    /// Focuses file viewer allowing moving through filesystem and file picking
    pub fn toggle_focus(&mut self) {
        match self.state.selected() {
            Some(_) => {
                self.previously_selected_index = self.state.selected();
                self.state = ListState::default();
                trace!("File viewer lost focus");
            }
            None => {
                match self.previously_selected_index {
                    Some(_) => self.state.select(self.previously_selected_index),
                    None => self.focus_first_entry_if_available(),
                }
                trace!("File viewer received focus");
            }
        };
    }

    /// Returns selected file if any is selected
    pub fn get_selected_file_entry(&self) -> Option<&FileEntry> {
        self.state.selected().map(|i| &self.items[i])
    }

    fn go_directory_up(&mut self) {
        let path = Path::new(&self.current_directory)
            .parent()
            .map(|dir| dir.to_string_lossy().into_owned());
        let new_path = match path {
            Some(x) => x,
            None => self.current_directory.to_string(),
        };
        self.current_directory = new_path;
        self.items = FileViewerList::list_directory_content(&self.current_directory).unwrap();
        match self.parent_selected_index {
            Some(_) => self.state.select(self.parent_selected_index),
            None => self.focus_first_entry_if_available(),
        };
        self.parent_selected_index = None;
    }

    fn enter_directory(&mut self) {
        let maybe_selected_entry = self.state.selected().map(|i| &self.items[i]);

        if let Some(entry) = maybe_selected_entry {
            if Path::new(&entry.path).is_file() {
                return;
            }
            match FileViewerList::list_directory_content(&entry.path) {
                Ok(items) => {
                    self.parent_selected_index = self.state.selected();
                    self.current_directory.clone_from(&entry.path);
                    self.items = items;
                    self.focus_first_entry_if_available();
                }
                Err(_) => error!(
                    "Missing permission to list files in directory {}!",
                    entry.path
                ),
            };
        };
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        if !self.items.is_empty() {
            self.state.select(Some(i));
        }
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        if !self.items.is_empty() {
            self.state.select(Some(i));
        }
    }

    fn list_directory_content(dir_name: &str) -> io::Result<Vec<FileEntry>> {
        let mut file_list = fs::read_dir(dir_name)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|e| !e.file_name().unwrap().to_string_lossy().starts_with('.'))
            .filter(|e| e.is_dir() || e.file_name().unwrap().to_string_lossy().ends_with(".mp3"))
            .map(|e| FileEntry::new(&e))
            .collect::<Vec<_>>();

        file_list.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(file_list)
    }

    fn focus_first_entry_if_available(&mut self) {
        if !self.items.is_empty() {
            self.state.select(Some(0));
        } else {
            self.state = ListState::default();
        }
    }
}
