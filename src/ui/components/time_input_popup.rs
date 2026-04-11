use crate::app::RenderContext;
use crate::events::app_event::{ActionEvent, UiEvent};
use crate::ui::components::BorderState;
use crate::ui::components::Button;
use crate::ui::components::{input::Input, Component};
use crate::ui::theme::Theme;
use chrono::Utc;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Constraint::{Fill, Percentage};
use ratatui::layout::{Direction, Layout};
use ratatui::prelude::Style;
use ratatui::style::Color;
use ratatui::widgets::Paragraph;
use ratatui::{layout::Rect, Frame};
use std::rc::Rc;
use std::time::Duration;

enum Focus {
    Input(usize),
    Button,
}

pub struct TimeInputPopup<'a> {
    pub inputs: Vec<Input<'a>>,
    focus: Focus,
    ticket_key: String,
    errors: Vec<Option<String>>,
}

impl<'a> TimeInputPopup<'a> {
    pub fn new(ticket_key: String) -> Self {
        let inputs = vec![
            Input::new("Input time - Jira Format", true).placeholder_text("2h30m").border_style(BorderState::Selected),
            Input::new("Input Date - Jira Format", false).placeholder_text("21/03/2026").border_style(BorderState::Default),
            Input::new("Input Description", false).placeholder_text("input description...").border_style(BorderState::Default),
        ];

        Self {
            inputs,
            focus: Focus::Input(0),
            ticket_key,
            errors: vec![None, None, None],
        }
    }

    fn update_focus(&mut self) {
        for (i, input) in self.inputs.iter_mut().enumerate() {
            input.set_border_style(if matches!(self.focus, Focus::Input(idx) if idx == i) {
                BorderState::Selected
            } else {
                BorderState::Default
            })
        }
    }

    fn update_error_borders(&mut self, frame: &mut Frame, area: Rc<[Rect]>) {
        for (i, input) in self.inputs.iter_mut().enumerate() {
            if self.errors.get(i).and_then(|e| e.as_ref()).is_some() {
                input.set_border_style(BorderState::Error)
            }

            frame.render_widget(&input.textarea, area[i]);

            // Render error message under input
            if let Some(Some(err)) = self.errors.get(i) {
                let error_area = Rect {
                    x: area[i].x,
                    y: area[i].y + area[i].height - 1,
                    width: area[i].width,
                    height: 1,
                };

                frame.render_widget(
                    Paragraph::new(err.clone())
                        .style(Style::default().fg(Color::Red)),
                    error_area,
                );
            }
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
    pub fn process_multi_lines_input(&self, input: &[String]) -> String {
        input.join("\n")
    }

    pub fn is_time_input_valid(&self, time_input: &str) -> bool {
        !time_input.trim().is_empty()
    }

    pub fn is_date_input_valid(&self, date_input: &str) -> bool {
        chrono::NaiveDate::parse_from_str(date_input, "%d/%m/%Y").is_ok()
    }
}

impl<'a> Component for TimeInputPopup<'a> {
    fn render(&mut self, frame: &mut Frame, area: Rect, context: &RenderContext, _dt: Duration) {
        let input_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Percentage(25),
                Percentage(25),
                Percentage(25),
                Percentage(25),
            ]).split(area);

        self.update_error_borders(frame, input_area.clone());

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
            }
            KeyCode::Down | KeyCode::Tab => {
                self.next();
                None
            }
            KeyCode::Enter => {
                if matches!(self.focus, Focus::Button) {
                    let time = self.inputs[0].textarea.lines().first().unwrap_or(&"".to_string()).clone();
                    let date = self.inputs[1].textarea.lines().first().unwrap_or(&"".to_string()).clone();

                    let mut has_error = false;

                    // NOTE: reset errors
                    self.errors = vec![None, None, None];

                    if !self.is_time_input_valid(&time) {
                        self.errors[0] = Some("Please input valid time input".to_string());
                        has_error = true;
                    }

                    if !self.is_date_input_valid(&date) {
                        self.errors[1] = Some("Please input valid date input".to_string());
                        has_error = true;
                    }

                    if has_error {
                        return None;
                    }

                    let processed_description = self.process_multi_lines_input(self.inputs[2].textarea.lines());
                    let started = Utc::now().format("%Y-%m-%dT%H:%M:%S.000+0000").to_string();

                    return Some(UiEvent::Action(ActionEvent::LogTime {
                        ticket_key: self.ticket_key.clone(),
                        time_spent_seconds: 2000,
                        started,
                        description: processed_description,
                    }));
                }

                if let Focus::Input(2) = self.focus {
                    if let Some(input) = self.inputs.get_mut(2) {
                        input.textarea.input(key);
                    }
                }

                None
            }
            _ => {
                if let Focus::Input(i) = self.focus {
                    if let Some(input) = self.inputs.get_mut(i) {
                        input.textarea.input(key);
                    }
                }

                None
            }
        }
    }
}
