use crate::api::models::JiraTicket;
use crate::app::RenderContext;
use crate::ui::components::Component;
use crate::ui::theme::Theme;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Line, Span, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;
use std::time::Duration;

pub struct TicketInfo {
    title: String,
}

impl TicketInfo {
    pub fn new() -> Self {
        Self {
            title: " Ticket Info ".to_string(),
        }
    }

    fn render_ticket(&self, frame: &mut Frame, area: Rect, ticket: &JiraTicket) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Theme::border_default())
            .title(Span::styled(&self.title, Theme::panel_title()))
            .style(Style::default().bg(Theme::panel_background()));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Split inner area into sections
        let sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header: icon + key + title
                Constraint::Length(1),  // Separator
                Constraint::Length(2),  // Status + Priority row
                Constraint::Length(1),  // Separator
                Constraint::Length(3),  // Time tracking
                Constraint::Length(1),  // Separator
                Constraint::Min(0),    // Metadata (assignee, dates, labels)
            ])
            .split(inner);

        // ── Header: Type icon + Key + Title ───────────────────
        let header_lines = vec![
            Line::from(vec![
                Span::styled(format!("{} ", ticket.issue_type.icon()), ticket.issue_type.style()),
                Span::styled(&ticket.key, Theme::ticket_key()),
            ]),
            Line::from(Span::styled(&ticket.title, Theme::text_bright())),
        ];
        frame.render_widget(Paragraph::new(header_lines), sections[0]);

        // ── Separator ─────────────────────────────────────────
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "─".repeat(inner.width as usize),
                Theme::border_default(),
            ))),
            sections[1],
        );

        // ── Status + Priority ─────────────────────────────────
        let status_priority = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(sections[2]);

        let status_style = status_color(&ticket.status_category);
        let status_lines = vec![
            Line::from(vec![
                Span::styled("Status  ", Theme::dimmed()),
                Span::styled(&ticket.status, status_style),
            ]),
        ];
        frame.render_widget(Paragraph::new(status_lines), status_priority[0]);

        let priority_style = priority_color(&ticket.priority);
        let priority_lines = vec![
            Line::from(vec![
                Span::styled("Priority  ", Theme::dimmed()),
                Span::styled(&ticket.priority, priority_style),
            ]),
        ];
        frame.render_widget(Paragraph::new(priority_lines), status_priority[1]);

        // ── Separator ─────────────────────────────────────────
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "─".repeat(inner.width as usize),
                Theme::border_default(),
            ))),
            sections[3],
        );

        // ── Time Tracking ─────────────────────────────────────
        let time_section = sections[4];
        self.render_time_tracking(frame, time_section, ticket);

        // ── Separator ─────────────────────────────────────────
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "─".repeat(inner.width as usize),
                Theme::border_default(),
            ))),
            sections[5],
        );

        // ── Metadata ──────────────────────────────────────────
        let meta_section = sections[6];
        self.render_metadata(frame, meta_section, ticket);
    }

    fn render_time_tracking(&self, frame: &mut Frame, area: Rect, ticket: &JiraTicket) {
        let total = ticket.original_estimate_seconds;
        let spent = ticket.time_spent_seconds;

        if total == 0 && spent == 0 {
            let lines = vec![
                Line::from(Span::styled("No time tracked", Theme::dimmed())),
            ];
            frame.render_widget(Paragraph::new(lines), area);
            return;
        }

        let layout = Layout::vertical([
            Constraint::Length(1), // Labels
            Constraint::Length(1), // Progress bar
            Constraint::Length(1), // Remaining
        ])
        .split(area);

        // Time labels
        let spent_str = if ticket.time_spent.is_empty() { "0h".to_string() } else { ticket.time_spent.clone() };
        let estimate_str = if ticket.original_estimate.is_empty() { "—".to_string() } else { ticket.original_estimate.clone() };

        let time_label = Line::from(vec![
            Span::styled("Logged ", Theme::dimmed()),
            Span::styled(&spent_str, Theme::accent()),
            Span::styled(" / ", Theme::dimmed()),
            Span::styled(&estimate_str, Theme::text()),
        ]);
        frame.render_widget(Paragraph::new(vec![time_label]), layout[0]);

        // Progress bar with visible outline
        let bar_width = layout[1].width as usize;
        if bar_width >= 4 {
            let inner_width = bar_width.saturating_sub(2); // account for [ ]
            let ratio = if total > 0 {
                (spent as f64 / total as f64).min(1.0)
            } else {
                0.0
            };
            let filled = ((ratio * inner_width as f64).round() as usize).min(inner_width);
            let empty = inner_width.saturating_sub(filled);

            let bar_color = if ratio > 0.9 {
                ratatui::style::Color::Rgb(0xe5, 0x49, 0x3a) // red
            } else if ratio > 0.7 {
                ratatui::style::Color::Rgb(0xdd, 0x99, 0x33) // amber
            } else {
                Theme::chart_spent() // healthy
            };

            let track_color = ratatui::style::Color::Rgb(0x28, 0x2c, 0x3a); // border color for empty track

            let bar_line = Line::from(vec![
                Span::styled("│", Style::default().fg(track_color)),
                Span::styled("█".repeat(filled), Style::default().fg(bar_color)),
                Span::styled("░".repeat(empty), Style::default().fg(track_color)),
                Span::styled("│", Style::default().fg(track_color)),
            ]);
            frame.render_widget(Paragraph::new(vec![bar_line]), layout[1]);
        }

        // Remaining
        let remaining_str = if ticket.remaining_estimate.is_empty() { "—".to_string() } else { ticket.remaining_estimate.clone() };
        let remaining_line = Line::from(vec![
            Span::styled("Remaining ", Theme::dimmed()),
            Span::styled(&remaining_str, Theme::text()),
        ]);
        frame.render_widget(Paragraph::new(vec![remaining_line]), layout[2]);
    }

    fn render_metadata(&self, frame: &mut Frame, area: Rect, ticket: &JiraTicket) {
        let mut lines: Vec<Line> = Vec::new();

        // Assignee
        lines.push(Line::from(vec![
            Span::styled("Assignee   ", Theme::dimmed()),
            Span::styled(&ticket.assignee, Theme::text()),
        ]));

        // Reporter
        lines.push(Line::from(vec![
            Span::styled("Reporter   ", Theme::dimmed()),
            Span::styled(&ticket.reporter, Theme::text()),
        ]));

        // Created
        if !ticket.created.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("Created    ", Theme::dimmed()),
                Span::styled(&ticket.created, Theme::text()),
            ]));
        }

        // Updated
        if !ticket.updated.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("Updated    ", Theme::dimmed()),
                Span::styled(&ticket.updated, Theme::text()),
            ]));
        }

        // Labels
        if !ticket.labels.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("Labels     ", Theme::dimmed()),
                Span::styled(ticket.labels.join(", "), Theme::accent()),
            ]));
        }

        // Subtasks count
        if !ticket.subtask_keys.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("Subtasks   ", Theme::dimmed()),
                Span::styled(
                    format!("{}", ticket.subtask_keys.len()),
                    Theme::text(),
                ),
            ]));
        }

        frame.render_widget(Paragraph::new(lines), area);
    }
}

/// Returns a style for the status based on Jira's status category.
fn status_color(category_key: &str) -> Style {
    match category_key {
        "done" => Style::default().fg(ratatui::style::Color::Rgb(0x36, 0xb3, 0x7e)), // green
        "indeterminate" => Style::default().fg(ratatui::style::Color::Rgb(0x42, 0x8f, 0xdc)), // blue (in progress)
        "new" => Style::default().fg(ratatui::style::Color::Rgb(0x6b, 0x77, 0x8d)), // grey (to do)
        _ => Theme::text(),
    }
}

/// Returns a style for the priority name.
fn priority_color(priority: &str) -> Style {
    match priority.to_lowercase().as_str() {
        "highest" | "blocker" => Style::default().fg(ratatui::style::Color::Rgb(0xe5, 0x49, 0x3a)),
        "high" => Style::default().fg(ratatui::style::Color::Rgb(0xdd, 0x77, 0x33)),
        "medium" => Style::default().fg(ratatui::style::Color::Rgb(0xdd, 0x99, 0x33)),
        "low" => Style::default().fg(ratatui::style::Color::Rgb(0x42, 0x8f, 0xdc)),
        "lowest" => Style::default().fg(ratatui::style::Color::Rgb(0x6b, 0x77, 0x8d)),
        _ => Theme::text(),
    }
}

impl Component for TicketInfo {
    fn render(&mut self, frame: &mut Frame, area: Rect, context: &RenderContext, _dt: Duration) {
        match context.selected_ticket {
            Some(ticket) => self.render_ticket(frame, area, ticket),
            None => {
                // Empty state
                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Theme::border_default())
                    .title(Span::styled(&self.title, Theme::panel_title()))
                    .style(Style::default().bg(Theme::panel_background()));

                let inner = block.inner(area);
                frame.render_widget(block, area);

                frame.render_widget(
                    Paragraph::new(Line::from(Span::styled(
                        "Select a ticket to view details",
                        Theme::dimmed(),
                    ))),
                    inner,
                );
            }
        }
    }
}
