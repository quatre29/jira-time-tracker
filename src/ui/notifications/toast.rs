use crate::ui::theme::Theme;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use ratatui::Frame;
use std::time::{Duration, Instant};

pub struct Toast {
    pub message: String,
    created_at: Instant,
    duration: Duration,
}

impl Toast {
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
            created_at: Instant::now(),
            duration: Duration::from_millis(4000),
        }
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() >= self.duration
    }

    /// Returns the total height this toast needs for a given outer width (borders included).
    pub fn required_height(&self, outer_width: u16) -> u16 {
        let inner_width = outer_width.saturating_sub(2).max(1) as usize;
        let msg_lines = self.message.len().div_ceil(inner_width);
        let msg_lines = msg_lines.max(1) as u16;
        1 + msg_lines + 1 + 1 // top border + message + progress bar + bottom border
    }

    pub fn progress(&self) -> f64 {
        let elapsed = self.created_at.elapsed().as_secs_f64();
        let total = self.duration.as_secs_f64();
        1.0 - (elapsed / total).clamp(0.0, 1.0)
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let undim = Style::default().remove_modifier(Modifier::DIM);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Theme::toast_border())
            .style(undim.bg(Theme::toast_bg()));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if inner.height == 0 {
            return;
        }

        // Message — all inner rows except the last (reserved for progress bar)
        let msg_lines = inner.height.saturating_sub(1).max(1);
        let msg_area = Rect { height: msg_lines, ..inner };
        frame.render_widget(
            Paragraph::new(self.message.as_str())
                .style(Theme::toast_text())
                .wrap(Wrap { trim: false }),
            msg_area,
        );

        // Progress bar — bottom row of inner area
        if inner.height >= 2 {
            let filled = (self.progress() * inner.width as f64) as u16;
            let empty = inner.width.saturating_sub(filled);

            let bar = Line::from(vec![
                Span::styled("▓".repeat(filled as usize), Theme::toast_progress_filled()),
                Span::styled(" ".repeat(empty as usize), Theme::toast_progress_empty()),
            ]);

            let bar_area = Rect {
                y: inner.y + inner.height - 1,
                height: 1,
                ..inner
            };
            frame.render_widget(Paragraph::new(bar), bar_area);
        }
    }
}
