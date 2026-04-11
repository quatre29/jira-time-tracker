use crate::app::{App, LoadState};
use crate::ui::components::Component;
use crate::ui::theme::Theme;
use ratatui::layout::Rect;
use ratatui::prelude::{Modifier, Span, Style};
use ratatui::style::Color;
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
            title: "User Info".to_string(),
        }
    }
}

impl Component for UserInfo {
    fn render(&mut self, app: &mut App, frame: &mut Frame, area: Rect, _dt: Duration) {
        let content = match &app.user_state {
            LoadState::Loading => vec![Line::from("Loading user...").style(Theme::dimmed())],

            LoadState::Loaded(user) => vec![
                Line::from(format!("Name: {}", user.display_name)),
                Line::from(format!("Email: {}", user.email_address)),
                Line::from(format!("Timezone: {}", user.time_zone)),
            ],

            LoadState::Error(err) => vec![
                Line::from(Span::styled("Error", Style::default().fg(Color::Red))),
                Line::from(err.clone()),
            ],
        };

        let paragraph = Paragraph::new(content).block(
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
        );

        frame.render_widget(paragraph, area);
    }
}
