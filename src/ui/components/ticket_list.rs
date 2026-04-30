use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
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
        let focused = matches!(context.focused, crate::ui::components::ComponentName::TicketList);
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(if focused { Theme::border_focused() } else { Theme::border_default() })
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

                let mut items: Vec<ListItem> = Vec::new();

                for ticket in tickets {
                    let has_subtasks = !ticket.subtask_keys.is_empty();
                    let is_expanded = context.expanded_keys.contains(&ticket.key);
                    let is_loading = !ticket.subtask_keys.is_empty()
                        && ticket.subtasks.is_empty()
                        && is_expanded;

                    let indicator = if has_subtasks {
                        if is_expanded { "▼ " } else { "▶ " }
                    } else {
                        "  "
                    };

                    let line = Line::from(vec![
                        Span::styled(indicator, Theme::accent()),
                        Span::styled(format!("{} ", ticket.issue_type.icon()), ticket.issue_type.style()),
                        Span::styled(ticket.key.clone(), Theme::ticket_key()),
                        Span::styled(" - ", Theme::dimmed()),
                        Span::styled(ticket.title.clone(), Theme::text()),
                    ]);
                    items.push(ListItem::new(line));

                    if is_expanded {
                        if is_loading {
                            items.push(ListItem::new(Line::from(vec![
                                Span::styled("    ⟳ loading subtasks…", Theme::dimmed()),
                            ])));
                        } else {
                            for subtask in &ticket.subtasks {
                                let line = Line::from(vec![
                                    Span::styled("    ↳ ", Theme::dimmed()),
                                    Span::styled(format!("{} ", subtask.issue_type.icon()), subtask.issue_type.style()),
                                    Span::styled(subtask.key.clone(), Theme::ticket_key()),
                                    Span::styled(" - ", Theme::dimmed()),
                                    Span::styled(subtask.title.clone(), Theme::text()),
                                ]);
                                items.push(ListItem::new(line));
                            }
                        }
                    }
                }

                let list = List::new(items)
                    .highlight_style(Theme::selected())
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
