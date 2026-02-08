use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, List, ListItem, ListState},
};

use crate::{
    jira::models::JiraTicket,
    ui::{components::Component, theme::Theme},
};

pub struct TicketList<'a> {
    tickets: &'a [JiraTicket],
    selected_idx: Option<usize>,
    title: String,
}

impl<'a> TicketList<'a> {
    pub fn new(tickets: &'a [JiraTicket]) -> Self {
        Self {
            tickets,
            selected_idx: None,
            title: "Tickets".to_string(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn selected(mut self, idx: Option<usize>) -> Self {
        self.selected_idx = idx;
        self
    }
}

impl Component for TicketList<'_> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered()
            .title(self.title.as_str())
            .border_style(Theme::border());

        if self.tickets.is_empty() {
            let empty = ratatui::widgets::Paragraph::new("No tickets found")
                .block(block)
                .style(Theme::dimmed());

            frame.render_widget(empty, area);
        }

        let items: Vec<ListItem> = self
            .tickets
            .iter()
            .map(|ticket| {
                let content = format!("{} - {}", ticket.branch_name, ticket.description);

                ListItem::new(content)
            })
            .collect();

        let list = List::new(items);
        let mut state = ListState::default();
        state.select(self.selected_idx);

        frame.render_stateful_widget(list, area, &mut state);
    }
}
