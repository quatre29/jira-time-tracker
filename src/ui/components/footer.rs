use crate::app::RenderContext;
use crate::ui::components::component::ComponentName;
use crate::ui::components::Component;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use std::time::Duration;

pub struct Footer;

impl Footer {
    pub fn new() -> Self {
        Footer
    }

    fn hints(focused: &ComponentName) -> &'static [(&'static str, &'static str)] {
        match focused {
            ComponentName::TicketList => &[
                ("↑ ↓", "Navigate"),
                ("Enter", "Log Time"),
                ("t", "Add Ticket"),
                ("d", "Delete"),
                ("q", "Quit"),
            ],
            ComponentName::TimeInputPopup => &[
                ("Tab / ↓", "Next"),
                ("Shift+Tab / ↑", "Prev"),
                ("Enter", "Confirm"),
                ("Esc", "Close"),
            ],
            ComponentName::TicketInputPopup => &[
                ("Enter", "Add Ticket"),
                ("Esc", "Close"),
            ],
            ComponentName::ConfirmationPopup => &[
                ("← →", "Switch"),
                ("Enter", "Confirm"),
                ("Esc", "Close"),
            ],
            _ => &[],
        }
    }
}

impl Component for Footer {
    fn render(&mut self, frame: &mut Frame, area: Rect, context: &RenderContext, _dt: Duration) {
        let undim = Style::default().remove_modifier(Modifier::DIM);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Theme::border_default().patch(undim))
            .style(Style::default().bg(Theme::panel_background()).patch(undim));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let hints = Self::hints(context.focused);
        if hints.is_empty() || inner.height == 0 {
            return;
        }

        let mut spans: Vec<Span> = Vec::new();
        for (i, (key, desc)) in hints.iter().enumerate() {
            spans.push(Span::styled(format!(" {} ", key), Theme::footer_key().patch(undim)));
            spans.push(Span::raw(" "));
            spans.push(Span::styled(*desc, Theme::footer_desc().patch(undim)));
            if i + 1 < hints.len() {
                spans.push(Span::styled("   │   ", Theme::footer_separator().patch(undim)));
            }
        }

        let line = Line::from(spans);
        let para = Paragraph::new(line).alignment(Alignment::Center);

        let y = inner.y + inner.height.saturating_sub(1) / 2;
        frame.render_widget(para, Rect { y, height: 1, ..inner });
    }
}
