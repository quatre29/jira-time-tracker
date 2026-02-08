use std::io;

use app::App;
use events::EventHandler;

mod app;
mod error;
mod events;
mod jira;
mod storage;
mod time;
mod ui;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App::new(vec![]);

    let event_handler = EventHandler::new();
    let event_rx = event_handler.start();

    let app_result = app.run(&mut terminal, event_rx);

    ratatui::restore();

    app_result
}
