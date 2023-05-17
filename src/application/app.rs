use log::LevelFilter;

use super::visualization_state::BarChartData;
use super::{actions::Action, visualization_state::ChartData};
use crate::{cli::config::Config, files::FileViewerList, player::Mp3Player};

const VISUALIZATION_BAR_COUNT: usize = 64;

pub struct AppState {
    pub help_visible: bool,
    pub logs_visible: bool,
    pub file_viewer_focused: bool,
    pub log_level: LevelFilter,
    pub visualization_style: VisualizationStyle,
}

pub enum VisualizationStyle {
    Bar { data: BarChartData },
    Chart { data: ChartData },
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
                visualization_style: VisualizationStyle::Bar {
                    data: BarChartData::new(VISUALIZATION_BAR_COUNT),
                },
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
            Action::ChangeVisualization => self.change_visualization_style(),
        };

        AppActionResult::Continue
    }

    fn change_visualization_style(&mut self) {
        match &self.state.visualization_style {
            VisualizationStyle::Bar { data: _ } => {
                self.state.visualization_style = VisualizationStyle::Chart {
                    data: ChartData::new(VISUALIZATION_BAR_COUNT),
                }
            }
            VisualizationStyle::Chart { data: _ } => {
                self.state.visualization_style = VisualizationStyle::Bar {
                    data: BarChartData::new(VISUALIZATION_BAR_COUNT),
                }
            }
        }
    }
}
