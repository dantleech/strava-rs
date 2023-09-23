use log::{log, Level};
use tokio::sync::mpsc::Sender;

use super::input::{EventSender, InputEvent};

pub struct Logger {
    event_sender: EventSender
}

impl Logger {
    pub async fn info(&self, message: String) {
        log!(Level::Info, "{}", message);
        self.event_sender.send(InputEvent::InfoMessage(message)).await.unwrap();
    }
    pub async fn error(&self, message: String) {
        log!(Level::Error, "{}", message);
        self.event_sender.send(InputEvent::ErrorMessage(message)).await.unwrap();
    }

    pub(crate) fn new(event_sender: Sender<InputEvent>) -> Self {
        Self{event_sender}
    }
}

impl Clone for Logger {
    fn clone(&self) -> Self {
        Logger{
            event_sender: self.event_sender.clone()
        }
    }
}
