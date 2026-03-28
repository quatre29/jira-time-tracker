use std::time::Duration;
use crossterm::event::{KeyEvent, KeyCode};
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::app::App;
use crate::events::app_event::{ActionEvent, AppEvent};
use crate::ui::components::{Component, Input};


pub struct TicketInputPopup<'a> {
    pub input: Input<'a>
}

impl<'a> TicketInputPopup <'a> {
    pub fn new() -> Self {
        Self {
            input: Input::new("Input ticket key").placeholder_text("EXAMPLE-1"),
        }
    }
}

impl <'a> Component for TicketInputPopup <'a> {
    fn render(&self, app: &App, frame: &mut Frame, area: Rect, dt: Duration) {
        frame.render_widget(self.input.textarea.widget(), area);
    }

    fn handle_key(&mut self, key: KeyEvent) -> Option<AppEvent> {
        match key.code {
         KeyCode::Enter => {
             let ticket_key = self.input.textarea.lines().first().unwrap_or(&"".to_string()).clone();

             Some(AppEvent::Action(ActionEvent::FetchTicket { ticket_key }))
         }
            _ => {
                self.input.textarea.input(key);

                None
            },
        }
    }
}