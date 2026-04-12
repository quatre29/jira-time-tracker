use crate::ui::theme::Theme;
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, Borders};
use tui_textarea::TextArea;

pub enum BorderState {
    Default,
    Selected,
    Error,
}

pub struct Input<'a> {
    pub textarea: TextArea<'a>,
    title: &'a str,
}

impl<'a> Input<'a> {
    pub fn new(title: &'a str, selected: bool) -> Self {
        let mut textarea = TextArea::default();

        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Theme::input_border())
                .title(title)
                .title_style(Theme::panel_title())
                .style(Style::default().bg(Theme::popup_background())),
        );

        textarea.set_style(Theme::text());
        textarea.set_cursor_line_style(Style::default());
        textarea.set_cursor_style(if selected {
            Theme::input_cursor()
        } else {
            Theme::input_cursor_inactive()
        });
        textarea.set_placeholder_style(Theme::placeholder());

        Self { textarea, title }
    }

    pub fn single_line(mut self) -> Self {
        todo!()
    }

    pub fn placeholder_text(mut self, text: &str) -> Self {
        self.textarea.set_placeholder_text(text);
        self
    }

    pub fn border_style(mut self, state: BorderState) -> Self {
        self.set_border_style(state);

        self
    }

    pub fn set_border_style(&mut self, state: BorderState) {
        self.textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(match state {
                    BorderState::Selected => Theme::border_focused(),
                    BorderState::Default => Theme::border_default(),
                    BorderState::Error => Theme::border_error(),
                })
                .title(self.title)
                .title_style(Theme::panel_title())
                .style(Style::default().bg(Theme::popup_background())),
        );
        self.textarea.set_cursor_style(match state {
            BorderState::Selected => Theme::input_cursor(),
            _ => Theme::input_cursor_inactive(),
        });
    }
}
