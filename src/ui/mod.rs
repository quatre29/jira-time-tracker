use ratatui::Frame;

use crate::{
    app::{App, PopupState},
    ui::components::Component,
};

pub mod components;
pub mod pages;
pub mod theme;
pub mod time_entry;

pub fn render(frame: &mut Frame, app: &App) {
    pages::home::render(frame, app);

    match &app.popup {
        PopupState::InputTime(dialog) => dialog.render(app, frame, frame.area()),
        PopupState::None => {}
    }
}
