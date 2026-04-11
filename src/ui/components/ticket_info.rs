use crate::app::App;
use crate::ui::components::Component;
use crate::ui::theme::Theme;
use ratatui::layout::Rect;
use ratatui::prelude::{Modifier, Span, Style};
use ratatui::widgets::{Block, BorderType, Borders};
use ratatui::Frame;
use std::time::Duration;

pub struct TicketInfo {
    title: String,
}

impl TicketInfo {
    pub fn new() -> Self {
        Self {
            title: "Ticket Info".to_string(),
        }
    }
}

impl Component for TicketInfo {
    fn render(&mut self, app: &mut App, frame: &mut Frame, area: Rect, _dt: Duration) {
        frame.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Theme::default_border_color()))
                .title(Span::styled(
                    &self.title,
                    Style::default()
                        .fg(Theme::primary_color())
                        .bg(Theme::panel_background())
                        .add_modifier(Modifier::BOLD),
                ))
                .style(Style::default().bg(Theme::panel_background())),
            area,
        );
    }
}
