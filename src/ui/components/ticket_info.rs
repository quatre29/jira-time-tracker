use std::time::Duration;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Span, Style};
use ratatui::widgets::{Block, BorderType, Borders};
use crate::app::App;
use crate::events::app_event::UiEvent;
use crate::ui::components::Component;
use crate::ui::theme::Theme;

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
    fn render(&self, _app: &App, frame: &mut Frame, area: Rect, _dt: Duration) {
        frame.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Theme::border())
                .title(Span::styled(&self.title, Theme::panel_title()))
                .style(Style::default().bg(Theme::panel_background())),
            area,
        );
    }

    fn handle_key(&mut self, _key: KeyEvent) -> Option<UiEvent> {
        None
    }
}
