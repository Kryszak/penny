use log::LevelFilter;

use super::actions::Action;
use crate::{cli::config::Config, files::FileViewerList, player::Mp3Player};

pub struct AppState {
    pub help_visible: bool,
    pub logs_visible: bool,
    pub file_viewer_focused: bool,
    pub log_level: LevelFilter,
    pub audio_spectrum: Vec<(&'static str, u64)>,
    pub audio_spectrum_band_count: usize,
}

/// Indicator used by app runner to continue running or terminate process
/// after completing an action
pub enum AppActionResult {
    Continue,
    Exit,
}

/// Structure keeping state of the application.
/// It's also responsible for dispatch of actions to it's components.
pub struct App {
    pub state: AppState,
    pub file_list: FileViewerList,
    pub player: Mp3Player,
}

impl App {
    pub fn new(config: &Config) -> Option<Self> {
        let log_level = match config.debug {
            true => log::LevelFilter::Debug,
            false => log::LevelFilter::Info,
        };
        FileViewerList::with_directory(&config.starting_directory).map(|file_list| App {
            state: AppState {
                help_visible: true,
                logs_visible: config.debug,
                file_viewer_focused: false,
                log_level,
                audio_spectrum: vec![],
                audio_spectrum_band_count: 32,
            },
            file_list,
            player: Mp3Player::new(),
        })
    }

    /// Dispatch action and return information to continue or terminate app
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
            Action::TogglePlayback | Action::StopPlayback => {
                self.player.handle_action(action);
            }
        };

        AppActionResult::Continue
    }

    /// Process raw spectrum from player for display
    /// Takes first half of frequencies, to display only audible range of frequencies (<20kHz)
    /// For display purposes, we divide that range into bands and sum values
    /// First two bins are skipped, as they always have high values
    pub fn update_spectrum(&mut self) {
        let unsigned_spectrum: Vec<u64> = self
            .player
            .get_audio_spectrum()
            .into_iter()
            .map(|v| v as u64)
            .collect();
        let usable_spectrum = &unsigned_spectrum[0..unsigned_spectrum.len() / 2];
        let band_count = self.state.audio_spectrum_band_count;
        if usable_spectrum.len() > band_count {
            let band_width = usable_spectrum.len() / band_count;
            self.state.audio_spectrum = usable_spectrum
                .chunks(band_width)
                .skip(2)
                .map(|chunk| ("", chunk.iter().copied().reduce(|a, b| a + b).unwrap_or(0)))
                .collect();
        } else {
            self.state.audio_spectrum = vec![];
        }
    }
}
