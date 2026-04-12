use ratatui::{
    layout::Rect,
    style::Style,
    text::Span,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame,
};
use std::time::Duration;

use crate::app::{LoadState, RenderContext};
use crate::ui::{components::Component, theme::Theme};

pub struct TicketList {
    title: String,
}

impl TicketList {
    pub fn new() -> Self {
        Self {
            title: " Ticket List ".to_string(),
        }
    }
}

impl Component for TicketList {
    fn render(&mut self, frame: &mut Frame, area: Rect, context: &RenderContext, _dt: Duration) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Theme::border_default())
            .title(Span::styled(&self.title, Theme::panel_title()))
            .style(Style::default().bg(Theme::panel_background()));

        match &context.tickets_state {
            LoadState::Loading => {
                let loading = ratatui::widgets::Paragraph::new("⟳ Loading tickets...")
                    .block(block)
                    .style(Theme::dimmed());

                frame.render_widget(loading, area);
            }
            LoadState::Loaded(tickets) => {
                if tickets.is_empty() {
                    let empty = ratatui::widgets::Paragraph::new("No tickets found")
                        .block(block)
                        .style(Theme::dimmed());

                    frame.render_widget(empty, area);
                    return;
                }

                let items: Vec<ListItem> = tickets
                    .iter()
                    .map(|ticket| {
                        ListItem::new(Span::styled(
                            format!("{} - {}", ticket.key, ticket.title),
                            Theme::text(),
                        ))
                    })
                    .collect();

                let list = List::new(items)
                    .highlight_style(Theme::selected())
                    .highlight_symbol("▸")
                    .block(block);

                let mut state = ListState::default();
                state.select(context.selected_idx);

                frame.render_stateful_widget(list, area, &mut state);
            }
            LoadState::Error(err) => {
                let error = ratatui::widgets::Paragraph::new(err.clone())
                    .block(block)
                    .style(Theme::error());

                frame.render_widget(error, area);
            }
        }
    }
}
