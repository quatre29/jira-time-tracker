use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{Event as CEvent, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::api::config::JiraConfig;
use crate::api::jira_client::JiraClient;
use crate::api::models::{JiraTicket, JiraUser};
use crate::app::LoadState::{Loaded, Loading};
use crate::events::app_event::ActionEvent;
use crate::events::app_event::AppEvent;
use crate::events::dispatcher::dispatch;
use crate::storage::storage::Storage;
use crate::ui::components::{ComponentName, TimeInputDialog};

pub enum PopupState<'a> {
    None,
    InputTime(Box<TimeInputDialog<'a>>),
}

pub enum LoadState<T> {
    Loading,
    Loaded(T),
    Error(String),
}

pub struct App<'a> {
    pub exit: bool,
    pub selected_idx: Option<usize>,
    pub popup: PopupState<'a>,
    pub focused: ComponentName,
    // pub error: Option<String>,
    pub storage: Storage,
    pub jira_client: JiraClient,
    pub app_tx: Sender<AppEvent>,
    pub tick: u64,

    pub tickets_state: LoadState<Vec<JiraTicket>>,
    pub user_state: LoadState<JiraUser>,
}

impl<'a> App<'a> {
    pub fn new(app_tx: Sender<AppEvent>) -> Self {
        let storage = Storage::new();
        let jira_client = JiraClient::new(JiraConfig::from_env());

        dispatch(
            ActionEvent::FetchTickets,
            app_tx.clone(),
            &storage,
            &jira_client,
        );

        dispatch(
            ActionEvent::FetchUser,
            app_tx.clone(),
            &storage,
            &jira_client
        );

        Self {
            exit: false,
            selected_idx: Some(0),
            popup: PopupState::None,
            focused: ComponentName::default(),
            tick: 0,
            // error: None,
            storage,
            jira_client,
            app_tx,

            tickets_state: Loading,
            user_state: Loading,
        }
    }

    pub fn user(&self) -> Option<&JiraUser> {
        match &self.user_state {
            Loaded(user) => Some(user),
            _ => None,
        }
    }

    pub fn tickets(&self) -> Option<&Vec<JiraTicket>> {
        match &self.tickets_state {
            Loaded(tickets) => Some(tickets),
            _ => None,
        }
    }

    pub fn tickets_mut(&mut self) -> Option<&mut Vec<JiraTicket>> {
        match &mut self.tickets_state {
            Loaded(tickets) => Some(tickets),
            _ => None,
        }
    }

    pub fn show_time_input_dialog(&mut self, title: &str) {
        self.popup = PopupState::InputTime(Box::new(TimeInputDialog::new(title)));
    }

    pub fn close_popup(&mut self) {
        self.popup = PopupState::None;
    }

    pub fn on_tick(&mut self) {
        self.tick = self.tick.wrapping_add(1)
    }

    pub async fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        mut app_rx: Receiver<AppEvent>,
    ) -> io::Result<()> {
        let tick_rate = std::time::Duration::from_millis(45);
        let last_tick = Instant::now();

        while !self.exit {
            let now = Instant::now();
            let dt = now - last_tick;

            // TODO: find if delta time is needed (dt)
            terminal.draw(|f| self.draw(f, dt))?;

            tokio::select! {
                Some(event) = app_rx.recv() => {
                    self.handle_event(event).await?;
                }

                _ = tokio::time::sleep(tick_rate) => {
                        self.on_tick();
                    }
            }
        }

        Ok(())
    }

    pub async fn handle_event(&mut self, event: AppEvent) -> io::Result<()> {
        match event {
            AppEvent::KeyEvent(CEvent::Key(key)) => self.handle_key_event(key)?,

            AppEvent::TicketsLoaded(tickets) => {
                self.tickets_state = Loaded(tickets);
                self.selected_idx = Some(0);
            }

            AppEvent::UserLoaded(user) => {
                self.user_state = Loaded(user);
            }

            AppEvent::TicketLoaded(ticket) => {
                todo!()
            }

            AppEvent::TimeLogged(ticket) => {
                todo!()
            }

            AppEvent::Tick => {
                self.on_tick();
            }

            // AppEvent::ApiError(err) => self.error = Some(err),

            _ => {}
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
        let tickets = self.tickets()?;
        let selected_idx = self.selected_idx?;

        tickets.get(selected_idx)
    }

    fn next_ticket(&mut self) {
        if let Some(tickets) = self.tickets() {
            if tickets.is_empty() {
                return;
            }

            self.selected_idx = Some(match self.selected_idx {
                Some(i) => (1 + i) % tickets.len(), // Move down, wrap to top
                None => 0,
            })
        }

    }

    fn previous_ticket(&mut self) {
        if let Some(tickets) = self.tickets() {
            if tickets.is_empty() {
                return;
            }

            self.selected_idx = Some(match self.selected_idx {
                Some(i) if i > 0 => i - 1, // Move up
                Some(_) => tickets.len() - 1, // Wrap to bottom
                None => 0,
            })
        }

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
            ComponentName::TimeInputDialog => self.handle_popup_keys(key_event)?,
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
                    self.show_time_input_dialog(&selected_ticket.title.clone().as_str());
                    self.focus(ComponentName::TimeInputDialog);
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
