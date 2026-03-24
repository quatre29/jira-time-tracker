use std::io;

use api::models::JiraTicket;
use app::App;

use crate::{
    api::{
        config::JiraConfig,
        jira_client::{self, JiraClient},
    },
    events::app_event::AppEvent,
    storage::storage::Storage,
};

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

    let storage = Storage {};
    let jira_client = JiraClient::new(JiraConfig::from_env());
    let initial_tickets = get_initial_tickets_stored(storage, jira_client.clone()).await?;

    let mut app = App::new(initial_tickets, storage, jira_client.clone());

    let (app_tx, app_rx) = tokio::sync::mpsc::channel::<AppEvent>(100);

    let _read_handle = terminal_input_reader::run_input_read(app_tx.clone())?;
    let app_result = app.run(&mut terminal, app_rx).await;

    ratatui::restore();

    app_result?;

    Ok(())
}

async fn get_initial_tickets_stored(
    storage: Storage,
    client: JiraClient,
) -> Result<Vec<JiraTicket>> {
    //TODO: To be passed into fetch_tickets once we move storage
    let stored_keys = storage.load_ticket_keys()?;
    let tickets = client.fetch_tickets(stored_keys).await?;

    Ok(tickets)
}
