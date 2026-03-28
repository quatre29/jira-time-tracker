use std::time::Duration;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect};

use crate::{
    app::App,
    ui::components::{Component, input::Input},
};
use crate::events::app_event::{ActionEvent, AppEvent};

pub struct TimeInputPopup<'a> {
    pub input: Input<'a>,
}

impl<'a> TimeInputPopup<'a> {
    pub fn new() -> Self {
        Self {
            input: Input::new("Input time - Jira Format").placeholder_text("2h30m"),
        }
    }
}

impl<'a> Component for TimeInputPopup<'a> {
    fn render(&self, _app: &App, frame: &mut Frame, area: Rect, _dt: Duration) {
        frame.render_widget(self.input.textarea.widget(), area);
    }

    fn handle_key(&mut self, key: KeyEvent) -> Option<AppEvent> {
        match key.code {
            KeyCode::Enter => {
                let ticket_key = self.input.textarea.lines().first().unwrap_or(&"".to_string()).clone();
                // let time_to_log = todo!();

                Some(AppEvent::Action(ActionEvent::LogTime {ticket_key, time: 2 }))
            }
            _ => {
                self.input.textarea.input(key);

                None
            },
        }

    }
}
