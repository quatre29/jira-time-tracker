use ratatui::style::Style;
use tui_textarea::TextArea;

pub struct Input<'a> {
    pub textarea: TextArea<'a>,
}

impl<'a> Input<'a> {
    pub fn new(title: &'a str) -> Self {
        let mut textarea = TextArea::default();

        textarea.set_block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
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
}
