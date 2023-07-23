
use std::sync::Arc;

use tokio::sync::mpsc::{Sender, Receiver, channel};

pub struct LogSender {
    event_sender: Sender<crate::event::input::InputEvent>,
}

impl LogSender {
    pub async fn info(&self, message: String) {
        self.event_sender.send(crate::event::input::InputEvent::InfoMessage(message)).await;
    }
     pub async fn error(&self, message: String) {
        self.event_sender.send(crate::event::input::InputEvent::ErrorMessage(message)).await;
    }

    pub(crate) fn new(event_sender: Sender<crate::event::input::InputEvent>) -> LogSender {
        LogSender{event_sender}
    }
}

impl Clone for LogSender {
    fn clone(&self) -> Self {
        Self {
            event_sender: self.event_sender.clone(),
        }
    }
}

