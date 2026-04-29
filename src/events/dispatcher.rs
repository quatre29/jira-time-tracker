use crate::{
    api::jira_client::JiraClient,
    events::app_event::{ActionEvent, AppEvent},
    storage::storage::Storage,
};
use tokio::sync::mpsc::Sender;

fn http_status_prefix(e: &anyhow::Error) -> String {
    e.downcast_ref::<reqwest::Error>()
        .and_then(|re| re.status())
        .map(|s| format!("{}: ", s.as_u16()))
        .unwrap_or_default()
}

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

                let (tickets, errors) = client.fetch_tickets(ticket_keys).await;

                let _ = app_tx.send(AppEvent::TicketsLoaded(tickets)).await;

                for (key, err) in errors {
                    let _ = app_tx.send(AppEvent::ApiError(format!("{}Failed to fetch ticket {}", http_status_prefix(&err), key))).await;
                }
            });
        }

        ActionEvent::FetchTicket { ticket_key } => {
            tokio::spawn(async move {
                match storage.is_ticket_stored(&ticket_key) {
                    Ok(true) => {
                        let _ = app_tx.send(AppEvent::ApiError(format!("Ticket {} is already loaded!", ticket_key))).await;
                    }
                    Ok(false) => {
                        match client.fetch_ticket(&ticket_key).await {
                            Ok(ticket) => {
                                storage.add_ticket_key(ticket.key.clone()).unwrap();
                                let _ = app_tx.send(AppEvent::TicketLoaded(ticket)).await;
                            }
                            Err(e) => {
                                let _ = app_tx.send(AppEvent::ApiError(format!("{}Failed to fetch ticket {}", http_status_prefix(&e), ticket_key))).await;
                            }
                        }
                    }
                    Err(e) => {
                        let _ = app_tx.send(AppEvent::ApiError(format!("Storage error: {}", e))).await;
                    }
                }
            });
        }

        ActionEvent::FetchSubtasks { parent_key, subtask_keys } => {
            tokio::spawn(async move {
                let (subtasks, errors) = client.fetch_tickets(subtask_keys).await;
                let _ = app_tx.send(AppEvent::SubtasksLoaded { parent_key, subtasks }).await;
                for (key, err) in errors {
                    let _ = app_tx.send(AppEvent::ApiError(
                        format!("{}Failed to fetch subtask {}", http_status_prefix(&err), key)
                    )).await;
                }
            });
        }

        ActionEvent::RemoveTicket { ticket_key } => {
            tokio::spawn(async move {
                match storage.remove_ticket_key(&ticket_key) {
                    Ok(_) => {
                        let _ = app_tx.send(AppEvent::TicketRemoved { ticket_key }).await;
                        let _ = app_tx.send(AppEvent::ClosePopup).await;
                    }
                    Err(_) => {
                        let _ = app_tx.send(AppEvent::ApiError("Storage error: Could not access storage".to_string())).await;
                    }
                }
            });
        }

        ActionEvent::LogTime { ticket_key, time_spent_seconds, started, description } => {
            tokio::spawn(async move {
                match client.log_time(ticket_key.clone(), time_spent_seconds, started, description).await {
                    Ok(_ticket) => {
                        let _ = app_tx.send(AppEvent::TimeLogged { ticket_key }).await;
                    }
                    Err(e) => {
                        let _ = app_tx.send(AppEvent::ApiError(format!("{}Failed to log time on {}", http_status_prefix(&e), ticket_key))).await;
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
                        let _ = app_tx.send(AppEvent::ApiError(format!("{}Failed to fetch user", http_status_prefix(&e)))).await;
                    }
                }
            });
        }
    }
}
