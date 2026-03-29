use std::collections::VecDeque;
use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{Event as CEvent, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::api::config::JiraConfig;
use crate::api::jira_client::JiraClient;
use crate::api::models::{JiraTicket, JiraUser};
use crate::app::LoadState::{Loaded, Loading};
use crate::events::app_event::{ActionEvent, UiEvent};
use crate::events::app_event::AppEvent;
use crate::events::dispatcher::dispatch;
use crate::events::effect::Effect;
use crate::storage::storage::Storage;
use crate::ui::components::popup::Popup;
use crate::ui::components::{Component, ComponentName, TimeInputPopup, TicketInputPopup, ConfirmationPopup};

pub enum PopupState<'a> {
    None,
    Active(Box<dyn Component + 'a>),
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

    pub pending_events: VecDeque<AppEvent>,
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
            &jira_client,
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

            pending_events: vec![].into(),
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

    pub fn show_popup<C>(&mut self, title: &str, content:C)
    where
    C: Component + 'static
    {
        let popup = Popup::new(title, content).size(40, 20);

        self.popup = PopupState::Active(Box::new(popup));
    }

    pub fn close_popup(&mut self) {
        self.popup = PopupState::None;

        self.focus(ComponentName::TicketList);
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
                    self.pending_events.push_back(event);
                }

                _ = tokio::time::sleep(tick_rate) => {
                        self.on_tick();
                    }
            }

            self.process_pending_events().await?;
        }

        Ok(())
    }

    pub fn dispatch(&self, action: ActionEvent) {
        dispatch(action, self.app_tx.clone(), &self.storage, &self.jira_client);
    }

    async fn process_pending_events(&mut self) -> io::Result<()> {
        while let Some(event) = self.pending_events.pop_front() {
            let effects = self.update(event);

            for effect in effects {
                match effect {
                    Effect::Action(action) => {
                        self.dispatch(action);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn update(&mut self, event: AppEvent) -> Vec<Effect> {
        match event {
            AppEvent::KeyEvent(CEvent::Key(key)) => {
                self.handle_key_event(key).unwrap_or_default();

                vec![]
            },

            AppEvent::TicketsLoaded(tickets) => {
                self.tickets_state = Loaded(tickets);
                self.selected_idx = Some(0);
                vec![]
            }

            AppEvent::UserLoaded(user) => {
                self.user_state = Loaded(user);
                vec![]
            }

            AppEvent::TicketLoaded(ticket) => {
                match self.tickets_mut() {
                    Some(tickets) => {
                        tickets.push(ticket);
                    },
                    None => {
                        self.tickets_state = Loaded(vec![ticket])
                    },
                };

                self.close_popup();

                vec![]
            }

            AppEvent::TimeLogged(ticket) => {
                todo!()
            }

            AppEvent::TicketRemoved { ticket_key } => {
               if let Some(tickets) = self.tickets_mut() {
                   tickets.retain(|ticket| ticket.key != ticket_key);
               }

                vec![]
            },

            AppEvent::Tick => {
                self.on_tick();

                vec![]
            }

            AppEvent::ClosePopup => {
                self.close_popup();
                vec![]
            },

            AppEvent::ConfirmPopup => {
                vec![]
            },

            // AppEvent::ApiError(err) => self.error = Some(err),
            _ => {
                vec![]
            }
        }
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
                Some(i) if i > 0 => i - 1,    // Move up
                Some(_) => tickets.len() - 1, // Wrap to bottom
                None => 0,
            })
        }
    }

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
            ComponentName::TimeInputPopup => self.handle_popup_keys(key_event)?,
            ComponentName::TicketInputPopup => self.handle_popup_keys(key_event)?,
            ComponentName::ConfirmationPopup => self.handle_popup_keys(key_event)?,
            _ => {}
        }

        Ok(())
    }

    fn handle_ticket_list_keys(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('t') if matches!(self.popup, PopupState::None) => {
                self.show_popup("Add Ticket", TicketInputPopup::new());
                self.focus(ComponentName::TicketInputPopup);
            }

            KeyCode::Char('d') => {
                let ticket_key =  self.selected_ticket().unwrap().key.clone();

                self.show_popup("Confirmation", ConfirmationPopup::new("Are you sure you want to remove this ticket?", ActionEvent::RemoveTicket { ticket_key }));
                self.focus(ComponentName::ConfirmationPopup);
            }

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
                    self.show_popup(selected_ticket.title.clone().as_str(), TimeInputPopup::new());
                    self.focus(ComponentName::TimeInputPopup);
                }
            }

            _ => {}
        }

        Ok(())
    }

    fn handle_popup_keys(&mut self, key: KeyEvent) -> io::Result<()> {
        match &mut self.popup {
            PopupState::Active(popup) => {
                if key.code == KeyCode::Esc {
                    self.pending_events.push_back(AppEvent::ClosePopup);
                    return Ok(());
                }

                if let Some(event) = popup.handle_key(key) {
                    match event {
                        UiEvent::App(app_event) => {
                            self.pending_events.push_back(app_event);
                        }
                        UiEvent::Action(action) => {
                            self.dispatch(action);
                        }
                    }
                }
            },
            PopupState::None => {}
        }

        Ok(())
    }
}
