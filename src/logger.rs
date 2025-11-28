use once_cell::sync::OnceCell;
use std::sync::mpsc::{Receiver, Sender, channel};

static LOG_CHANNEL: OnceCell<Sender<String>> = OnceCell::new();

pub fn init_logger() -> Receiver<String> {
    let (tx, rx) = channel();
    LOG_CHANNEL.set(tx).expect("Logger déjà initialisé");
    rx
}

pub fn log(message: String) {
    if let Some(tx) = LOG_CHANNEL.get() {
        let _ = tx.send(message); // Ignore si le receiver est fermé
    }
}
