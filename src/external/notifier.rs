use log::debug;
use notify_rust::Notification;

use crate::player::SelectedSongFile;

pub fn notify_playback_start(song_metadata: &SelectedSongFile) {
    if Notification::new()
        .summary("Penny")
        .body(&song_metadata.display_short())
        .show()
        .is_err()
    {
        debug!("Failed to send playback start notification.");
    }
}

pub fn notify_playback_stopped() {
    if Notification::new()
        .summary("Penny")
        .body("Stopped")
        .show()
        .is_err()
    {
        debug!("Failed to send playback stop notification.");
    }
}
