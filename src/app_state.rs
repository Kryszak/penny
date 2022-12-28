use log::{debug, error};
use std::{fs, io, path::Path};
use tui::widgets::ListState;

pub struct AppState {
    pub help_visible: bool,
    pub logs_visible: bool,
    pub file_list: FileViewerList,
}

pub struct FileViewerList {
    pub state: ListState,
    pub items: Vec<String>,
    pub current_directory: String,
    pub file_viewer_focused: bool,
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
            file_viewer_focused: false,
            previously_selected_index: None,
            parent_selected_index: None,
        }
    }

    pub fn go_directory_up(&mut self) {
        if self.file_viewer_focused {
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
    }

    pub fn enter_directory(&mut self) {
        if self.file_viewer_focused {
            let maybe_selected_path = self.state.selected().map(|i| &self.items[i]);

            if let Some(path) = maybe_selected_path {
                if Path::new(path).is_dir() {
                    match FileViewerList::list_directory_content(&path.to_string()) {
                        Ok(items) => {
                            self.parent_selected_index = self.state.selected();
                            self.current_directory = path.to_string();
                            self.items = items;
                            self.focus_first_entry_if_available();
                        }
                        Err(_) => error!(
                            "Missing permission to list files in directory {}!",
                            path.to_string()
                        ),
                    };
                }
            };
        }
    }

    pub fn next(&mut self) {
        if self.file_viewer_focused {
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
    }

    pub fn previous(&mut self) {
        if self.file_viewer_focused {
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
    }

    pub fn focus(&mut self) {
        match self.file_viewer_focused {
            true => {
                self.file_viewer_focused = false;
                self.previously_selected_index = self.state.selected();
                self.state = ListState::default();
                debug!("File viewer lost focus");
            }
            false => {
                self.file_viewer_focused = true;
                match self.previously_selected_index {
                    Some(_) => self.state.select(self.previously_selected_index),
                    None => self.focus_first_entry_if_available(),
                }
                debug!("File viewer received focus");
            }
        }
    }

    fn list_directory_content(dir_name: &str) -> io::Result<Vec<String>> {
        Ok(fs::read_dir(dir_name)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|e| !e.file_name().unwrap().to_string_lossy().starts_with('.'))
            .filter(|e| e.is_dir() || e.file_name().unwrap().to_string_lossy().ends_with(".mp3"))
            .map(|e| e.to_string_lossy().into_owned())
            .collect::<Vec<String>>())
    }

    fn focus_first_entry_if_available(&mut self) {
        if !self.items.is_empty() {
            self.state.select(Some(0));
        } else {
            self.state = ListState::default();
        }
    }
}
