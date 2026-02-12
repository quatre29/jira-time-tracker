use std::time::Duration;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Stylize},
    widgets::{Block, Clear},
};
use tachyonfx::{Duration as FxDuration, EffectRenderer, EffectTimer, Interpolation, fx};

use crate::{
    app::App,
    ui::{components::Component, theme::Theme},
};

pub struct TimeInputDialog {
    title: String,
    border_color: Color,
    width: u16,
    height: u16,
}

impl TimeInputDialog {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            border_color: Theme::default_border_color(),
            width: 60,
            height: 10,
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
    fn render(&self, app: &App, frame: &mut Frame, area: Rect) {
        let area = self.centered_rect(self.width, self.height, area);

        frame.render_widget(Clear, area);

        let block = Block::bordered()
            .title(self.title.as_str().add_modifier(Modifier::BOLD))
            .title_alignment(Alignment::Center)
            .border_style(Theme::border());

        let mut slide_effect = fx::slide_in(
            tachyonfx::Motion::UpToDown,
            100,
            50,
            Color::Cyan,
            EffectTimer::from_ms(300, Interpolation::CubicOut),
        );

        // let duration = duration.as_millis();
        frame.render_effect(&mut slide_effect, area, FxDuration::ZERO);

        frame.render_widget(block, area);
    }
}
