use std::time::Duration;
use ratatui::Frame;
use crate::{
    app::{App, PopupState},
    ui::components::Component,
};

pub mod components;
pub mod matrix_rain;
pub mod pages;
pub mod theme;
pub mod time_entry;

pub fn render(frame: &mut Frame, app: &App, dt: Duration) {
    pages::home::render(frame, app, dt);

    match &app.popup {
        PopupState::Active(popup) => popup.render(app, frame, frame.area(), dt),
        PopupState::None => {}
    }
}
