use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, List, ListItem, ListState},
};

use crate::{
    app::App,
    ui::{components::Component, theme::Theme},
};

pub struct TicketList {
    title: String,
}

impl TicketList {
    pub fn new() -> Self {
        Self {
            title: "Tickets".to_string(),
        }
    }
}

impl Component for TicketList {
    fn render(&self, app: &App, frame: &mut Frame, area: Rect) {
        let block = Block::bordered()
            .title(self.title.as_str())
            .border_style(Theme::border());

        if app.tickets.is_empty() {
            let empty = ratatui::widgets::Paragraph::new("No tickets found")
                .block(block.clone())
                .style(Theme::dimmed());

            frame.render_widget(empty, area);

            return;
        }

        let items: Vec<ListItem> = app
            .tickets
            .iter()
            .map(|ticket| {
                let content = format!("{} - {}", ticket.branch_name, ticket.description);

                ListItem::new(content)
            })
            .collect();

        let list = List::new(items)
            .highlight_style(Theme::selected())
            .highlight_symbol(">> ")
            .block(block);

        let mut state = ListState::default();
        state.select(app.selected_idx);

        frame.render_stateful_widget(list, area, &mut state);
    }
}
