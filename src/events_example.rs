use std::{sync::mpsc, thread};

use crossterm::event::KeyEvent;

pub enum Event {
    Input(KeyEvent),
}

pub struct EventHandler {
    tx: mpsc::Sender<Event>,
    rx: mpsc::Receiver<Event>,
}

// TODO: Remove/Keep mpsc events? maybe needed when we implement JiraApi?
impl EventHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        Self { tx, rx }
    }

    // starts background thread and returns the receiver
    pub fn start(self) -> mpsc::Receiver<Event> {
        let tx_input = self.tx.clone();
        thread::spawn(move || {
            Self::handle_input_events(tx_input);
        });

        self.rx
    }

    fn handle_input_events(tx: mpsc::Sender<Event>) {
        loop {
            // TODO: Handle unwrap() with custom errors!
            match crossterm::event::read().unwrap() {
                crossterm::event::Event::Key(key_event) => {
                    // TODO: Handle unwrap() with custom errors!
                    tx.send(Event::Input(key_event)).unwrap()
                }
                _ => {}
            }
        }
    }
}
