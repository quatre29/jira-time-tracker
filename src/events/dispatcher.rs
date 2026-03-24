// use futures::{SinkExt, channel::mpsc::Sender};
use reqwest::Client;
use tokio::sync::mpsc::Sender;

use crate::{
    api::{
        config::JiraConfig,
        jira_client::{self, JiraClient},
    },
    app,
    events::app_event::{ActionEvent, AppEvent},
    storage::storage::Storage,
};

// fn spawn_api_call<F, T, E, MapFn>(
//     future: F,
//     mut app_tx: Sender<AppEvent>,
//     success: fn(T) -> AppEvent,
// ) where
//     F: Future<Output = Result<T, E>> + Send + 'static,
//     T: Send + 'static,
//     E: ToString + Send + 'static,
//     MapFn: Fn(T) -> AppEvent + Send + 'static,
// {
//     tokio::spawn(async move {
//         match future.await {
//             Ok(data) => {
//                 let _ = app_tx.send(success(data)).await;
//             }
//             Err(err) => {
//                 let _ = app_tx.send(AppEvent::ApiError(err.to_string())).await;
//             }
//         }
//     });
// }

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
            tokio::spawn(async move {
                match client.fetch_ticket(&ticket_key).await {
                    Ok(ticket) => {
                        let _ = app_tx.send(AppEvent::TicketLoaded(ticket)).await;
                    }
                    Err(e) => {
                        let _ = app_tx.send(AppEvent::ApiError(e.to_string())).await;
                    }
                }
            });
        }

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

        ActionEvent::FetchUser(user) => {
            tokio::spawn(async move {
                match client.fetch_user(user.user_id).await {
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
