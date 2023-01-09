use crate::input::events::KeyPress;
use crossterm::event::KeyCode;

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
}

pub struct Actions;

impl Actions {
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
            _ => None,
        }
    }
}
