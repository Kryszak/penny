use notify_rust::Notification;

use crate::player::SelectedSongFile;

pub fn notify_playback_start(song_metadata: &SelectedSongFile) {
    Notification::new()
        .summary("Penny")
        .body(&song_metadata.display_short())
        .show()
        .unwrap();
}

pub fn notify_playback_stopped() {
    Notification::new()
        .summary("Penny")
        .body("Stopped")
        .show()
        .unwrap();
}
