use std::time::Duration;

use ratatui::{Frame, layout::Rect};

use crate::app::App;

pub trait Component {
    fn render(&self, app: &App, frame: &mut Frame, area: Rect, dt: Duration);
}
