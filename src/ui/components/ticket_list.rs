use std::time::Duration;
use crossterm::event::KeyEvent;
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
use crate::app::LoadState;
use crate::events::app_event::{AppEvent, UiEvent};

pub struct TicketList {
    title: String,
}

impl TicketList {
    pub fn new() -> Self {
        Self {
            title: "Ticket List".to_string(),
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
                &self.title,
                Style::default()
                    .fg(Theme::primary_color())
                    .bg(Theme::panel_background())
                    .add_modifier(Modifier::BOLD),
            ))
            .style(Style::default().bg(Theme::panel_background()));

        match &app.tickets_state {
            LoadState::Loading => {
                let loading = ratatui::widgets::Paragraph::new("Loading tickets...")
                    .block(block)
                    .style(Theme::dimmed());

                frame.render_widget(loading, area);
            },
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
                        ListItem::new(format!("{} - {}", ticket.key, ticket.title))
                    })
                    .collect();

                let list = List::new(items)
                    .highlight_style(Theme::selected())
                    .highlight_symbol(">> ")
                    .block(block);

                let mut state = ListState::default();
                state.select(app.selected_idx);

                frame.render_stateful_widget(list, area, &mut state);
            },
            LoadState::Error(err) => {
                let error = ratatui::widgets::Paragraph::new(err.clone())
                    .block(block)
                    .style(Style::default().fg(Color::Red));

                frame.render_widget(error, area);
            }
        }
    }
}
