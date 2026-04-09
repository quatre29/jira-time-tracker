use std::time::Duration;
use crossterm::event::{KeyEvent, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::Style;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::app::App;
use crate::events::app_event::{ActionEvent, AppEvent, UiEvent};
use crate::ui::components::Component;
use crate::ui::theme::Theme;

pub struct ConfirmationPopup {
    confirmation_text: String,
    selected: usize,
    on_confirm: ActionEvent,
}

impl ConfirmationPopup {
    pub fn new(confirmation_text: impl Into<String>, on_confirm: ActionEvent) -> Self {
        Self {
            confirmation_text: confirmation_text.into(),
            selected: 0,
            on_confirm,
        }
    }
}

impl Component for ConfirmationPopup {
    fn render(&self, _app: &App, frame: &mut Frame, area: Rect, _dt: Duration) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Min(1),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(area);

        frame.render_widget(
            Paragraph::new(self.confirmation_text.clone())
                .alignment(Alignment::Center)
                .style(Theme::text()),
            layout[1],
        );

        let buttons = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(28),
                Constraint::Percentage(4),
                Constraint::Percentage(28),
                Constraint::Percentage(20),
            ])
            .split(layout[2]);

        let confirm_style = if self.selected == 0 {
            Theme::button_active()
        } else {
            Theme::button_inactive()
        };

        let cancel_style = if self.selected == 1 {
            Theme::button_active()
        } else {
            Theme::button_inactive()
        };

        let confirm_border = if self.selected == 0 {
            Theme::border_focused()
        } else {
            Theme::border()
        };

        let cancel_border = if self.selected == 1 {
            Theme::border_focused()
        } else {
            Theme::border()
        };

        frame.render_widget(
            Paragraph::new(" Confirm ")
                .alignment(Alignment::Center)
                .style(confirm_style)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(confirm_border)
                        .style(Style::default().bg(Theme::popup_background())),
                ),
            buttons[1],
        );

        frame.render_widget(
            Paragraph::new(" Cancel ")
                .alignment(Alignment::Center)
                .style(cancel_style)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(cancel_border)
                        .style(Style::default().bg(Theme::popup_background())),
                ),
            buttons[3],
        );
    }

    fn handle_key(&mut self, key: KeyEvent) -> Option<UiEvent> {
        match key.code {
            KeyCode::Left | KeyCode::Right | KeyCode::Tab => {
                self.selected = (self.selected + 1) % 2;
                None
            },
            KeyCode::Enter => {
                if self.selected == 0 {
                    Some(UiEvent::Action(self.on_confirm.clone()))
                } else {
                    Some(UiEvent::App(AppEvent::ClosePopup))
                }
            },
            _ => None,
        }
    }
}
