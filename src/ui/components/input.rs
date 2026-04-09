use ratatui::style::Style;
use tui_textarea::TextArea;

use crate::ui::theme::Theme;

pub struct Input<'a> {
    pub textarea: TextArea<'a>,
}

impl<'a> Input<'a> {
    pub fn new(title: &'a str) -> Self {
        let mut textarea = TextArea::default();

        textarea.set_block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .border_style(Theme::input_border())
                .title(title)
                .title_style(Theme::panel_title())
                .style(Style::default().bg(Theme::popup_background())),
        );
        textarea.set_style(Theme::text());
        textarea.set_cursor_line_style(Style::default());
        textarea.set_cursor_style(Theme::input_cursor());
        textarea.set_placeholder_style(Theme::placeholder());

        Self { textarea }
    }

    pub fn single_line(mut self) -> Self {
        todo!()
    }

    pub fn placeholder_text(mut self, text: &str) -> Self {
        self.textarea.set_placeholder_text(text);
        self
    }
}
