use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};
use std::time::Duration;

use crate::app::App;
use crate::events::app_event::UiEvent;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ComponentName {
    Header,
    #[default]
    TicketList,
    TimeInputPopup,
    TicketInputPopup,
    ConfirmationPopup,
}

pub trait Component {
    fn render(&mut self, app: &mut App, frame: &mut Frame, area: Rect, dt: Duration);

    fn handle_key(&mut self, _key: KeyEvent) -> Option<UiEvent> {
        None
    }
}
