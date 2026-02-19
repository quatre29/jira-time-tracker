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
    ui::{
        components::{Component, input::Input},
        theme::Theme,
    },
};

pub struct TimeInputDialog<'a> {
    pub time_input_textarea: Input<'a>,
    title: String,
    border_color: Color,
    width: u16,
    height: u16,
    animation_start_time: Instant,
}

impl<'a> TimeInputDialog<'a> {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            border_color: Theme::default_border_color(),
            width: 40,
            height: 30,
            animation_start_time: Instant::now(),
            time_input_textarea: Input::new("Iput time - Jira Format").placeholder_text("2h30m"),
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

    fn centered_rect_percent(&self, percent_x: u16, percent_y: u16, area: Rect) -> Rect {
        let vertical = Layout::vertical([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ]);

        let horizontal = Layout::horizontal([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ]);

        let [_, middle, _] = vertical.areas(area);
        let [_, center, _] = horizontal.areas(middle);

        center
    }
}

// FIXME: the structure of rendering stuff inside the input dialog popup is chaotic
impl<'a> Component for TimeInputDialog<'a> {
    fn render(&self, _app: &App, frame: &mut Frame, area: Rect, _dt: Duration) {
        let area = self.centered_rect_percent(self.width, self.height, area);

        frame.render_widget(Clear, area);

        let block = Block::bordered()
            .title(self.title.as_str().add_modifier(Modifier::BOLD))
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .border_style(self.border_color);

        let inner_area = block.inner(area);

        frame.render_widget(self.time_input_textarea.textarea.widget(), inner_area);

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
