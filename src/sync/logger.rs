
use std::sync::Arc;

use tokio::sync::mpsc::{Sender, Receiver, channel};

pub struct Logger {
    pub info_receiver: Receiver<String>,
    pub error_receiver: Receiver<String>,
    info_sender: Arc<Sender<String>>,
    error_sender: Arc<Sender<String>>,
}

impl Logger {
    pub fn new () -> Self {
        let (info_send, info_rec) = channel(32);
        let (error_send, error_rec) = channel(32);
        Logger{
            info_receiver: info_rec,
            error_receiver: error_rec,
            info_sender: Arc::new(info_send),
            error_sender: Arc::new(error_send),
        }
    }

    pub fn sender(&self) -> LogSender {
        LogSender {
            info_sender: self.info_sender.clone(),
            error_sender: self.error_sender.clone()
        }
    }
}

pub struct LogSender {
    info_sender: Arc<Sender<String>>,
    error_sender: Arc<Sender<String>>,
}

impl LogSender {
    pub async fn info(&self, message: String) {
        self.info_sender.send(message).await;
    }
     pub async fn error(&self, message: String) {
        self.error_sender.send(message).await;
    }
}

impl Clone for LogSender {
    fn clone(&self) -> Self {
        Self {
            info_sender: self.info_sender.clone(),
            error_sender: self.error_sender.clone(),
        }
    }
}

