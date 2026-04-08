use std::time::Duration;
use chrono::Utc;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{Frame, layout::Rect};
use ratatui::layout::{Direction, Layout};
use ratatui::layout::Constraint::{Fill, Percentage};
use crate::{
    app::App,
    ui::components::{Component, input::Input},
};
use crate::events::app_event::{ActionEvent, UiEvent};
use crate::ui::components::Button::Button;
use crate::ui::theme::Theme;

enum Focus {
    Input(usize),
    Button
}

pub struct TimeInputPopup<'a> {
    pub inputs: Vec<Input<'a>>,
    focus: Focus,
    ticket_key: String,
}

impl<'a> TimeInputPopup<'a> {
    pub fn new(ticket_key: String) -> Self {
        let inputs = vec![
            Input::new("Input time - Jira Format", true).placeholder_text("2h30m").border_style(true),
            Input::new("Input Date - Jira Format", false).placeholder_text("21/03/2026").border_style(false),
            Input::new("Input Description", false).placeholder_text("input description...").border_style(false),
        ];

        Self {
            inputs,
            focus: Focus::Input(0),
            ticket_key,
        }
    }

    fn update_focus(&mut self) {
        for (i, input) in self.inputs.iter_mut().enumerate() {
            input.set_border_style(matches!(self.focus, Focus::Input(idx) if idx == i))
        }
    }

    fn next(&mut self) {
        self.focus = match self.focus {
            Focus::Input(i) if i + 1 < self.inputs.len() => Focus::Input(i + 1),
            Focus::Input(_) => Focus::Button,
            Focus::Button => Focus::Input(0),
        };
        self.update_focus();
    }

    fn previous(&mut self) {
        self.focus = match self.focus {
            Focus::Input(0) => Focus::Button,
            Focus::Input(i) => Focus::Input(i - 1),
            Focus::Button => Focus::Input(self.inputs.len() - 1),

        };

        self.update_focus();
    }
}

impl<'a> Component for TimeInputPopup<'a> {
    fn render(&self, _app: &App, frame: &mut Frame, area: Rect, _dt: Duration) {
        let input_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Percentage(25),
                Percentage(25),
                Percentage(25),
                Percentage(25),
            ]).split(area);

        for (i, input) in self.inputs.iter().enumerate() {
                frame.render_widget(&input.textarea, input_area[i]);
        }

        let h_button_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Fill(1),
                Percentage(33),
                Fill(1),
            ]).split(input_area[self.inputs.len()]);

        let v_button_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Fill(1),
                Percentage(33),
                Fill(1),
            ]).split(h_button_area[1]);


        let is_button_selected = matches!(self.focus, Focus::Button);

        frame.render_widget(
            Button::new("Log Time")
                .theme(if is_button_selected {
                    Theme::button_green()
                } else {
                    Theme::button_blue()
                }),
            v_button_area[1],
        );
    }

    fn handle_key(&mut self, key: KeyEvent) -> Option<UiEvent> {
        match &key.code {
            KeyCode::Up | KeyCode::BackTab => {
                self.previous();
                None
            },
            KeyCode::Down | KeyCode::Tab => {
                self.next();
                None
            },
            KeyCode::Enter => {
                if matches!(self.focus, Focus::Button) {
                    // let ticket_key = self.input.textarea.lines().first().unwrap_or(&"".to_string()).clone();
                    let started = Utc::now().format("%Y-%m-%dT%H:%M:%S.000+0000").to_string();
                    // let time_to_log = todo!();

                    return Some(UiEvent::Action(ActionEvent::LogTime {ticket_key: self.ticket_key.clone(), time_spent_seconds: 2000 , started, description: String::from(" This is a description ") }))
                }

                None
            },
            _ => {
                if let Focus::Input(i) = self.focus {
                    if let Some(input) = self.inputs.get_mut(i) {
                        input.textarea.input(key);
                    }
                }

                None
            },
        }

    }
}
