use std::time::Duration;

use ratatui::{Frame, layout::Rect, widgets::Paragraph};

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
    fn render(&self, _app: &App, frame: &mut Frame, area: Rect) {
        let header = Paragraph::new(self.title.as_str())
            .style(Theme::title())
            .centered();

        frame.render_widget(header, area);
    }
}
