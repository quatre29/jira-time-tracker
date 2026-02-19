use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event as CEvent, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};

use crate::jira::models::JiraTicket;
use crate::ui::components::{ComponentName, TimeInputDialog};

pub enum PopupState<'a> {
    None,
    InputTime(Box<TimeInputDialog<'a>>),
}

pub struct App<'a> {
    pub exit: bool,
    pub tickets: Vec<JiraTicket>,
    pub selected_idx: Option<usize>,
    pub popup: PopupState<'a>,
    pub focused: ComponentName,
}

impl<'a> App<'a> {
    // TODO: Read the tickets straight from storage.rs
    pub fn new(tickets: Vec<JiraTicket>) -> Self {
        Self {
            exit: false,
            tickets,
            selected_idx: Some(0),
            popup: PopupState::None,
            focused: ComponentName::default(),
        }
    }

    pub fn show_time_input_dialog(&mut self, title: &str) {
        self.popup = PopupState::InputTime(Box::new(TimeInputDialog::new(title)));
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

    pub fn is_focused(&self, component_name: &ComponentName) -> bool {
        &self.focused == component_name
    }

    pub fn focus(&mut self, component_name: ComponentName) {
        self.focused = component_name;
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

        //NOTE: FORCE QUIT
        if key_event.code == KeyCode::Char('c')
            && key_event.modifiers.contains(KeyModifiers::CONTROL)
        {
            self.exit = true;

            return Ok(());
        }

        match self.focused {
            ComponentName::TicketList => self.handle_ticket_list_keys(key_event)?,
            ComponentName::InputDialog => self.handle_popup_keys(key_event)?,
            _ => {}
        }

        Ok(())
    }

    // TODO: This should be handled by its Component
    fn handle_ticket_list_keys(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
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
                    self.show_time_input_dialog(selected_ticket.branch_name.clone().as_str());
                    self.focus(ComponentName::InputDialog);
                }
            }
            _ => {}
        }

        Ok(())
    }

    // TODO: This should be handled by its Component
    fn handle_popup_keys(&mut self, key: KeyEvent) -> io::Result<()> {
        match &mut self.popup {
            PopupState::InputTime(dialog) => match key.code {
                KeyCode::Esc => {
                    self.close_popup();
                    self.focus(ComponentName::TicketList);
                }
                KeyCode::Enter => {
                    // TODO: Log time | Validate input -> Error || POST
                }
                _ => {
                    dialog.time_input_textarea.textarea.input(key);
                }
            },
            PopupState::None => {}
        }

        Ok(())
    }
}
