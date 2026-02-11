use std::{io, sync::mpsc};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::events::Event;
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

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        rx: mpsc::Receiver<Event>,
    ) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            // TODO: Move to events.rs ??
            // TODO: Handle unwrap() with custom errors!
            match rx.recv().unwrap() {
                Event::Input(key_event) => self.handle_key_event(key_event)?,
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        crate::ui::render(frame, self);
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
            KeyCode::Enter => {
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
