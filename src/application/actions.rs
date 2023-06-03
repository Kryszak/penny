use crate::input::events::{KeyPress, PlaybackEvent};
use crossterm::event::KeyCode;

/// Actions available in app
#[derive(Debug)]
pub enum Action {
    Quit,
    ToggleHelp,
    ToggleLogs,
    ChangeViewFocus,
    ViewerUp,
    ViewerDown,
    FileViewerDirUp,
    FileViewerEnterDir,
    Select,
    TogglePlayback,
    StopPlayback,
    ChangeVisualization,
    ChangeColor,
    OnSongFinished,
    DeleteFromQueue,
    PlayNextFromQueue,
    PlayPreviousFromQueue,
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
            KeyCode::Down => Some(Action::ViewerDown),
            KeyCode::Up => Some(Action::ViewerUp),
            KeyCode::Right => Some(Action::FileViewerEnterDir),
            KeyCode::Char('f') => Some(Action::ChangeViewFocus),
            KeyCode::Enter => Some(Action::Select),
            KeyCode::Char('d') => Some(Action::DeleteFromQueue),
            KeyCode::Char('p') => Some(Action::TogglePlayback),
            KeyCode::Char('s') => Some(Action::StopPlayback),
            KeyCode::Char('v') => Some(Action::ChangeVisualization),
            KeyCode::Char('c') => Some(Action::ChangeColor),
            KeyCode::Char('j') => Some(Action::PlayPreviousFromQueue),
            KeyCode::Char('k') => Some(Action::PlayNextFromQueue),
            _ => None,
        }
    }

    pub fn from_event(event: PlaybackEvent) -> Action {
        match event {
            PlaybackEvent::SongFinished => Action::OnSongFinished,
        }
    }
}
