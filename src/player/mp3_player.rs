use crate::{
    application::actions::Action, files::FileEntry, player::FrameDecoder, player::SelectedSongFile,
};
use log::{debug, error, trace};
use minimp3::{Decoder, Error, Frame};
use rodio::{OutputStream, Sink};
use std::{
    fs::File,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

#[derive(PartialEq)]
enum PlayerState {
    New,
    SongSelected,
    Playing,
    Paused,
}

pub struct Mp3Player {
    pub song: Option<SelectedSongFile>,
    state: Arc<Mutex<PlayerState>>,
    paused: Arc<AtomicBool>,
    stop: Arc<AtomicBool>,
    frames: Vec<Frame>,
}

impl Mp3Player {
    pub fn new() -> Self {
        Mp3Player {
            song: None,
            state: Arc::new(Mutex::new(PlayerState::New)),
            paused: Arc::new(AtomicBool::new(false)),
            stop: Arc::new(AtomicBool::new(false)),
            frames: vec![],
        }
    }

    pub fn set_song_file(&mut self, file_entry: &FileEntry) {
        {
            let mut state = self.state.lock().unwrap();
            *state = PlayerState::SongSelected;
            match *state {
                PlayerState::Playing | PlayerState::Paused => {
                    self.stop.store(true, Ordering::Relaxed);
                }
                _ => {}
            }
            while self.stop.load(Ordering::Relaxed) {}
        }
        self.frames = Mp3Player::read_mp3_frames(&file_entry.path);
        self.song = Some(SelectedSongFile::new(file_entry, self.get_track_duration()));
        {
            let mut state = self.state.lock().unwrap();
            *state = PlayerState::SongSelected;
        }
        self.toggle_playback();
    }

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::TogglePlayback => self.toggle_playback(),
            Action::StopPlayback => self.stop_playback(),
            _ => error!("Action {:?} is not supported for Mp3Player!", action),
        }
    }

    fn play(&mut self) {
        let paused = self.paused.clone();
        let should_stop = self.stop.clone();
        let player_state = self.state.clone();
        let frame_duration = self.get_frame_duration();
        let mut frames_iterator = self.frames.clone().into_iter();
        thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            loop {
                if should_stop.load(Ordering::Relaxed) {
                    break;
                }
                while paused.load(Ordering::Relaxed) {
                    if should_stop.load(Ordering::Relaxed) {
                        paused.store(false, Ordering::Relaxed);
                        break;
                    }
                }
                match frames_iterator.next() {
                    Some(frame) => {
                        let source = FrameDecoder::new(frame);
                        sink.append(source);
                    }
                    None => break,
                }
                std::thread::sleep(frame_duration);
            }
            should_stop.store(false, Ordering::Relaxed);
            paused.store(false, Ordering::Relaxed);
            debug!("Song playback finished.");
            let mut state = player_state.lock().unwrap();
            *state = PlayerState::SongSelected;
        });
    }

    fn stop_playback(&mut self) {
        {
            let mut state = self.state.lock().unwrap();
            *state = PlayerState::SongSelected;
        }
        self.stop.store(true, Ordering::Relaxed);
    }

    fn get_track_duration(&self) -> Duration {
        let first_frame = &self.frames[0];
        let sample_rate = first_frame.sample_rate;
        let samples_per_frame = first_frame.data.len() / first_frame.channels;
        let duration = (samples_per_frame as i32 * self.frames.len() as i32) / sample_rate;

        Duration::from_secs(duration.try_into().unwrap())
    }

    fn get_frame_duration(&self) -> Duration {
        let first_frame = &self.frames[0];
        let frame_duration = (first_frame.data.len() as f64 / first_frame.channels as f64)
            / first_frame.sample_rate as f64;
        Duration::from_millis((frame_duration * 1024.0) as u64)
    }

    fn toggle_playback(&mut self) {
        let state_mutex = self.state.clone();
        let mut state = state_mutex.lock().unwrap();
        match *state {
            PlayerState::New => trace!("Nothing in player yet, skipping."),
            PlayerState::SongSelected => {
                debug!(
                    "Starting playback of {:?}",
                    self.song.as_ref().unwrap().display()
                );
                self.play();
                *state = PlayerState::Playing;
            }
            PlayerState::Playing => {
                *state = PlayerState::Paused;
                self.paused.store(true, Ordering::Relaxed);
                debug!("Paused playback");
            }
            PlayerState::Paused => {
                *state = PlayerState::Playing;
                self.paused.store(false, Ordering::Relaxed);
                debug!("Resumed playback");
            }
        }
    }

    fn read_mp3_frames(file_path: &str) -> Vec<Frame> {
        let mut decoder = Decoder::new(File::open(file_path).unwrap());
        let mut frames: Vec<Frame> = vec![];
        loop {
            match decoder.next_frame() {
                Ok(frame) => {
                    frames.push(frame);
                }
                Err(Error::Eof) => break,
                Err(e) => {
                    error!("{:?}", e);
                    break;
                }
            }
        }

        frames
    }
}
