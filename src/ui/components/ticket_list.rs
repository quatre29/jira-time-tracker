use std::time::Duration;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
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
    fn render(&self, app: &App, frame: &mut Frame, area: Rect, _dt: Duration) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::HeavyTripleDashed)
            .border_style(Style::default().fg(Theme::default_border_color()))
            .title(Span::styled(
                "Ticket List",
                Style::default()
                    .fg(Theme::primary_color())
                    .bg(Theme::panel_background())
                    .add_modifier(Modifier::BOLD),
            ))
            .style(Style::default().bg(Theme::panel_background()));

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
                let content = format!("{} - {}", ticket.key, ticket.title);

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
