use crossterm::event::{self, KeyCode, KeyEvent};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread,
    time::Duration,
};

// Combination of keys pressed in app
pub struct KeyPress {
    pub key: KeyCode,
}

impl KeyPress {
    pub fn new(key_event: KeyEvent) -> Self {
        KeyPress {
            key: key_event.code,
        }
    }
}

/// Type of event in app
pub enum AppEvent {
    /// Key pressed by user
    Input(KeyPress),
    /// Defined span of time elapsed in app
    Tick,
    /// Event occurred during playback
    Playback(PlaybackEvent),
}

pub enum PlaybackEvent {
    SongFinished,
}

/// Event handling in application
/// Captures key presses and allows to poll for them
/// In case of no key press, sends [Tick](InputEvent::Tick) event
pub struct EventBus {
    tx: Sender<AppEvent>,
    rx: Receiver<AppEvent>,
    stop_capture: Arc<AtomicBool>,
}

impl EventBus {
    /// Creates event instance starting key press capture loop in separate thread
    pub fn new(tick_rate: Duration) -> EventBus {
        let (tx, rx) = mpsc::channel();
        let stop_capture = Arc::new(AtomicBool::new(false));

        let event_stop_capture = stop_capture.clone();
        let loop_tx = tx.clone();

        thread::spawn(move || Self::poll_key_events(tick_rate, loop_tx, event_stop_capture));

        EventBus {
            tx,
            rx,
            stop_capture,
        }
    }

    /// Alows to send event in application
    pub fn send(&mut self, event: PlaybackEvent) {
        self.tx.send(AppEvent::Playback(event)).unwrap();
    }

    /// Fetches next key press event or returns [Tick](InputEvent::Tick)
    pub fn next(&mut self) -> AppEvent {
        self.rx.recv().unwrap_or(AppEvent::Tick)
    }

    /// Stops keypress capture thread
    pub fn close(&mut self) {
        self.stop_capture.store(true, Ordering::Relaxed)
    }

    fn poll_key_events(tick_rate: Duration, tx: Sender<AppEvent>, stop_capture: Arc<AtomicBool>) {
        loop {
            // poll for tick rate duration, if no event, sent tick event.
            if crossterm::event::poll(tick_rate).unwrap() {
                if let event::Event::Key(key_event) = event::read().unwrap() {
                    tx.send(AppEvent::Input(KeyPress::new(key_event))).unwrap();
                }
            }
            tx.send(AppEvent::Tick).unwrap();
            if stop_capture.load(Ordering::Relaxed) {
                break;
            }
        }
    }
}
