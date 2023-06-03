use std::sync::{Arc, Mutex};

use events::EventBus;
use log::{info, LevelFilter};
use tui::style::Color;

use super::visualization_state::BarChartData;
use super::{actions::Action, visualization_state::ChartData};
use crate::input::events;
use crate::queue::queue_view::QueueView;
use crate::{cli::config::Config, files::FileViewerList, player::Mp3Player};

pub struct AppState {
    pub help_visible: bool,
    pub logs_visible: bool,
    pub file_viewer_focused: bool,
    pub log_level: LevelFilter,
    pub visualization_style: VisualizationStyle,
    pub color_style: Color,
    pub band_count: usize,
}

/// Types of visualization style available in app
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
    pub queue_view: QueueView,
    pub player: Mp3Player,
}

impl App {
    pub fn new(config: &Config, events: Arc<Mutex<EventBus>>) -> Option<Self> {
        let log_level = match config.debug {
            true => log::LevelFilter::Debug,
            false => log::LevelFilter::Info,
        };
        FileViewerList::with_directory(&config.starting_directory).map(|file_list| App {
            state: AppState {
                help_visible: true,
                logs_visible: config.debug,
                file_viewer_focused: true,
                log_level,
                visualization_style: VisualizationStyle::Bar {
                    data: BarChartData::new(config.band_count),
                },
                color_style: config.color.to_tui_color(),
                band_count: config.band_count,
            },
            file_list,
            queue_view: QueueView::new(),
            player: Mp3Player::new(events),
        })
    }

    /// Dispatch action and return information to continue or terminate app
    pub fn do_action(&mut self, action: Action) -> AppActionResult {
        match action {
            Action::Quit => return AppActionResult::Exit,
            Action::ToggleHelp => self.state.help_visible = !self.state.help_visible,
            Action::ToggleLogs => self.state.logs_visible = !self.state.logs_visible,
            Action::ChangeViewFocus => self.handle_lists_focus_change(),
            Action::ViewerUp | Action::ViewerDown => self.handle_list_item_change(action),
            Action::FileViewerDirUp | Action::FileViewerEnterDir => {
                self.handle_file_list_directory_change(action)
            }
            Action::Select => self.handle_list_item_select(),
            Action::TogglePlayback | Action::StopPlayback => {
                self.player.handle_action(action);
            }
            Action::ChangeVisualization => self.change_visualization_style(),
            Action::ChangeColor => self.change_color(),
            Action::OnSongFinished => self.handle_song_finished(),
            Action::DeleteFromQueue => self.handle_delete_from_queue(action),
            Action::PlayNextFromQueue | Action::PlayPreviousFromQueue => {
                self.handle_play_from_queue(action)
            }
        };

        AppActionResult::Continue
    }

    fn handle_lists_focus_change(&mut self) {
        self.state.file_viewer_focused = !self.state.file_viewer_focused;
        self.file_list.toggle_focus();
        self.queue_view.toggle_focus();
    }

    fn handle_list_item_change(&mut self, action: Action) {
        match self.state.file_viewer_focused {
            true => self.file_list.do_action(action),
            false => self.queue_view.do_action(action),
        }
    }

    fn handle_file_list_directory_change(&mut self, action: Action) {
        if self.state.file_viewer_focused {
            self.file_list.do_action(action);
        }
    }

    fn handle_list_item_select(&mut self) {
        match self.state.file_viewer_focused {
            true => {
                if let Some(file_entry) = self.file_list.get_selected_file_entry() {
                    if !file_entry.is_file {
                        return;
                    }
                    self.queue_view.add(file_entry);
                    if self.queue_view.items.len() > 1 {
                        return;
                    }
                    self.queue_view.do_action(Action::ViewerDown);
                    self.update_currently_playing();
                }
            }
            false => {
                self.update_currently_playing();
            }
        }
    }

    fn handle_delete_from_queue(&mut self, action: Action) {
        if self.state.file_viewer_focused {
            return;
        }
        let currently_playing = self.queue_view.now_playing;
        let removed_index = self.queue_view.state.selected();
        self.queue_view.do_action(action);
        if currently_playing == removed_index {
            self.update_currently_playing();
        }
    }

    fn handle_song_finished(&mut self) {
        info!("Playing next song from queue...");
        self.queue_view.do_action(Action::ViewerDown);
        self.update_currently_playing();
    }

    fn handle_play_from_queue(&mut self, action: Action) {
        self.queue_view.do_action(action);
        self.update_currently_playing();
    }

    fn update_currently_playing(&mut self) {
        if let Some(selected_song) = self.queue_view.get_selected_file_entry() {
            self.player.set_song_file(selected_song.clone());
            self.queue_view.now_playing = self.queue_view.state.selected();
            self.player.handle_action(Action::TogglePlayback);
        }
    }

    fn change_visualization_style(&mut self) {
        match &self.state.visualization_style {
            VisualizationStyle::Bar { data: _ } => {
                self.state.visualization_style = VisualizationStyle::Chart {
                    data: ChartData::new(self.state.band_count),
                }
            }
            VisualizationStyle::Chart { data: _ } => {
                self.state.visualization_style = VisualizationStyle::Bar {
                    data: BarChartData::new(self.state.band_count),
                }
            }
        }
    }

    fn change_color(&mut self) {
        match &self.state.color_style {
            Color::Cyan => self.state.color_style = Color::Red,
            Color::Red => self.state.color_style = Color::Magenta,
            Color::Magenta => self.state.color_style = Color::Green,
            Color::Green => self.state.color_style = Color::Blue,
            Color::Blue => self.state.color_style = Color::Cyan,
            Color::Indexed(_)
            | Color::Rgb(_, _, _)
            | Color::White
            | Color::LightRed
            | Color::LightCyan
            | Color::LightMagenta
            | Color::LightBlue
            | Color::LightYellow
            | Color::LightGreen
            | Color::DarkGray
            | Color::Gray
            | Color::Yellow
            | Color::Black
            | Color::Reset => (),
        }
    }
}
