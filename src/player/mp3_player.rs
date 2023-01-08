use super::{metadata::Mp3Metadata, MetadataReader};
use crate::{application::actions::Action, files::FileEntry, player::FrameDecoder};
use log::{debug, error};
use minimp3::{Decoder, Error};
use rodio::{OutputStream, Sink};
use std::{
    fs::File,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

pub struct SelectedSongFile {
    pub metadata: Mp3Metadata,
}

impl SelectedSongFile {
    fn new(file_entry: &FileEntry) -> Self {
        SelectedSongFile {
            metadata: MetadataReader::read_metadata(file_entry).unwrap(),
        }
    }
}

#[derive(PartialEq)]
enum PlayerState {
    New,
    SongSelected,
    Playing,
    Paused,
}

pub struct Mp3Player {
    pub song: Option<SelectedSongFile>,
    state: PlayerState,
    paused: Arc<AtomicBool>,
}

impl Mp3Player {
    pub fn new() -> Self {
        Mp3Player {
            song: None,
            state: PlayerState::New,
            paused: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn set_song_file(&mut self, file_entry: &FileEntry) {
        self.song = Some(SelectedSongFile::new(file_entry));
        self.state = PlayerState::SongSelected;
        self.toggle_playback();
    }

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::TogglePlayback => self.toggle_playback(),
            _ => error!("Action {:?} is not supported for Mp3Player!", action),
        }
    }

    fn play(&mut self) {
        let mut decoder =
            Decoder::new(File::open(&self.song.as_ref().unwrap().metadata.file_path).unwrap());
        let paused = self.paused.clone();
        tokio::spawn(async move {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            loop {
                while paused.load(std::sync::atomic::Ordering::Relaxed) {}
                match decoder.next_frame() {
                    Ok(frame) => {
                        let source = FrameDecoder::new(frame);
                        sink.append(source);
                    }
                    Err(Error::Eof) => break,
                    Err(e) => {
                        error!("{:?}", e);
                        break;
                    }
                }
                // to keep playback thread alive until the end
                std::thread::sleep(Duration::from_millis(20));
            }
            // TODO change player status to 'SongSelected' here
        });
    }

    fn toggle_playback(&mut self) {
        if self.state == PlayerState::SongSelected {
            debug!(
                "Starting playback of {:?}",
                self.song.as_ref().unwrap().metadata.display()
            );
            self.play();
            self.state = PlayerState::Playing;
        } else if self.state == PlayerState::Playing {
            self.state = PlayerState::Paused;
            self.paused.store(true, Ordering::Relaxed);
            debug!("Paused playback");
        } else if self.state == PlayerState::Paused {
            self.state = PlayerState::Playing;
            self.paused.store(false, Ordering::Relaxed);
            debug!("Resumed playback");
        }
    }
}
