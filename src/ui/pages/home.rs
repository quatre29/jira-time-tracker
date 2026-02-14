use std::time::Duration;

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::Widget,
};

use crate::{
    app::App,
    ui::components::{Component, Header, TicketList},
};

pub fn render(frame: &mut Frame, app: &App, dt: Duration) {
    let area = frame.area();

    let vertical_layout = Layout::vertical([
        Constraint::Length(3), // Title
        // TODO: split body in vertically in half
        Constraint::Min(0), // Body
                            // TODO: add footer area
                            // Constraint::Length(1), // Footer
    ]);

    let [title_area, body_area] = vertical_layout.areas(area);

    Header::new("Jira Time Tracker").render(app, frame, title_area, dt);
    TicketList::new().render(app, frame, body_area, dt);
}
