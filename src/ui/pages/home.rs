use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::Widget,
};

use crate::{
    app::App,
    ui::components::{Component, Header, TicketList},
};

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let vertical_layout = Layout::vertical([
        Constraint::Length(3), // Title
        Constraint::Min(0),    // Body
                               // Constraint::Length(1), // Footer
    ]);

    let [title_area, body_area] = vertical_layout.areas(area);

    Header::new("Jira Time Tracker").render(frame, title_area);
    TicketList::new(&app.tickets)
        .selected(Some(0))
        .render(frame, body_area);
}
