use std::time::Duration;

use futures::{StreamExt, future::FutureExt, io, select};
use futures_timer::Delay;

use crossterm::event::EventStream;
use tokio::{sync::mpsc::Sender, task::JoinHandle};

use crate::{Result, events::app_event::AppEvent};

// pub fn run_input_read(app_tx: Sender<AppEvent>) -> Result<JoinHandle<()>> {
//     let mut reader = EventStream::new();
//
//     let handle = tokio::spawn(async move {
//         loop {
//             let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
//             let mut event = reader.next().fuse();
//
//             select! {
//                 _ = delay => { }
//                 maybe_event = event => {
//                     match maybe_event {
//                         Some(Ok(event)) => {
//                             app_tx.send(event.into()).await;
//                         }
//                         Some(Err(e)) => println!("Error: {e:?}\r"),
//                         None => break,
//                     }
//                 }
//             };
//         }
//     });
//
//     Ok(handle)
// }

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
