use crate::app::RenderContext;
use crate::ui::components::Component;
use crate::ui::theme::Theme;
use ratatui::layout::Rect;
use ratatui::prelude::{Span, Style};
use ratatui::widgets::{Block, BorderType, Borders};
use ratatui::Frame;
use std::time::Duration;

pub struct TicketInfo {
    title: String,
}

impl TicketInfo {
    pub fn new() -> Self {
        Self {
            title: " Ticket Info ".to_string(),
        }
    }
}

impl Component for TicketInfo {
    fn render(&mut self, frame: &mut Frame, area: Rect, context: &RenderContext, _dt: Duration) {
        frame.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Theme::border_default())
                .title(Span::styled(&self.title, Theme::panel_title()))
                .style(Style::default().bg(Theme::panel_background())),
            area,
        );
    }
}
