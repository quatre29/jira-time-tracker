use crate::app::RenderContext;
use crate::events::app_event::{ActionEvent, AppEvent, UiEvent};
use crate::ui::components::button::ButtonState;
use crate::ui::components::Button;
use crate::ui::components::Component;
use crate::ui::theme::Theme;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use std::time::Duration;

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
    fn render(&mut self, frame: &mut Frame, area: Rect, _context: &RenderContext, _dt: Duration) {
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
                Constraint::Fill(1),
                Constraint::Length(20),
                Constraint::Length(4),
                Constraint::Length(20),
                Constraint::Fill(1),
            ])
            .split(layout[2]);

        let confirm_state = if self.selected == 0 {
            ButtonState::Selected
        } else {
            ButtonState::Normal
        };

        let cancel_state = if self.selected == 1 {
            ButtonState::Selected
        } else {
            ButtonState::Normal
        };

        frame.render_widget(
            Button::new("Confirm")
                .theme(Theme::button_confirm())
                .state(confirm_state),
            buttons[1],
        );

        frame.render_widget(
            Button::new("Cancel")
                .theme(Theme::button_cancel())
                .state(cancel_state),
            buttons[3],
        );
    }

    fn handle_key(&mut self, key: KeyEvent) -> Option<UiEvent> {
        match key.code {
            KeyCode::Left | KeyCode::Right | KeyCode::Tab => {
                self.selected = (self.selected + 1) % 2;
                None
            }
            KeyCode::Enter => {
                if self.selected == 0 {
                    Some(UiEvent::Action(self.on_confirm.clone()))
                } else {
                    Some(UiEvent::App(AppEvent::ClosePopup))
                }
            }
            _ => {
                None
            }
        }
    }
}