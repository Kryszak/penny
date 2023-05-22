use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
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
    pub modifiers: KeyModifiers,
}

impl KeyPress {
    pub fn new(key_event: KeyEvent) -> Self {
        KeyPress {
            key: key_event.code,
            modifiers: key_event.modifiers,
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
pub struct Events {
    tx: Sender<AppEvent>,
    rx: Receiver<AppEvent>,
    stop_capture: Arc<AtomicBool>,
}

impl Events {
    /// Creates event instance starting key press capture loop in separate thread
    pub fn new(tick_rate: Duration) -> Events {
        let (tx, rx) = mpsc::channel();
        let stop_capture = Arc::new(AtomicBool::new(false));

        let event_stop_capture = stop_capture.clone();
        let loop_tx = tx.clone();
        thread::spawn(move || {
            loop {
                // poll for tick rate duration, if no event, sent tick event.
                if crossterm::event::poll(tick_rate).unwrap() {
                    if let event::Event::Key(key_event) = event::read().unwrap() {
                        loop_tx
                            .send(AppEvent::Input(KeyPress::new(key_event)))
                            .unwrap();
                    }
                }
                loop_tx.send(AppEvent::Tick).unwrap();
                if event_stop_capture.load(Ordering::Relaxed) {
                    break;
                }
            }
        });

        Events {
            tx,
            rx,
            stop_capture,
        }
    }

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
}
