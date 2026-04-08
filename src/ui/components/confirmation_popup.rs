use std::time::Duration;
use crossterm::event::{KeyEvent, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::widgets::Paragraph;
use crate::app::App;
use crate::events::app_event::{ActionEvent, AppEvent, UiEvent};
use crate::ui::components::Component;
use crate::ui::components::Button::Button;
use crate::ui::theme::Theme;

pub struct ConfirmationPopup  {
    confirmation_text: String,
    selected: usize,
    on_confirm: ActionEvent,
}

impl ConfirmationPopup  {
    pub fn new(confirmation_text: impl Into<String>, on_confirm: ActionEvent) -> Self {
        Self {
            confirmation_text: confirmation_text.into(),
            selected: 0,
            on_confirm
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
                .alignment(Alignment::Center),
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

        frame.render_widget(Button::new("Confirm").theme(if self.selected == 0 {Theme::button_green()} else {Theme::button_blue()}), buttons[1]);
        frame.render_widget(Button::new("Cancel").theme(if self.selected == 1 {Theme::button_green()} else {Theme::button_blue()}), buttons[3]);
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
            _ => {
                None
            },
        }

    }
}