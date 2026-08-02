use crate::app::RenderContext;
use crate::events::app_event::{ActionEvent, UiEvent};
use crate::ui::components::{Component, Input};
use crate::ui::theme::Theme;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Modifier;
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListItem, ListState};
use ratatui::Frame;
use std::time::Duration;

pub struct CategoryPopup<'a> {
    ticket_key: String,
    categories: Vec<String>,
    selected: usize,
    creating: bool,
    new_category_input: Input<'a>,
}

impl<'a> CategoryPopup<'a> {
    pub fn new(ticket_key: String, categories: Vec<String>) -> Self {
        Self {
            ticket_key,
            categories,
            selected: 0,
            creating: false,
            new_category_input: Input::new("New category name", true).placeholder_text("e.g. Work"),
        }
    }

    fn item_count(&self) -> usize {
        // Uncategorized + each category + "New Category"
        1 + self.categories.len() + 1
    }
}

impl<'a> Component for CategoryPopup<'a> {
    fn render(&mut self, frame: &mut Frame, area: Rect, _context: &RenderContext, _dt: Duration) {
        if self.creating {
            let [_, input_area, _] = Layout::vertical([
                Constraint::Fill(1),
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .areas(area);
            frame.render_widget(&self.new_category_input.textarea, input_area);
            return;
        }

        let mut items: Vec<ListItem> = vec![];

        // Uncategorized option
        let uncategorized_line = Line::from(vec![
            Span::styled("✕ ", Theme::dimmed()),
            Span::styled("Uncategorized", Theme::text()),
        ]);
        items.push(ListItem::new(uncategorized_line));

        // Existing categories
        for name in &self.categories {
            let line = Line::from(vec![
                Span::styled("  ", Theme::dimmed()),
                Span::styled(name.clone(), Theme::text()),
            ]);
            items.push(ListItem::new(line));
        }

        // New Category option
        let new_line = Line::from(vec![Span::styled(
            "+ New Category",
            Theme::accent().add_modifier(Modifier::ITALIC),
        )]);
        items.push(ListItem::new(new_line));

        let list = List::new(items).highlight_style(Theme::selected());
        let mut state = ListState::default();
        state.select(Some(self.selected));

        frame.render_stateful_widget(list, area, &mut state);
    }

    fn handle_key(&mut self, key: KeyEvent) -> Option<UiEvent> {
        if self.creating {
            match key.code {
                KeyCode::Enter => {
                    let name = self
                        .new_category_input
                        .textarea
                        .lines()
                        .first()
                        .cloned()
                        .unwrap_or_default()
                        .trim()
                        .to_string();

                    if name.is_empty() {
                        self.creating = false;
                        return None;
                    }

                    return Some(UiEvent::Action(ActionEvent::AssignToCategory {
                        ticket_key: self.ticket_key.clone(),
                        category_name: Some(name),
                    }));
                }
                KeyCode::Esc => {
                    self.creating = false;
                    return None;
                }
                _ => {
                    self.new_category_input.textarea.input(key);
                    return None;
                }
            }
        }

        match key.code {
            KeyCode::Up => {
                let count = self.item_count();
                self.selected = if self.selected == 0 { count - 1 } else { self.selected - 1 };
                None
            }
            KeyCode::Down => {
                self.selected = (self.selected + 1) % self.item_count();
                None
            }
            KeyCode::Enter => {
                if self.selected == 0 {
                    // Uncategorized
                    Some(UiEvent::Action(ActionEvent::AssignToCategory {
                        ticket_key: self.ticket_key.clone(),
                        category_name: None,
                    }))
                } else if self.selected <= self.categories.len() {
                    // Existing category
                    let name = self.categories[self.selected - 1].clone();
                    Some(UiEvent::Action(ActionEvent::AssignToCategory {
                        ticket_key: self.ticket_key.clone(),
                        category_name: Some(name),
                    }))
                } else {
                    // New Category
                    self.creating = true;
                    None
                }
            }
            _ => None,
        }
    }
}
