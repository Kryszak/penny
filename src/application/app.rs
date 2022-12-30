use super::{actions::Action, AppState};
use crate::{files::FileViewerList, player::Mp3Player};
use std::env;

pub enum AppActionResult {
    Continue,
    Exit,
}

pub struct App {
    pub state: AppState,
    pub file_list: FileViewerList,
    pub player: Mp3Player,
}

impl App {
    pub fn new() -> Self {
        App {
            state: AppState {
                help_visible: true,
                logs_visible: true,
                file_viewer_focused: false,
            },
            file_list: FileViewerList::with_directory(&env::var("HOME").unwrap()),
            player: Mp3Player::new(),
        }
    }

    pub fn do_action(&mut self, action: Action) -> AppActionResult {
        match action {
            Action::Quit => return AppActionResult::Exit,
            Action::ToggleHelp => self.state.help_visible = !self.state.help_visible,
            Action::ToggleLogs => self.state.logs_visible = !self.state.logs_visible,
            Action::FocusFileViewer => {
                self.state.file_viewer_focused = !self.state.file_viewer_focused;
                self.file_list.focus();
            }
            Action::FileViewerUp
            | Action::FileViewerDown
            | Action::FileViewerDirUp
            | Action::FileViewerEnterDir => {
                if self.state.file_viewer_focused {
                    self.file_list.do_action(action);
                }
            }
            Action::SelectSongFile => {
                if let Some(file_entry) = self.file_list.get_selected_file_entry() {
                    if file_entry.is_file {
                        self.player.set_song_file(file_entry);
                    }
                }
            }
        };

        AppActionResult::Continue
    }
}
