use derive_more::From;

use crate::api::models::{JiraTicket, JiraUser};

#[derive(From)]
pub enum AppEvent {
    #[from]
    KeyEvent(crossterm::event::Event),

    TicketsLoaded(Vec<JiraTicket>),
    TicketLoaded(JiraTicket),
    UserLoaded(JiraUser),
    TimeLogged { ticket_key: String },
    ApiError(String),
    TicketRemoved { ticket_key: String },

    ConfirmPopup,
    ClosePopup,

    Tick,
}

#[derive(Clone)]
pub enum ActionEvent {
    FetchTickets,
    FetchTicket { ticket_key: String },
    RemoveTicket { ticket_key: String },
    LogTime { ticket_key: String, time_spent_seconds: u64, description: String, started: String },
    FetchUser,

}

pub enum UiEvent {
    App(AppEvent),
    Action(ActionEvent),
}
