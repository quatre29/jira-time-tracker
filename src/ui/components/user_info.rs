use crate::app::{LoadState, RenderContext};
use crate::ui::components::Component;
use crate::ui::theme::Theme;
use ratatui::layout::Rect;
use ratatui::prelude::{Span, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;
use std::time::Duration;

pub struct UserInfo {
    title: String,
}

impl UserInfo {
    pub fn new() -> Self {
        Self {
            title: " User Info ".to_string(),
        }
    }
}

impl Component for UserInfo {
    fn render(&mut self, frame: &mut Frame, area: Rect, context: &RenderContext, _dt: Duration) {
        let content = match &context.user_state {
            LoadState::Loading => vec![Line::from("⟳ Loading user...").style(Theme::dimmed())],

            LoadState::Loaded(user) => vec![
                Line::from(Span::styled(
                    format!("Name: {}", user.display_name),
                    Theme::text(),
                )),
                Line::from(Span::styled(
                    format!("Email: {}", user.email_address),
                    Theme::text(),
                )),
                Line::from(Span::styled(
                    format!("Timezone: {}", user.time_zone),
                    Theme::text(),
                )),
            ],

            LoadState::Error(err) => vec![
                Line::from(Span::styled("Error", Theme::error())),
                Line::from(Span::styled(err.clone(), Theme::error_dim())),
            ],
        };

        let paragraph = Paragraph::new(content).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Theme::border_default())
                .title(Span::styled(&self.title, Theme::panel_title()))
                .style(Style::default().bg(Theme::panel_background())),
        );

        frame.render_widget(paragraph, area);
    }
}
