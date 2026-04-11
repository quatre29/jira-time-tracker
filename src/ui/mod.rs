use crate::{
    app::{App, PopupState},
    ui::components::Component,
};
use ratatui::Frame;
use std::time::Duration;

pub mod components;
pub mod matrix_rain;
pub mod pages;
pub mod theme;
pub mod time_entry;

pub fn render(frame: &mut Frame, app: &mut App, dt: Duration) {
    pages::home::render(frame, app, dt);

    match &mut app.popup {
        PopupState::Active(popup) => popup.render(app, frame, frame.area(), dt),
        PopupState::None => {}
    }
}
