use ratatui::{layout::Rect, widgets::Paragraph, Frame};
use std::time::Duration;

use crate::{
    app::App,
    ui::{components::component::Component, theme::Theme},
};

pub struct Header {
    title: String,
}

impl Header {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }
}

impl Component for Header {
    fn render(&mut self, _app: &mut App, frame: &mut Frame, area: Rect, _dt: Duration) {
        let header = Paragraph::new(self.title.as_str())
            .style(Theme::title())
            .centered();

        frame.render_widget(header, area);
    }
}
