use std::collections::{HashSet, VecDeque};
use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{Event as CEvent, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::api::config::JiraConfig;
use crate::api::jira_client::JiraClient;
use crate::api::models::{JiraTicket, JiraUser};
use crate::app::LoadState::{Loaded, Loading};
use crate::events::app_event::AppEvent;
use crate::events::app_event::{ActionEvent, UiError, UiEvent};
use crate::events::dispatcher::dispatch;
use crate::events::effect::Effect;
use crate::storage::storage::Storage;
use crate::ui::components::popup::Popup;
use crate::ui::components::{
    Component, ComponentName, ConfirmationPopup, TicketInputPopup, TimeInputPopup,
};
use crate::ui::notifications::toast_manager::ToastManager;

pub enum PopupState<'a> {
    None,
    Active(Box<dyn Component + 'a>),
}

pub enum LoadState<T> {
    Loading,
    Loaded(T),
    Error(String),
}

pub struct RenderContext<'a> {
    pub tickets_state: &'a LoadState<Vec<JiraTicket>>,
    pub user_state: &'a LoadState<JiraUser>,
    pub selected_idx: Option<usize>,
    pub focused: &'a ComponentName,
    pub tick: u64,
    pub expanded_keys: &'a HashSet<String>,
}

pub struct App<'a> {
    pub exit: bool,
    pub selected_idx: Option<usize>,
    pub popup: PopupState<'a>,
    pub focused: ComponentName,
    pub storage: Storage,
    pub jira_client: JiraClient,
    pub app_tx: Sender<AppEvent>,
    pub tick: u64,

    pub ui_errors: Vec<UiError>,

    pub tickets_state: LoadState<Vec<JiraTicket>>,
    pub user_state: LoadState<JiraUser>,

    pub pending_events: VecDeque<AppEvent>,
    pub toast_manager: ToastManager,
    pub expanded_keys: HashSet<String>,
    pub loading_subtasks: HashSet<String>,
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
            storage,
            jira_client,
            app_tx,

            ui_errors: vec![],

            tickets_state: Loading,
            user_state: Loading,

            pending_events: vec![].into(),
            toast_manager: ToastManager::new(),
            expanded_keys: HashSet::new(),
            loading_subtasks: HashSet::new(),
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

    pub fn show_popup<C>(&mut self, title: &str, width: u16, height: u16, content: C)
    where
        C: Component + 'static,
    {
        let popup = Popup::new(title, content).size(width, height);

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
        dispatch(
            action,
            self.app_tx.clone(),
            &self.storage,
            &self.jira_client,
        );
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
            }

            AppEvent::TicketsLoaded(tickets) => {
                self.tickets_state = Loaded(tickets);
                self.selected_idx = Some(0);
                vec![]
            }

            AppEvent::UserLoaded(user) => {
                self.toast_manager.push_success(format!("User {} loaded successfully!", user.display_name));
                self.user_state = Loaded(user);
                vec![]
            }

            AppEvent::TicketLoaded(ticket) => {
                let ticket_key = ticket.key.clone();

                match self.tickets_mut() {
                    Some(tickets) => {
                        if let Some(existing) = tickets.iter_mut().find(|t| t.key == ticket.key) {
                            *existing = ticket;
                        } else {
                            tickets.push(ticket);
                        }
                    }
                    None => self.tickets_state = Loaded(vec![ticket]),
                };

                self.close_popup();
                self.toast_manager.push_success(format!("Ticket {} loaded successfully", ticket_key));

                vec![]
            }

            AppEvent::TimeLogged { ticket_key } => {
                self.toast_manager.push_success("Time logged successfully");
                vec![Effect::Action(ActionEvent::FetchTicket { ticket_key })]
            }

            AppEvent::TicketRemoved { ticket_key } => {
                if let Some(tickets) = self.tickets_mut() {
                    tickets.retain(|ticket| ticket.key != ticket_key);
                }

                self.toast_manager.push_success(format!("Ticket {} removed successfully!", ticket_key));

                vec![]
            }

            AppEvent::Tick => {
                self.on_tick();

                vec![]
            }

            AppEvent::ClosePopup => {
                self.close_popup();
                vec![]
            }

            AppEvent::ConfirmPopup => {
                vec![]
            }

            AppEvent::UiError(err) => {
                self.ui_errors.push(err);
                vec![]
            }

            AppEvent::SubtasksLoaded { parent_key, subtasks } => {
                self.loading_subtasks.remove(&parent_key);
                if let LoadState::Loaded(tickets) = &mut self.tickets_state {
                    if let Some(parent) = tickets.iter_mut().find(|t| t.key == parent_key) {
                        parent.subtasks = subtasks;
                    }
                }
                vec![]
            }

            AppEvent::ApiError(err) => {
                self.ui_errors.push(UiError::Global { message: err.clone() });
                self.toast_manager.push_error(err);
                vec![]
            }
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

    fn draw(&mut self, frame: &mut Frame, dt: Duration) {
        crate::ui::render(frame, self, dt);
    }

    fn flat_len(&self) -> usize {
        let tickets = match &self.tickets_state {
            LoadState::Loaded(t) => t,
            _ => return 0,
        };
        tickets.iter().map(|t| {
            1 + if self.expanded_keys.contains(&t.key) { t.subtasks.len() } else { 0 }
        }).sum()
    }

    fn selected_ticket(&self) -> Option<&JiraTicket> {
        let idx = self.selected_idx?;
        let tickets = match &self.tickets_state {
            LoadState::Loaded(t) => t,
            _ => return None,
        };
        let mut flat = 0;
        for ticket in tickets {
            if flat == idx { return Some(ticket); }
            flat += 1;
            if self.expanded_keys.contains(&ticket.key) {
                for subtask in &ticket.subtasks {
                    if flat == idx { return Some(subtask); }
                    flat += 1;
                }
            }
        }
        None
    }

    fn next_ticket(&mut self) {
        let len = self.flat_len();
        if len == 0 { return; }
        self.selected_idx = Some(match self.selected_idx {
            Some(i) => (i + 1) % len,
            None => 0,
        });
    }

    fn previous_ticket(&mut self) {
        let len = self.flat_len();
        if len == 0 { return; }
        self.selected_idx = Some(match self.selected_idx {
            Some(0) | None => len - 1,
            Some(i) => i - 1,
        });
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

    /// If the flat item at `idx` is a parent, returns `(key, subtask_keys, subtasks_loaded)`.
    fn parent_at(&self, idx: usize) -> Option<(String, Vec<String>, bool)> {
        let tickets = match &self.tickets_state {
            LoadState::Loaded(t) => t,
            _ => return None,
        };
        let mut flat = 0;
        for ticket in tickets {
            if flat == idx {
                return Some((
                    ticket.key.clone(),
                    ticket.subtask_keys.clone(),
                    !ticket.subtasks.is_empty(),
                ));
            }
            flat += 1;
            if self.expanded_keys.contains(&ticket.key) {
                flat += ticket.subtasks.len();
            }
        }
        None
    }

    /// If the flat item at `idx` is a subtask, returns the parent key.
    fn subtask_parent_at(&self, idx: usize) -> Option<String> {
        let tickets = match &self.tickets_state {
            LoadState::Loaded(t) => t,
            _ => return None,
        };
        let mut flat = 0;
        for ticket in tickets {
            flat += 1; // parent slot
            if self.expanded_keys.contains(&ticket.key) {
                for _ in &ticket.subtasks {
                    if flat == idx {
                        return Some(ticket.key.clone());
                    }
                    flat += 1;
                }
            }
        }
        None
    }

    /// Returns the flat index of a parent ticket by key.
    fn parent_flat_idx(&self, parent_key: &str) -> Option<usize> {
        let tickets = match &self.tickets_state {
            LoadState::Loaded(t) => t,
            _ => return None,
        };
        let mut flat = 0;
        for ticket in tickets {
            if ticket.key == parent_key {
                return Some(flat);
            }
            flat += 1;
            if self.expanded_keys.contains(&ticket.key) {
                flat += ticket.subtasks.len();
            }
        }
        None
    }

    fn handle_ticket_list_keys(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('t') if matches!(self.popup, PopupState::None) => {
                self.show_popup("Add Ticket", 20, 10, TicketInputPopup::new());
                self.focus(ComponentName::TicketInputPopup);
            }

            KeyCode::Char('d') => {
                let selected_ticket = self.selected_ticket();

                if let Some(ticket) = selected_ticket {
                    let ticket_key = ticket.key.clone();

                    self.show_popup(
                        "Confirmation",
                        40,
                        20,
                        ConfirmationPopup::new(
                            "Are you sure you want to remove this ticket?",
                            ActionEvent::RemoveTicket { ticket_key },
                        ),
                    );
                    self.focus(ComponentName::ConfirmationPopup);
                }
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

            KeyCode::Right if matches!(self.popup, PopupState::None) => {
                if let Some(idx) = self.selected_idx {
                    if let Some((parent_key, subtask_keys, already_loaded)) = self.parent_at(idx) {
                        self.expanded_keys.insert(parent_key.clone());
                        if !already_loaded && !self.loading_subtasks.contains(&parent_key) {
                            self.loading_subtasks.insert(parent_key.clone());
                            self.dispatch(ActionEvent::FetchSubtasks { parent_key, subtask_keys });
                        }
                    }
                }
            }

            KeyCode::Left if matches!(self.popup, PopupState::None) => {
                if let Some(idx) = self.selected_idx {
                    if let Some(parent_key) = self.subtask_parent_at(idx) {
                        // On a subtask — jump to parent and collapse
                        let parent_flat = self.parent_flat_idx(&parent_key);
                        self.expanded_keys.remove(&parent_key);
                        self.selected_idx = parent_flat.or(Some(0));
                    } else if let Some((key, _, _)) = self.parent_at(idx) {
                        // On an expanded parent — collapse it
                        self.expanded_keys.remove(&key);
                    }
                }
            }

            KeyCode::Enter if matches!(self.popup, PopupState::None) => {
                let selected_ticket = self.selected_ticket();

                if let Some(selected_ticket) = selected_ticket {
                    self.show_popup(
                        selected_ticket.title.clone().as_str(),
                        40,
                        40,
                        TimeInputPopup::new(self.selected_ticket().unwrap().key.clone()),
                    );
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
            }
            PopupState::None => {}
        }

        Ok(())
    }
}
