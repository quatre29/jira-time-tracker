use crate::app::{App, RenderContext};
use crate::{
    app::PopupState,
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
    let context = RenderContext {
        tickets_state: &app.tickets_state,
        user_state: &app.user_state,
        selected_idx: app.selected_idx,
        focused: &app.focused,
        tick: app.tick,
    };

    pages::home::render(frame, &context, dt);


    if let PopupState::Active(ref mut popup) = app.popup {
        popup.render(frame, frame.area(), &context, dt);
    }
}
