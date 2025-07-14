use crate::{
    application::actions::Action,
    external::notifier::{notify_playback_start, notify_playback_stopped},
    input::{events::PlaybackEvent::SongFinished, EventBus},
    player::{frame_decoder::FrameDuration, spectrum_analyzer::SpectrumAnalyzer, FrameDecoder},
    queue::SongFile,
};
use log::{debug, error};
use minimp3_fixed as minimp3;
use minimp3::{Decoder, Error};
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
    /// Playback finished
    Stopped,
}

/// Structure responsible for playing mp3 files.
/// Also allows to retrieve information about playback progress
/// and selected song information.
pub struct Mp3Player {
    /// Miliseconds elapsed since start of playback
    current_playback_ms_elapsed: Arc<Mutex<f64>>,
    song: Option<SongFile>,
    state: Arc<Mutex<PlayerState>>,
    /// Flag indicating that player should pause playback
    paused: Arc<AtomicBool>,
    /// Flag indicating that player should stop playback
    stop: Arc<AtomicBool>,
    /// current frame spectrum analyzed data
    spectrum: Arc<Mutex<Vec<f32>>>,
    /// struct allowing for sending application events
    events: Arc<Mutex<EventBus>>,
    notify_song_end: Arc<AtomicBool>,
}

impl Mp3Player {
    pub fn new(events: Arc<Mutex<EventBus>>) -> Self {
        Mp3Player {
            song: None,
            state: Arc::new(Mutex::new(PlayerState::New)),
            paused: Arc::new(AtomicBool::new(false)),
            stop: Arc::new(AtomicBool::new(false)),
            current_playback_ms_elapsed: Arc::new(Mutex::new(0.0)),
            spectrum: Arc::new(Mutex::new(vec![])),
            events,
            notify_song_end: Arc::new(AtomicBool::new(true)),
        }
    }

    /// Sets provided file as current song in player and starts playback.
    pub fn set_song_file(&mut self, song_file: SongFile) {
        //! In case player is currently playing other file, stops it
        self.stop_playback(false);
        self.song = Some(song_file);
        *self.state.lock().unwrap() = PlayerState::SongSelected;
    }

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::TogglePlayback => self.toggle_playback(),
            Action::StopPlayback => self.stop_playback(false),
            _ => error!("Action {action:?} is not supported for Mp3Player!"),
        }
    }

    /// Returns true if player is currently playing song or in paused state
    pub fn is_playing(&self) -> bool {
        match *self.state.lock().unwrap() {
            PlayerState::Playing | PlayerState::Paused => true,
            PlayerState::New | PlayerState::SongSelected | PlayerState::Stopped => false,
        }
    }

    /// Returns labels for current state of player
    pub fn get_playback_status_string(&self) -> String {
        match *self.state.lock().unwrap() {
            PlayerState::New => String::from(" \u{231B} "),
            PlayerState::SongSelected => String::from(" \u{23F9} Stop "),
            PlayerState::Playing => String::from(" \u{23F5} Playing "),
            PlayerState::Paused => String::from(" \u{23F8} Paused "),
            PlayerState::Stopped => String::from(" \u{23F9} Stop "),
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
        let playback_progress = self.current_playback_ms_elapsed.clone();
        let spectrum_data = self.spectrum.clone();
        let event_sender = self.events.clone();
        let should_notify = self.notify_song_end.clone();
        let mut decoder = self.get_file_decoder();
        notify_playback_start(self.song.as_ref().unwrap());
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
                let frame_duration;
                match decoder.next_frame() {
                    Ok(frame) => {
                        {
                            *spectrum_data.lock().unwrap() = spectrum_analyzer.analyze(&frame.data);
                        }
                        frame_duration = frame.get_duration() - Duration::from_millis(2);
                        let source = FrameDecoder::new(frame);
                        sink.append(source);
                    }
                    Err(Error::Eof) => break,
                    Err(e) => {
                        error!("{e:?}");
                        break;
                    }
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
            if should_notify.load(Ordering::Relaxed) {
                event_sender.lock().unwrap().send(SongFinished);
            }
            should_notify.store(true, Ordering::Relaxed);
            let mut state = player_state.lock().unwrap();
            *state = PlayerState::Stopped;
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
            PlayerState::Stopped => {
                debug!("Now playing {:?}", self.song.as_ref().unwrap().display());
                self.play();
                *state = PlayerState::Playing;
            }
        }
    }

    pub fn stop_playback(&mut self, with_notification: bool) {
        if !self.is_playing() {
            return;
        }
        self.notify_song_end
            .store(with_notification, Ordering::Relaxed);
        self.stop.store(true, Ordering::Relaxed);
        self.wait_for_stopped_state();
        if with_notification {
            notify_playback_stopped();
        }
    }

    fn get_file_decoder(&self) -> Decoder<File> {
        let song_path = self
            .song
            .as_ref()
            .map(|s| s.file_entry.path.clone())
            .unwrap();
        Decoder::new(File::open(song_path).unwrap())
    }

    fn get_song_elapsed_seconds(&self) -> f64 {
        *self.current_playback_ms_elapsed.lock().unwrap() / 1000.0
    }

    fn wait_for_stopped_state(&self) {
        loop {
            let state = self.state.lock().unwrap();
            if *state == PlayerState::Stopped {
                break;
            }
        }
    }
}
