use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style, Stylize},
    widgets::{Block, BorderType, Clear},
};
use std::time::{Duration, Instant};
use tachyonfx::{Duration as FxDuration, EffectRenderer, fx};

use crate::{
    app::App,
    ui::{
        components::Component,
        theme::Theme,
    },
};
use crate::events::app_event::UiEvent;

pub struct Popup<'a, C: Component> {
    title: String,
    width: u16,
    height: u16,
    animation_start_time: Instant,
    content: C,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a, C: Component> Popup<'a, C> {
    pub fn new(title: impl Into<String>, content: C) -> Self {
        Self {
            title: title.into(),
            width: 40,
            height: 30,
            animation_start_time: Instant::now(),
            content,
            _marker: Default::default(),
        }
    }

    pub fn size(mut self, width: u16, height: u16) -> Self {
        self.width = width;
        self.height = height;
        self
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

impl<'a, C: Component> Component for Popup<'a, C> {
    fn render(&self, app: &App, frame: &mut Frame, area: Rect, dt: Duration) {
        // Dim overlay
        frame.render_widget(
            Block::default().style(
                Style::default()
                    .bg(Theme::bg())
                    .add_modifier(Modifier::DIM),
            ),
            area,
        );

        let area = self.centered_rect_percent(self.width, self.height, area);

        frame.render_widget(Clear, area);

        let block = Block::bordered()
            .title(self.title.as_str().add_modifier(Modifier::BOLD))
            .title_style(Theme::popup_title())
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Double)
            .border_style(Theme::popup_border())
            .style(Style::default().bg(Theme::popup_background()));

        let inner_area = block.inner(area);

        self.content.render(app, frame, inner_area, dt);

        let mut fade_effect = fx::coalesce_from(
            Style::default(),
            (1000, tachyonfx::Interpolation::ExpoInOut),
        );

        let duration =
            FxDuration::from_millis(self.animation_start_time.elapsed().as_millis() as u32);

        frame.render_widget(block, area);
        frame.render_effect(&mut fade_effect, area, duration);
    }

    fn handle_key(&mut self, key: KeyEvent) -> Option<UiEvent> {
        self.content.handle_key(key)
    }
}
