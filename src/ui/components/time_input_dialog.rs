use std::time::{Duration, Instant};

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, BorderType, Clear},
};
use tachyonfx::{Duration as FxDuration, EffectRenderer, fx};

use crate::{
    app::App,
    ui::{components::Component, theme::Theme},
};

pub struct TimeInputDialog {
    title: String,
    border_color: Color,
    width: u16,
    height: u16,
    animation_start_time: Instant,
}

impl TimeInputDialog {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            border_color: Theme::default_border_color(),
            width: 60,
            height: 10,
            animation_start_time: Instant::now(),
        }
    }

    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    pub fn size(mut self, width: u16, height: u16) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    fn centered_rect(&self, width: u16, height: u16, area: Rect) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Length((area.height.saturating_sub(height)) / 2),
            Constraint::Length(height),
            Constraint::Min(0),
        ]);
        let horizontal = Layout::horizontal([
            Constraint::Length((area.width.saturating_sub(width)) / 2),
            Constraint::Length(width),
            Constraint::Min(0),
        ]);

        let [_, middle, _] = vertical.areas(area);
        let [_, center, _] = horizontal.areas(middle);

        center
    }
}

impl Component for TimeInputDialog {
    fn render(&self, app: &App, frame: &mut Frame, area: Rect, dt: Duration) {
        let area = self.centered_rect(self.width, self.height, area);

        frame.render_widget(Clear, area);

        let block = Block::bordered()
            .title(self.title.as_str().add_modifier(Modifier::BOLD))
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .border_style(self.border_color);

        let mut fade_effect = fx::coalesce_from(
            Style::default(),
            (1000, tachyonfx::Interpolation::ExpoInOut),
        );

        let duration =
            FxDuration::from_millis(self.animation_start_time.elapsed().as_millis() as u32);

        frame.render_widget(block, area);
        frame.render_effect(&mut fade_effect, area, duration);
    }
}
