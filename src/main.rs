use app::App;

use crate::events::app_event::AppEvent;

mod api;
mod app;
mod error;
mod events;
mod events_example;
mod storage;
mod time;
mod ui;

pub use error::Result;
pub use events::terminal_input_reader;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let mut terminal = ratatui::init();

    let (app_tx, app_rx) = tokio::sync::mpsc::channel::<AppEvent>(100);

    let mut app = App::new(app_tx.clone());
    let _read_handle = terminal_input_reader::run_input_read(app_tx.clone())?;
    let app_result = app.run(&mut terminal, app_rx).await;

    ratatui::restore();

    app_result?;

    Ok(())
}
