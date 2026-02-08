use ratatui::Frame;

use crate::app::App;

pub mod components;
pub mod pages;
pub mod theme;
pub mod time_entry;

pub fn render(frame: &mut Frame, app: &App) {
    pages::home::render(frame, app);
}
