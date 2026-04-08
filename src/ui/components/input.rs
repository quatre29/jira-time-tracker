use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, Borders};
use tui_textarea::TextArea;
use crate::ui::theme::Theme;

pub struct Input<'a> {
    pub textarea: TextArea<'a>,
}

impl<'a> Input<'a> {
    pub fn new(title: &'a str, selected: bool) -> Self {
        let mut textarea = TextArea::default();

        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title(title),
        );

        Self { textarea }
    }

    pub fn single_line(mut self) -> Self {
        todo!()
    }

    pub fn placeholder_text(mut self, text: &str) -> Self {
        self.textarea.set_placeholder_text(text);
        self.textarea.set_cursor_line_style(Style::default());
        self
    }

    pub fn border_style(mut self, selected: bool) -> Self {
        self.set_border_style(selected);

        self
    }

    pub fn set_border_style(&mut self, selected: bool) {
        self.textarea.set_block(Block::default().borders(Borders::ALL).border_type(BorderType::Plain).border_style(
            match selected {
                true => Style::default().fg(Theme::focused_border_color()),
                false => Style::default().fg(Theme::default_border_color()),
            }
        ));
    }
}
