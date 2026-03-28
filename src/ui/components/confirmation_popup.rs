use std::time::Duration;
use crossterm::event::{KeyEvent, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use crate::app::App;
use crate::events::app_event::{ActionEvent, AppEvent};
use crate::ui::components::{Component};
use crate::ui::theme::Theme;

pub struct ConfirmationPopup  {
    confirmation_text: String,
    selected: usize
}

impl ConfirmationPopup  {
    pub fn new(confirmation_text: impl Into<String>) -> Self {
        Self {
            confirmation_text: confirmation_text.into(),
            selected: 0,
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

        let selected_style = Style::default()
            .fg(Theme::focused_border_color())
            .add_modifier(Modifier::BOLD);

        let normal_style = Style::default()
            .fg(Theme::default_border_color())
            .add_modifier(Modifier::BOLD);

        let confirm_style = if self.selected == 0 {
            selected_style
        } else {
            normal_style
        };

        let cancel_style = if self.selected == 1 {
            selected_style
        } else {
            normal_style
        };

        frame.render_widget(
            Paragraph::new(" Confirm ")
                .alignment(Alignment::Center)
                .style(confirm_style)
                .block(Block::default().borders(Borders::ALL).padding(ratatui::widgets::Padding::new(0, 0, 0, 0)),),
            buttons[1],
        );

        frame.render_widget(
            Paragraph::new(" Cancel ")
                .alignment(Alignment::Center)
                .style(cancel_style)
                .block(Block::default().borders(Borders::ALL).padding(ratatui::widgets::Padding::new(0, 0, 0, 0)),),
            buttons[3],
        );
    }

    fn handle_key(&mut self, key: KeyEvent) -> Option<AppEvent> {
        match key.code {
            KeyCode::Left | KeyCode::Right | KeyCode::Tab => {
                self.selected = (self.selected + 1) % 2;
                None
            },
            KeyCode::Enter => {
                if self.selected == 0 {
                    None
                } else {
                    None
                }
            },
            _ => {
                None
            },
        }

    }
}