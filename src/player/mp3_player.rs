use crate::{
    application::actions::Action,
    files::FileEntry,
    player::SelectedSongFile,
    player::{spectrum_analyzer::SpectrumAnalyzer, FrameDecoder},
};
use log::{debug, error};
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

/// States that player can be in
#[derive(PartialEq)]
enum PlayerState {
    /// Created
    New,
    /// Loaded mp3 file, not playing, ready to start playback
    SongSelected,
    /// Playing selected file
    Playing,
    /// Playback paused
    Paused,
}

/// Structure responsible for playing mp3 files.
/// Also allows to retrieve information about playback progress
/// and selected song information.
pub struct Mp3Player {
    /// Miliseconds elapsed since start of playback
    current_playback_ms_elapsed: Arc<Mutex<f64>>,
    song: Option<SelectedSongFile>,
    state: Arc<Mutex<PlayerState>>,
    /// Flag indicating that player should pause playback
    paused: Arc<AtomicBool>,
    /// Flag indicating that player should stop playback
    stop: Arc<AtomicBool>,
    /// all mp3 frames read from selected file
    frames: Vec<Frame>,
    /// current frame spectrum analyzed data
    spectrum: Arc<Mutex<Vec<f32>>>,
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
            spectrum: Arc::new(Mutex::new(vec![])),
        }
    }

    /// Sets provided file as current song in player and starts playback.
    pub fn set_song_file(&mut self, file_entry: &FileEntry) {
        //! In case player is currently playing other file, stops it
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

    /// Returns labels for current state of player
    pub fn get_playback_status_string(&self) -> String {
        match *self.state.lock().unwrap() {
            PlayerState::New => String::from(" \u{231B} "),
            PlayerState::SongSelected => String::from(" \u{23F9} Stop "),
            PlayerState::Playing => String::from(" \u{23F5} Playing "),
            PlayerState::Paused => String::from(" \u{23F8} Paused "),
        }
    }

    /// Returns vector of information retrieved from [SelectedSongFile](SelectedSongFile::display)
    /// or default information in case nothing is selected
    pub fn display_information(&mut self) -> Vec<String> {
        match &self.song {
            Some(song_info) => song_info.display(),
            None => vec![String::from("Artist: --"), String::from("Title : --")],
        }
    }

    /// Returns normalized fraction of finished playback [0..1]
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

    /// Returns text label for playback progress
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

    pub fn get_audio_spectrum(&self) -> Vec<f32> {
        (*self.spectrum.clone().lock().unwrap()).clone()
    }

    fn play(&mut self) {
        let paused = self.paused.clone();
        let should_stop = self.stop.clone();
        let player_state = self.state.clone();
        let frame_duration = self.get_frame_duration() - Duration::from_millis(1);
        let mut frames_iterator = self.frames.clone().into_iter();
        let playback_progress = self.current_playback_ms_elapsed.clone();
        let spectrum_data = self.spectrum.clone();
        thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            let mut spectrum_analyzer = SpectrumAnalyzer::new();
            loop {
                if should_stop.load(Ordering::Relaxed) {
                    break;
                }
                if paused.load(Ordering::Relaxed) {
                    *spectrum_data.lock().unwrap() = vec![];
                }
                while paused.load(Ordering::Relaxed) {
                    if should_stop.load(Ordering::Relaxed) {
                        paused.store(false, Ordering::Relaxed);
                        break;
                    }
                }
                match frames_iterator.next() {
                    Some(frame) => {
                        {
                            *spectrum_data.lock().unwrap() = spectrum_analyzer.analyze(&frame.data);
                        }
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
            *spectrum_data.lock().unwrap() = vec![];
            debug!("Playback finished.");
            let mut state = player_state.lock().unwrap();
            *state = PlayerState::SongSelected;
        });
    }

    fn toggle_playback(&mut self) {
        let state_mutex = self.state.clone();
        let mut state = state_mutex.lock().unwrap();
        match *state {
            PlayerState::New => debug!("Nothing in player yet, skipping."),
            PlayerState::SongSelected => {
                debug!("Now playing {:?}", self.song.as_ref().unwrap().display());
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

    fn stop_playback(&mut self) {
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

    fn get_song_elapsed_seconds(&self) -> f64 {
        *self.current_playback_ms_elapsed.lock().unwrap() / 1000.0
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
