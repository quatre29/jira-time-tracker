use crate::app::RenderContext;
use crate::events::app_event::{ActionEvent, UiEvent};
use crate::ui::components::{Component, Input};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::Frame;
use std::time::Duration;

pub struct TicketInputPopup<'a> {
    pub input: Input<'a>,
}

impl<'a> TicketInputPopup<'a> {
    pub fn new() -> Self {
        Self {
            input: Input::new("Input ticket key", true).placeholder_text("EXAMPLE-1"),
        }
    }
}

impl<'a> Component for TicketInputPopup<'a> {
    fn render(&mut self, frame: &mut Frame, area: Rect, _context: &RenderContext, _dt: Duration) {
        let [_, input_area, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .areas(area);

        frame.render_widget(&self.input.textarea, input_area);
    }

    fn handle_key(&mut self, key: KeyEvent) -> Option<UiEvent> {
        match key.code {
            KeyCode::Enter => {
                let ticket_key = self
                    .input
                    .textarea
                    .lines()
                    .first()
                    .unwrap_or(&"".to_string())
                    .clone();

                Some(UiEvent::Action(ActionEvent::FetchTicket { ticket_key }))
            }
            _ => {
                self.input.textarea.input(key);

                None
            }
        }
    }
}
