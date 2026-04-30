use crate::app::{App, RenderContext};
use crate::{
    app::PopupState,
    ui::components::{Component, Footer},
};
use ratatui::Frame;
use std::time::Duration;

pub mod components;
pub mod matrix_rain;
pub mod pages;
pub mod theme;
pub mod time_entry;
pub mod notifications;

pub fn render(frame: &mut Frame, app: &mut App, dt: Duration) {
    let selected_ticket = app.selected_ticket_ref().cloned();

    let context = RenderContext {
        tickets_state: &app.tickets_state,
        user_state: &app.user_state,
        selected_idx: app.selected_idx,
        selected_ticket: selected_ticket.as_ref(),
        focused: &app.focused,
        tick: app.tick,
        expanded_keys: &app.expanded_keys,
    };

    pages::home::render(frame, &context, dt);

    if let PopupState::Active(ref mut popup) = app.popup {
        popup.render(frame, frame.area(), &context, dt);
    }

    // Render footer last so it always appears above the popup dim overlay
    let footer_area = pages::home::footer_area(frame.area());
    Footer::new().render(frame, footer_area, &context, dt);

    app.toast_manager.tick();
    app.toast_manager.render(frame, frame.area(), &context, dt)
}
