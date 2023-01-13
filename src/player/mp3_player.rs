use crate::{
    application::actions::Action, files::FileEntry, player::FrameDecoder, player::SelectedSongFile,
};
use log::{debug, error, trace};
use minimp3::{Decoder, Error, Frame};
use rodio::{OutputStream, Sink};
use std::{
    f64,
    fs::File,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use super::duration_formatter::{DurationFormat, DurationFormatter};

#[derive(PartialEq)]
enum PlayerState {
    New,
    SongSelected,
    Playing,
    Paused,
}

pub struct Mp3Player {
    pub current_playback_ms_elapsed: Arc<Mutex<f64>>,
    song: Option<SelectedSongFile>,
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
            current_playback_ms_elapsed: Arc::new(Mutex::new(0.0)),
        }
    }

    pub fn set_song_file(&mut self, file_entry: &FileEntry) {
        {
            match *self.state.lock().unwrap() {
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

    pub fn get_playback_status_string(&self) -> String {
        match *self.state.lock().unwrap() {
            PlayerState::New => String::from(" \u{231B} "),
            PlayerState::SongSelected => String::from(" \u{23F9} Stop "),
            PlayerState::Playing => String::from(" \u{23F5} Playing "),
            PlayerState::Paused => String::from(" \u{23F8} Paused "),
        }
    }

    pub fn display_information(&mut self) -> Vec<String> {
        match &self.song {
            Some(song_info) => song_info.display(),
            None => vec![String::from("Artist: --"), String::from("Title : --")],
        }
    }

    pub fn get_current_song_percentage_progress(&self) -> f64 {
        match &self.song {
            Some(s) => {
                let current_progress_mutex = self.get_song_elapsed_seconds();
                let song_length = s.duration.as_secs();
                (current_progress_mutex / (song_length as f64)).min(1.0)
            }
            None => 0.0,
        }
    }

    pub fn get_text_progress(&self) -> Option<String> {
        self.song.as_ref().map(|s| {
            format!(
                "{} / {}",
                Duration::from_secs(self.get_song_elapsed_seconds() as u64)
                    .format(DurationFormat::MmSs),
                s.duration.format(DurationFormat::MmSs)
            )
        })
    }

    fn play(&mut self) {
        let paused = self.paused.clone();
        let should_stop = self.stop.clone();
        let player_state = self.state.clone();
        let frame_duration = self.get_frame_duration();
        let mut frames_iterator = self.frames.clone().into_iter();
        let playback_progress = self.current_playback_ms_elapsed.clone();
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
                {
                    *playback_progress.lock().unwrap() += frame_duration.as_millis() as f64;
                }
            }
            should_stop.store(false, Ordering::Relaxed);
            paused.store(false, Ordering::Relaxed);
            *playback_progress.lock().unwrap() = 0.0;
            debug!("Playback finished.");
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
                debug!("Now playing {:?}", self.song.as_ref().unwrap().display());
                self.play();
                *state = PlayerState::Playing;
            }
            PlayerState::Playing => {
                *state = PlayerState::Paused;
                self.paused.store(true, Ordering::Relaxed);
                trace!("Paused playback");
            }
            PlayerState::Paused => {
                *state = PlayerState::Playing;
                self.paused.store(false, Ordering::Relaxed);
                trace!("Resumed playback");
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

    fn get_song_elapsed_seconds(&self) -> f64 {
        *self.current_playback_ms_elapsed.lock().unwrap() / 1000.0
    }
}
