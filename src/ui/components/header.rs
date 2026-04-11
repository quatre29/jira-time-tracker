use crate::app::RenderContext;
use crate::ui::{components::component::Component, theme::Theme};
use ratatui::{layout::Rect, widgets::Paragraph, Frame};
use std::time::Duration;

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
    fn render(&mut self, frame: &mut Frame, area: Rect, context: &RenderContext, _dt: Duration) {
        let header = Paragraph::new(self.title.as_str())
            .style(Theme::title())
            .centered();

        frame.render_widget(header, area);
    }
}
