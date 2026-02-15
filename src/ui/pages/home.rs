use std::time::Duration;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use tui_piechart::{PieChart, PieSlice};

use crate::{
    app::App,
    ui::components::{Component, Header, TicketList},
};

pub fn render(frame: &mut Frame, app: &App, dt: Duration) {
    let area = frame.area();

    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(area);

    let body_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(outer_layout[1]);

    let body_info_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(body_layout[1]);

    frame.render_widget(
        Paragraph::new("Body 1").block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        ),
        body_layout[0],
    );

    frame.render_widget(
        Paragraph::new("Ticket Info").block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        ),
        body_info_layout[0],
    );

    frame.render_widget(
        Paragraph::new("User Info").block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        ),
        body_info_layout[1],
    );
    frame.render_widget(
        Paragraph::new("Footer").block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        ),
        outer_layout[2],
    );

    Header::new("Jira Time Tracker").render(app, frame, outer_layout[0], dt);
    TicketList::new().render(app, frame, body_layout[0], dt);

    let slices = vec![
        PieSlice::new("Spent", 45.0, Color::Red),
        PieSlice::new("Remaining", 55.0, Color::Blue),
    ];

    let pie_chart = PieChart::new(slices).high_resolution(true);

    frame.render_widget(pie_chart, body_info_layout[0]);
}
