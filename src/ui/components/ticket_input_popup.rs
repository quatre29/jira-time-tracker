use std::time::Duration;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::app::App;
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

    fn handle_key(&mut self, key: KeyEvent) {
        self.input.textarea.input(key);
    }
}