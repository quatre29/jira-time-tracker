use derive_more::From;

use crate::api::models::{JiraTicket, JiraUser, UserStats};
use crate::ui::components::ComponentName;

#[derive(From)]
pub enum AppEvent {
    #[from]
    KeyEvent(crossterm::event::Event),

    TicketsLoaded(Vec<JiraTicket>),
    TicketLoaded(JiraTicket),
    UserLoaded(JiraUser),
    UserStatsLoaded(UserStats),
    SubtasksLoaded { parent_key: String, subtasks: Vec<JiraTicket> },
    TimeLogged { ticket_key: String },
    TicketRemoved { ticket_key: String },

    ApiError(String),
    UiError(UiError),

    ConfirmPopup,
    ClosePopup,

    Tick,
}

#[derive(Clone)]
pub enum ActionEvent {
    FetchTickets,
    FetchTicket { ticket_key: String },
    FetchSubtasks { parent_key: String, subtask_keys: Vec<String> },
    RemoveTicket { ticket_key: String },
    LogTime { ticket_key: String, time_spent_seconds: u64, description: String, started: String },
    FetchUser,
    FetchUserStats,
}

pub enum UiEvent {
    App(AppEvent),
    Action(ActionEvent),
}

#[derive(Debug, Clone)]
pub enum UiError {
    Field {
        component: ComponentName,
        field_index: usize,
        message: String,
    },
    Popup {
        message: String,
    },
    Global {
        message: String,
    },
}
