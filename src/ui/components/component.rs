use crate::app::RenderContext;
use crate::events::app_event::UiEvent;
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};
use std::time::Duration;

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
    fn render(&mut self, frame: &mut Frame, area: Rect, context: &RenderContext, dt: Duration);

    fn handle_key(&mut self, _key: KeyEvent) -> Option<UiEvent> {
        None
    }
}
