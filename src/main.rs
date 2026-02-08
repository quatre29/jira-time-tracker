use std::io;

use app::App;
use events::EventHandler;
use jira::models::JiraTicket;

mod app;
mod error;
mod events;
mod jira;
mod storage;
mod time;
mod ui;

// NOTE: Placeholder
fn get_placeholder_tickets() -> Vec<JiraTicket> {
    vec![
        JiraTicket::new("PPD-2311", "Widget Creation"),
        JiraTicket::new("PPD-2333", "LSM Menu"),
        JiraTicket::new("PPD-2423", "Configuration"),
        JiraTicket::new("PPD-2355", "Basket Menu"),
        JiraTicket::new("PPD-2778", "PUN Numbers"),
    ]
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App::new(get_placeholder_tickets());

    let event_handler = EventHandler::new();
    let event_rx = event_handler.start();

    let app_result = app.run(&mut terminal, event_rx);

    ratatui::restore();

    app_result
}
