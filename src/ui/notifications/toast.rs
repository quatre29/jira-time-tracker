use crate::ui::theme::Theme;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use ratatui::Frame;
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
pub enum ToastKind {
    Error,
    Warn,
    Success,
}

pub struct Toast {
    pub message: String,
    pub kind: ToastKind,
    created_at: Instant,
    duration: Duration,
}

impl Toast {
    pub fn error(msg: impl Into<String>) -> Self {
        Self::new(msg, ToastKind::Error)
    }

    pub fn warn(msg: impl Into<String>) -> Self {
        Self::new(msg, ToastKind::Warn)
    }

    pub fn success(msg: impl Into<String>) -> Self {
        Self::new(msg, ToastKind::Success)
    }

    fn new(msg: impl Into<String>, kind: ToastKind) -> Self {
        Self {
            message: msg.into(),
            kind,
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

        let (bg, border, text, filled, empty) = match self.kind {
            ToastKind::Error => (
                Theme::toast_error_bg(),
                Theme::toast_error_border(),
                Theme::toast_error_text(),
                Theme::toast_error_progress_filled(),
                Theme::toast_error_progress_empty(),
            ),
            ToastKind::Warn => (
                Theme::toast_warn_bg(),
                Theme::toast_warn_border(),
                Theme::toast_warn_text(),
                Theme::toast_warn_progress_filled(),
                Theme::toast_warn_progress_empty(),
            ),
            ToastKind::Success => (
                Theme::toast_success_bg(),
                Theme::toast_success_border(),
                Theme::toast_success_text(),
                Theme::toast_success_progress_filled(),
                Theme::toast_success_progress_empty(),
            ),
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border)
            .style(undim.bg(bg));

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
                .style(text)
                .wrap(Wrap { trim: false }),
            msg_area,
        );

        // Progress bar — bottom row of inner area
        if inner.height >= 2 {
            let filled_count = (self.progress() * inner.width as f64) as u16;
            let empty_count = inner.width.saturating_sub(filled_count);

            let bar = Line::from(vec![
                Span::styled("▓".repeat(filled_count as usize), filled),
                Span::styled(" ".repeat(empty_count as usize), empty),
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
