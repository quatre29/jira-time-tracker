use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event as CEvent, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::jira::models::JiraTicket;
use crate::ui::components::TimeInputDialog;

pub enum PopupState {
    None,
    InputTime(TimeInputDialog),
}

pub struct App {
    pub exit: bool,
    pub tickets: Vec<JiraTicket>,
    pub selected_idx: Option<usize>,
    pub popup: PopupState,
}

impl App {
    // TODO: Read the tickets straight from storage.rs
    pub fn new(tickets: Vec<JiraTicket>) -> Self {
        Self {
            exit: false,
            tickets,
            selected_idx: Some(0),
            popup: PopupState::None,
        }
    }

    pub fn show_time_input_dialog(&mut self, title: &str) {
        self.popup = PopupState::InputTime(TimeInputDialog::new(title));
    }

    pub fn close_popup(&mut self) {
        self.popup = PopupState::None;
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let tick_rate = std::time::Duration::from_millis(16); // ~60 FPS
        let mut last_frame = Instant::now();

        while !self.exit {
            let now = Instant::now();
            let dt = now - last_frame;
            last_frame = now;

            // TODO: find if delta time is needed (dt)
            terminal.draw(|f| self.draw(f, dt))?;

            let timeout = tick_rate
                .checked_sub(last_frame.elapsed())
                .unwrap_or(Duration::ZERO);

            if event::poll(timeout)? {
                if let CEvent::Key(key) = event::read()? {
                    self.handle_key_event(key)?;
                }
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame, dt: Duration) {
        crate::ui::render(frame, self, dt);
    }

    fn selected_ticket(&self) -> Option<&JiraTicket> {
        self.selected_idx.and_then(|i| self.tickets.get(i))
    }

    fn next_ticket(&mut self) {
        if self.tickets.is_empty() {
            return;
        }

        self.selected_idx = Some(match self.selected_idx {
            Some(i) => (1 + i) % self.tickets.len(), // Move down, wrap to top
            None => 0,
        })
    }

    fn previous_ticket(&mut self) {
        if self.tickets.is_empty() {
            return;
        }

        self.selected_idx = Some(match self.selected_idx {
            Some(i) if i > 0 => i - 1,         // Move up
            Some(_) => self.tickets.len() - 1, // Wrap to bottom
            None => 0,
        })
    }

    // TODO: Move to events.rs
    fn handle_key_event(&mut self, key_event: KeyEvent) -> io::Result<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }

        match &mut self.popup {
            PopupState::InputTime(dialog) => match key_event.code {
                KeyCode::Esc => self.close_popup(),
                _ => {}
            },
            PopupState::None => {}
        }

        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Up => {
                if let PopupState::None = self.popup {
                    self.previous_ticket();
                }
            }
            KeyCode::Down => {
                if let PopupState::None = self.popup {
                    self.next_ticket();
                }
            }
            KeyCode::Enter if matches!(self.popup, PopupState::None) => {
                let selected_ticket = self.selected_ticket();

                if let Some(selected_ticket) = selected_ticket {
                    // TODO: fix this lifetime hack - clone().as_str()
                    // we might need to pass the ticket ---- or maybe we can get the ticket details
                    // from the popup::app.selected_ticket()
                    self.show_time_input_dialog(selected_ticket.branch_name.clone().as_str());
                }
            }
            _ => {}
        }

        Ok(())
    }
}
