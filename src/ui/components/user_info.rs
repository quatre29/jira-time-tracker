use crate::app::{LoadState, RenderContext};
use crate::ui::components::Component;
use crate::ui::theme::Theme;
use chrono::{Datelike, NaiveDate};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Line, Span, Style};
use ratatui::style::Modifier;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Row, Table, Cell};
use ratatui::Frame;
use std::time::Duration;

pub struct UserInfo {
    title: String,
}

impl UserInfo {
    pub fn new() -> Self {
        Self {
            title: " User Info ".to_string(),
        }
    }

    fn render_loaded(
        &self,
        frame: &mut Frame,
        inner: Rect,
        context: &RenderContext,
        user: &crate::api::models::JiraUser,
    ) {
        let sections = Layout::vertical([
            Constraint::Length(3),  // User details
            Constraint::Length(1),  // Separator
            Constraint::Length(4),  // Burnout meter
            Constraint::Length(1),  // Separator
            Constraint::Min(3),    // Weekly time table
        ])
        .split(inner);

        // ── User details ──────────────────────────────────────
        let user_lines = vec![
            Line::from(vec![
                Span::styled("Name      ", Theme::dimmed()),
                Span::styled(&user.display_name, Theme::text()),
            ]),
            Line::from(vec![
                Span::styled("Email     ", Theme::dimmed()),
                Span::styled(&user.email_address, Theme::text()),
            ]),
            Line::from(vec![
                Span::styled("Timezone  ", Theme::dimmed()),
                Span::styled(&user.time_zone, Theme::text()),
            ]),
        ];
        frame.render_widget(Paragraph::new(user_lines), sections[0]);

        // ── Separator ─────────────────────────────────────────
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "─".repeat(inner.width as usize),
                Theme::border_default(),
            ))),
            sections[1],
        );

        // ── Burnout Meter ─────────────────────────────────────
        self.render_burnout_meter(frame, sections[2], context);

        // ── Separator ─────────────────────────────────────────
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "─".repeat(inner.width as usize),
                Theme::border_default(),
            ))),
            sections[3],
        );

        // ── Weekly Time Log ───────────────────────────────────
        self.render_weekly_time(frame, sections[4], context);
    }

    fn render_burnout_meter(&self, frame: &mut Frame, area: Rect, context: &RenderContext) {
        let in_progress_count = match context.user_stats {
            LoadState::Loaded(stats) => stats.in_progress_count as usize,
            LoadState::Loading => {
                frame.render_widget(
                    Paragraph::new(Line::from(Span::styled("⟳ Loading...", Theme::dimmed()))),
                    area,
                );
                return;
            }
            _ => 0,
        };

        // Burnout scale: 0-2 = chill, 3-4 = busy, 5-6 = stressed, 7+ = burnout
        let (level, label, color) = burnout_level(in_progress_count);

        let layout = Layout::vertical([
            Constraint::Length(1), // Title
            Constraint::Length(1), // Bar
            Constraint::Length(1), // Label
            Constraint::Length(1), // Count
        ])
        .split(area);

        // Title
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled("Burnout Meter", Theme::dimmed()))),
            layout[0],
        );

        // Bar
        let bar_width = layout[1].width as usize;
        if bar_width >= 4 {
            let inner_width = bar_width.saturating_sub(2);
            let max_level = 5;
            let filled = ((level as f64 / max_level as f64) * inner_width as f64)
                .round() as usize;
            let filled = filled.min(inner_width);
            let empty = inner_width.saturating_sub(filled);

            let track_color = ratatui::style::Color::Rgb(0x28, 0x2c, 0x3a);

            // Use fire characters for high burnout
            let fill_char = if level >= 4 { "🔥" } else { "█" };
            let fill_count = if level >= 4 { filled / 2 } else { filled }; // emoji is 2 wide

            let bar_line = if level >= 4 {
                let emoji_width = fill_count * 2;
                let remaining_empty = inner_width.saturating_sub(emoji_width);
                Line::from(vec![
                    Span::styled("│", Style::default().fg(track_color)),
                    Span::styled(fill_char.repeat(fill_count), Style::default().fg(color)),
                    Span::styled("░".repeat(remaining_empty), Style::default().fg(track_color)),
                    Span::styled("│", Style::default().fg(track_color)),
                ])
            } else {
                Line::from(vec![
                    Span::styled("│", Style::default().fg(track_color)),
                    Span::styled("█".repeat(filled), Style::default().fg(color)),
                    Span::styled("░".repeat(empty), Style::default().fg(track_color)),
                    Span::styled("│", Style::default().fg(track_color)),
                ])
            };
            frame.render_widget(Paragraph::new(vec![bar_line]), layout[1]);
        }

        // Label
        let label_line = Line::from(vec![
            Span::styled(label, Style::default().fg(color).add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("  ({} in progress)", in_progress_count),
                Theme::dimmed(),
            ),
        ]);
        frame.render_widget(Paragraph::new(vec![label_line]), layout[2]);
    }

    fn render_weekly_time(&self, frame: &mut Frame, area: Rect, context: &RenderContext) {
        let worklogs = match context.user_stats {
            LoadState::Loaded(stats) => &stats.weekly_worklogs,
            LoadState::Loading => {
                frame.render_widget(
                    Paragraph::new(Line::from(Span::styled("⟳ Loading worklogs...", Theme::dimmed()))),
                    area,
                );
                return;
            }
            _ => {
                frame.render_widget(
                    Paragraph::new(Line::from(Span::styled("No data", Theme::dimmed()))),
                    area,
                );
                return;
            }
        };

        if worklogs.is_empty() {
            frame.render_widget(
                Paragraph::new(Line::from(Span::styled("No time logged this week", Theme::dimmed()))),
                area,
            );
            return;
        }

        // Aggregate time by day of week (Mon=0 .. Fri=4)
        let mut day_seconds: [u64; 5] = [0; 5];
        let mut total_seconds: u64 = 0;

        for entry in worklogs {
            if let Some(day_idx) = parse_weekday_index(&entry.started) {
                if day_idx < 5 {
                    day_seconds[day_idx] += entry.time_spent_seconds;
                }
            }
            total_seconds += entry.time_spent_seconds;
        }

        let days = ["Mon", "Tue", "Wed", "Thu", "Fri"];
        let today_idx = chrono::Local::now().date_naive().weekday().num_days_from_monday() as usize;

        // Build table rows
        let mut rows: Vec<Row> = Vec::new();

        for (i, &day) in days.iter().enumerate() {
            let secs = day_seconds[i];
            let time_str = if secs > 0 { format_seconds(secs) } else { "—".to_string() };

            let is_today = i == today_idx;

            let day_style = if is_today {
                Style::default()
                    .fg(ratatui::style::Color::Rgb(0xe0, 0xa0, 0x40))
                    .add_modifier(Modifier::BOLD)
            } else {
                Theme::text()
            };

            let time_style = if secs == 0 {
                Theme::dimmed()
            } else if is_today {
                Style::default()
                    .fg(ratatui::style::Color::Rgb(0xe0, 0xa0, 0x40))
                    .add_modifier(Modifier::BOLD)
            } else {
                Theme::accent()
            };

            // Visual bar for the day
            let bar = if secs > 0 {
                let max_bar = 8; // max chars for the bar
                let hours = secs as f64 / 3600.0;
                let bar_len = ((hours / 8.0) * max_bar as f64).ceil() as usize; // 8h = full bar
                let bar_len = bar_len.min(max_bar).max(1);
                "█".repeat(bar_len)
            } else {
                String::new()
            };

            let bar_color = if secs >= 8 * 3600 {
                ratatui::style::Color::Rgb(0x36, 0xb3, 0x7e) // green — full day
            } else if secs >= 4 * 3600 {
                ratatui::style::Color::Rgb(0x42, 0x8f, 0xdc) // blue — half day+
            } else {
                ratatui::style::Color::Rgb(0x28, 0x2c, 0x3a) // dim — under half
            };

            rows.push(Row::new(vec![
                Cell::from(Span::styled(day, day_style)),
                Cell::from(Span::styled(time_str, time_style)),
                Cell::from(Span::styled(bar, Style::default().fg(bar_color))),
            ]));
        }

        // Total row
        rows.push(Row::new(vec![
            Cell::from(Span::styled(
                "Total",
                Style::default()
                    .fg(ratatui::style::Color::Rgb(0xe8, 0xe0, 0xd0))
                    .add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                format_seconds(total_seconds),
                Style::default()
                    .fg(ratatui::style::Color::Rgb(0xe0, 0xa0, 0x40))
                    .add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled("", Style::default())),
        ]));

        let header = Row::new(vec![
            Cell::from(Span::styled("Day", Theme::dimmed())),
            Cell::from(Span::styled("Logged", Theme::dimmed())),
            Cell::from(Span::styled("", Theme::dimmed())),
        ]);

        let widths = [
            Constraint::Length(5),
            Constraint::Length(8),
            Constraint::Fill(1),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .column_spacing(1);

        frame.render_widget(table, area);
    }
}

/// Returns (level 0-5, label, color) based on in-progress ticket count.
fn burnout_level(in_progress: usize) -> (u8, &'static str, ratatui::style::Color) {
    match in_progress {
        0..=2 => (1, "Chill", ratatui::style::Color::Rgb(0x36, 0xb3, 0x7e)),       // green
        3..=4 => (2, "Busy", ratatui::style::Color::Rgb(0x42, 0x8f, 0xdc)),        // blue
        5..=6 => (3, "Stressed", ratatui::style::Color::Rgb(0xdd, 0x99, 0x33)),    // amber
        7..=8 => (4, "Overloaded", ratatui::style::Color::Rgb(0xdd, 0x55, 0x33)),  // orange
        _ =>     (5, "🔥 BURNOUT", ratatui::style::Color::Rgb(0xe5, 0x49, 0x3a)),  // red
    }
}

/// Format seconds to a readable time string.
fn format_seconds(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    match (hours, minutes) {
        (0, 0) => "0m".to_string(),
        (0, m) => format!("{}m", m),
        (h, 0) => format!("{}h", h),
        (h, m) => format!("{}h {}m", h, m),
    }
}

/// Parse a Jira datetime string (e.g. "2025-04-28T10:30:00.000+0000") to weekday index (Mon=0..Sun=6).
fn parse_weekday_index(started: &str) -> Option<usize> {
    if started.len() < 10 {
        return None;
    }
    let date = NaiveDate::parse_from_str(&started[..10], "%Y-%m-%d").ok()?;
    Some(date.weekday().num_days_from_monday() as usize)
}

impl Component for UserInfo {
    fn render(&mut self, frame: &mut Frame, area: Rect, context: &RenderContext, _dt: Duration) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Theme::border_default())
            .title(Span::styled(&self.title, Theme::panel_title()))
            .style(Style::default().bg(Theme::panel_background()));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        match &context.user_state {
            LoadState::Loading => {
                frame.render_widget(
                    Paragraph::new(Line::from(Span::styled(
                        "⟳ Loading user...",
                        Theme::dimmed(),
                    ))),
                    inner,
                );
            }
            LoadState::Loaded(user) => {
                self.render_loaded(frame, inner, context, user);
            }
            LoadState::Error(err) => {
                let lines = vec![
                    Line::from(Span::styled("Error", Theme::error())),
                    Line::from(Span::styled(err.clone(), Theme::error_dim())),
                ];
                frame.render_widget(Paragraph::new(lines), inner);
            }
        }
    }
}
