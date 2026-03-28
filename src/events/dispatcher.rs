use tokio::sync::mpsc::Sender;
use crate::{
    api::{
        jira_client::{JiraClient},
    },
    events::app_event::{ActionEvent, AppEvent},
    storage::storage::Storage,
};

pub fn dispatch(
    data_event: ActionEvent,
    app_tx: Sender<AppEvent>,
    storage: &Storage,
    client: &JiraClient,
) {
    let client = client.clone();
    let app_tx = app_tx.clone();
    let storage = storage.clone();

    match data_event {
        ActionEvent::FetchTickets => {
            tokio::spawn(async move {
                let ticket_keys = match storage.load_ticket_keys() {
                    Ok(keys) => keys,
                    Err(err) => {
                        _ = app_tx.send(AppEvent::ApiError(err.to_string())).await;

                        return;
                    }
                };

                match client.fetch_tickets(ticket_keys).await {
                    Ok(tickets) => {
                        let _ = app_tx.send(AppEvent::TicketsLoaded(tickets)).await;
                    }
                    Err(e) => {
                        let _ = app_tx.send(AppEvent::ApiError(e.to_string())).await;
                    }
                }
            });
        }

        ActionEvent::FetchTicket { ticket_key } => {
            match storage.is_ticket_stored(&ticket_key) {
                Ok(ticket) => {
                    tokio::spawn(async move {
                        match client.fetch_ticket(&ticket_key).await {
                            Ok(ticket) => {
                                storage.add_ticket_key(ticket.key.clone()).unwrap();

                                let _ = app_tx.send(AppEvent::TicketLoaded(ticket)).await;
                            }
                            Err(e) => {
                                let _ = app_tx.send(AppEvent::ApiError(e.to_string())).await;
                            }
                        }
                    });
                },
                Err(err) => {
                    // TODO: we need to display error - ticket already existing!
                },
            }
        }

        ActionEvent::RemoveTicket { ticket_key } => {
            tokio::spawn(async move {
                storage.remove_ticket_key(&ticket_key).expect("Storage error: Could not access storage");
                app_tx.send(AppEvent::TicketRemoved { ticket_key, }).await
            });

        },

        ActionEvent::LogTime { ticket_key, time } => {
            tokio::spawn(async move {
                match client.log_time(ticket_key, time).await {
                    Ok(ticket) => {
                        let _ = app_tx.send(AppEvent::TimeLogged(ticket)).await;
                    }
                    Err(e) => {
                        let _ = app_tx.send(AppEvent::ApiError(e.to_string())).await;
                    }
                }
            });
        }

        ActionEvent::FetchUser => {
            tokio::spawn(async move {
                match client.fetch_user().await {
                    Ok(user) => {
                        let _ = app_tx.send(AppEvent::UserLoaded(user)).await;
                    }
                    Err(e) => {
                        let _ = app_tx.send(AppEvent::ApiError(e.to_string())).await;
                    }
                }
            });
        }
    }
}
