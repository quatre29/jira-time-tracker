use std::time::Duration;

use ratatui::{Frame, layout::Rect};

use crate::app::App;

#[derive(Default, PartialEq, Eq)]
pub enum ComponentName {
    Header,
    #[default]
    TicketList,
    InputDialog,
}

pub trait Component {
    fn render(&self, app: &App, frame: &mut Frame, area: Rect, dt: Duration);
}
