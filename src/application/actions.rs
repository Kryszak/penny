use crate::input::events::KeyPress;
use crossterm::event::KeyCode;

/// Actions available in app
#[derive(Debug)]
pub enum Action {
    Quit,
    ToggleHelp,
    ToggleLogs,
    FocusFileViewer,
    FileViewerUp,
    FileViewerDown,
    FileViewerDirUp,
    FileViewerEnterDir,
    SelectSongFile,
    TogglePlayback,
    StopPlayback,
    ChangeVisualization,
}

/// Translator for keypresses to actions inside of app
pub struct Actions;

impl Actions {
    /// Returns action for given key press, or `None` if keypress is not handled
    pub fn from(key_press: KeyPress) -> Option<Action> {
        match key_press.key {
            KeyCode::Char('q') => Some(Action::Quit),
            KeyCode::Char('h') => Some(Action::ToggleHelp),
            KeyCode::Char('l') => Some(Action::ToggleLogs),
            KeyCode::Left => Some(Action::FileViewerDirUp),
            KeyCode::Down => Some(Action::FileViewerDown),
            KeyCode::Up => Some(Action::FileViewerUp),
            KeyCode::Right => Some(Action::FileViewerEnterDir),
            KeyCode::Char('f') => Some(Action::FocusFileViewer),
            KeyCode::Enter => Some(Action::SelectSongFile),
            KeyCode::Char('p') => Some(Action::TogglePlayback),
            KeyCode::Char('s') => Some(Action::StopPlayback),
            KeyCode::Char('v') => Some(Action::ChangeVisualization),
            _ => None,
        }
    }
}
