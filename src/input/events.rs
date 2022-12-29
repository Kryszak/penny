use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use std::{
    sync::mpsc::{channel, Receiver, RecvError, Sender},
    thread,
    time::Duration,
};

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

pub enum InputEvent {
    /// An input event occurred.
    Input(KeyPress),
    /// An tick event occurred.
    Tick,
}

pub struct Events {
    rx: Receiver<InputEvent>,
    // Need to be kept around to prevent disposing the sender side.
    _tx: Sender<InputEvent>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Events {
        let (tx, rx) = channel();

        let event_tx = tx.clone(); // the thread::spawn own event_tx
        thread::spawn(move || {
            loop {
                // poll for tick rate duration, if no event, sent tick event.
                if crossterm::event::poll(tick_rate).unwrap() {
                    if let event::Event::Key(key_event) = event::read().unwrap() {
                        event_tx
                            .send(InputEvent::Input(KeyPress::new(key_event)))
                            .unwrap();
                    }
                }
                event_tx.send(InputEvent::Tick).unwrap();
            }
        });

        Events { rx, _tx: tx }
    }

    /// Attempts to read an event.
    /// This function block the current thread.
    pub fn next(&self) -> Result<InputEvent, RecvError> {
        self.rx.recv()
    }
}
