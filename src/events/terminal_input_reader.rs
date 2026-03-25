use futures::{StreamExt, io};
use crossterm::event::EventStream;
use tokio::{sync::mpsc::Sender, task::JoinHandle};

use crate::{events::app_event::AppEvent};

pub fn run_input_read(app_tx: Sender<AppEvent>) -> io::Result<JoinHandle<()>> {
    let mut reader = EventStream::new();

    let handle = tokio::spawn(async move {
        while let Some(Ok(event)) = reader.next().await {
            if app_tx.send(AppEvent::KeyEvent(event)).await.is_err() {
                break;
            }
        }
    });

    Ok(handle)
}
