use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver},
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
pub enum InputEvent {
    /// Key pressed by user
    Input(KeyPress),
    /// Defined span of time elapsed in app
    Tick,
}

/// Event handling in application
/// Captures key presses and allows to poll for them
/// In case of no key press, sends [Tick](InputEvent::Tick) event
pub struct Events {
    rx: Receiver<InputEvent>,
    stop_capture: Arc<AtomicBool>,
}

impl Events {
    /// Creates event instance starting key press capture loop in separate thread
    pub fn new(tick_rate: Duration) -> Events {
        let (tx, rx) = mpsc::channel();
        let stop_capture = Arc::new(AtomicBool::new(false));

        let event_stop_capture = stop_capture.clone();
        thread::spawn(move || {
            loop {
                // poll for tick rate duration, if no event, sent tick event.
                if crossterm::event::poll(tick_rate).unwrap() {
                    if let event::Event::Key(key_event) = event::read().unwrap() {
                        tx.send(InputEvent::Input(KeyPress::new(key_event)))
                            .unwrap();
                    }
                }
                tx.send(InputEvent::Tick).unwrap();
                if event_stop_capture.load(Ordering::Relaxed) {
                    break;
                }
            }
        });

        Events { rx, stop_capture }
    }

    /// Fetches next key press event or returns [Tick](InputEvent::Tick)
    pub fn next(&mut self) -> InputEvent {
        self.rx.recv().unwrap_or(InputEvent::Tick)
    }

    /// Stops keypress capture thread
    pub fn close(&mut self) {
        self.stop_capture.store(true, Ordering::Relaxed)
    }
}
