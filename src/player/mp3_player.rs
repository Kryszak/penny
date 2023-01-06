use super::{metadata::Mp3Metadata, MetadataReader};
use crate::{application::actions::Action, files::FileEntry, player::FrameDecoder};
use log::{debug, error};
use minimp3::{Decoder, Error};
use rodio::{OutputStream, Sink};
use std::fs::File;

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
}

impl Mp3Player {
    pub fn new() -> Self {
        Mp3Player {
            song: None,
            state: PlayerState::New,
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
        // TODO add controls for play/pause
        tokio::spawn(async move {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            loop {
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
            }
            sink.sleep_until_end();
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
            debug!("Paused playback");
        } else if self.state == PlayerState::Paused {
            self.state = PlayerState::Playing;
            debug!("Resumed playback");
        }
    }
}
