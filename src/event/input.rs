use std::{time::Duration, thread};

use crossterm::event::{Event, KeyEvent, self, poll};
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub enum InputEvent {
    Input(KeyEvent),
    Tick,
    InfoMessage(String),
    ErrorMessage(String),
    Reload,
    Sync,
}

pub type EventSender = Sender<InputEvent>;

pub fn start(event_sender: EventSender) {
    thread::spawn(move || {
        loop {
            if poll(Duration::from_millis(20)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    event_sender.blocking_send(InputEvent::Input(key)).unwrap();
                }
            }

            // ignore errors from tick - it causes panics on shutdown
            let _ = event_sender.blocking_send(InputEvent::Tick);
        }
    });
}

