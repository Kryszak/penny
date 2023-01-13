use log::LevelFilter;

pub struct AppState {
    pub help_visible: bool,
    pub logs_visible: bool,
    pub file_viewer_focused: bool,
    pub log_level: LevelFilter,
}
