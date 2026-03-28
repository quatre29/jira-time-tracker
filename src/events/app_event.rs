use derive_more::From;

use crate::api::models::{JiraTicket, JiraUser};

#[derive(From)]
pub enum AppEvent {
    #[from]
    KeyEvent(crossterm::event::Event),

    TicketsLoaded(Vec<JiraTicket>),
    TicketLoaded(JiraTicket),
    UserLoaded(JiraUser),
    TimeLogged(JiraTicket),
    ApiError(String),
    TicketRemoved { ticket_key: String },

    #[from]
    Action(ActionEvent),

    ConfirmPopup,
    CancelPopup,

    Tick,
}

pub enum ActionEvent {
    FetchTickets,
    FetchTicket { ticket_key: String },
    RemoveTicket { ticket_key: String },
    LogTime { ticket_key: String, time: u32 },
    FetchUser,

}
