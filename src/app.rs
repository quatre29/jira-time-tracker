use std::{io, sync::mpsc};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::events::Event;
use crate::jira::models::JiraTicket;

pub struct App {
    pub exit: bool,
    pub tickets: Vec<JiraTicket>,
}

impl App {
    // TODO: Read the tickets straight from storage.rs
    pub fn new(tickets: Vec<JiraTicket>) -> Self {
        Self {
            exit: false,
            tickets,
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        rx: mpsc::Receiver<Event>,
    ) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

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

    fn handle_key_event(&mut self, key_event: KeyEvent) -> io::Result<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }

        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            _ => {}
        }

        Ok(())
    }
}
