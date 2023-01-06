use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use log::error;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::sync::mpsc::Receiver;

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
    Input(KeyPress),
    Tick,
}

pub struct Events {
    rx: Receiver<InputEvent>,
    stop_capture: Arc<AtomicBool>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Events {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let stop_capture = Arc::new(AtomicBool::new(false));

        let event_stop_capture = stop_capture.clone();
        tokio::spawn(async move {
            loop {
                // poll for tick rate duration, if no event, sent tick event.
                if crossterm::event::poll(tick_rate).unwrap() {
                    if let event::Event::Key(key_event) = event::read().unwrap() {
                        if let Err(err) = tx.send(InputEvent::Input(KeyPress::new(key_event))).await
                        {
                            error!("Failed to send KeyPress event, {}", err);
                        }
                    }
                }
                if let Err(err) = tx.send(InputEvent::Tick).await {
                    error!("Failed to send Tick event, {}", err);
                }
                if event_stop_capture.load(Ordering::Relaxed) {
                    break;
                }
            }
        });

        Events { rx, stop_capture }
    }

    pub async fn next(&mut self) -> InputEvent {
        self.rx.recv().await.unwrap_or(InputEvent::Tick)
    }

    pub fn close(&mut self) {
        self.stop_capture.store(true, Ordering::Relaxed)
    }
}
