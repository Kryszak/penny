use super::{actions::Action, FileEntry};
use log::{debug, error};
use std::{fs, io, path::Path};
use tui::widgets::ListState;

pub struct FileViewerList {
    pub state: ListState,
    pub items: Vec<FileEntry>,
    pub current_directory: String,
    previously_selected_index: Option<usize>,
    parent_selected_index: Option<usize>,
}

impl FileViewerList {
    pub fn with_directory(dir_name: &str) -> Self {
        FileViewerList {
            items: FileViewerList::list_directory_content(dir_name)
                .expect("Failed to open user $HOME directory"),
            state: ListState::default(),
            current_directory: dir_name.to_string(),
            previously_selected_index: None,
            parent_selected_index: None,
        }
    }

    pub fn do_action(&mut self, action: Action) {
        match action {
            Action::FileViewerUp => self.previous(),
            Action::FileViewerDown => self.next(),
            Action::FileViewerDirUp => self.go_directory_up(),
            Action::FileViewerEnterDir => self.enter_directory(),
            _ => panic!("Unsupported file viewer action: {:?}", action),
        }
    }

    pub fn focus(&mut self) {
        match self.state.selected() {
            Some(_) => {
                self.previously_selected_index = self.state.selected();
                self.state = ListState::default();
                debug!("File viewer lost focus");
            }
            None => {
                match self.previously_selected_index {
                    Some(_) => self.state.select(self.previously_selected_index),
                    None => self.focus_first_entry_if_available(),
                }
                debug!("File viewer received focus");
            }
        };
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
            if Path::new(&entry.path).is_dir() {
                match FileViewerList::list_directory_content(&entry.path) {
                    Ok(items) => {
                        self.parent_selected_index = self.state.selected();
                        self.current_directory = entry.path.clone();
                        self.items = items;
                        self.focus_first_entry_if_available();
                    }
                    Err(_) => error!(
                        "Missing permission to list files in directory {}!",
                        entry.path
                    ),
                };
            }
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
