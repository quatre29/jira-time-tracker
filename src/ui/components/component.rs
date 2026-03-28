use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};
use std::time::Duration;

use crate::app::App;
use crate::events::app_event::AppEvent;

#[derive(Default, PartialEq, Eq)]
pub enum ComponentName {
    Header,
    #[default]
    TicketList,
    TimeInputPopup,
    TicketInputPopup,
}

pub trait Component {
    fn render(&self, app: &App, frame: &mut Frame, area: Rect, dt: Duration);

    fn handle_key(&mut self, _key: KeyEvent) -> Option<AppEvent>;
}
