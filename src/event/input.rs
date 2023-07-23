use std::{time::Duration, thread};

use crossterm::event::{Event, KeyEvent, self, poll};
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub enum InputEvent {
    Input(KeyEvent),
    Tick,
}

pub fn start(event_sender: Sender<InputEvent>) {
    thread::spawn(move || {
        loop {
            if poll(Duration::from_millis(10)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    event_sender.blocking_send(InputEvent::Input(key)).unwrap();
                }
            }
            event_sender.blocking_send(InputEvent::Tick).unwrap();
        }
    });
}

