use crate::app::RenderContext;
use crate::ui::components::ticket_info::TicketInfo;
use crate::ui::components::user_info::UserInfo;
use crate::ui::{
    components::{Component, Header, TicketList},
    matrix_rain,
    theme::Theme,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::Block,
    Frame,
};
use std::time::Duration;

pub fn footer_area(area: Rect) -> Rect {
    let horizontal = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Percentage(70),
        Constraint::Fill(1),
    ])
        .split(area);

    Layout::vertical([
        Constraint::Percentage(10),
        Constraint::Fill(1),
        Constraint::Length(5),
        Constraint::Length(2),
    ])
    .split(horizontal[1])[2]
}

pub fn render(frame: &mut Frame, context: &RenderContext, dt: Duration) {
    let area = frame.area();

    frame.render_widget(
        Block::default().style(Style::default().bg(Theme::bg())),
        area,
    );

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Percentage(70),
            Constraint::Fill(1),
        ])
        .split(area);

    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Fill(1),
            Constraint::Length(5),
            Constraint::Length(2),
        ])
        .split(horizontal[1]);

    let body_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(outer_layout[1]);

    let body_info_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(body_layout[1]);

    let title_vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(outer_layout[0]);

    let title_horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Percentage(30),
            Constraint::Fill(1),
        ])
        .split(title_vertical_layout[1]);

    let matrix_rain_occluders = [
        body_layout[0],
        body_info_layout[0],
        body_info_layout[1],
        outer_layout[2],
        title_horizontal_layout[1],
    ];

    matrix_rain::render_matrix_rain(frame, context.tick, area, &matrix_rain_occluders, 1, 0.3, 24);

    Header::new("Jira Time Tracker").render(frame, title_horizontal_layout[1], context, dt);
    TicketList::new().render(frame, body_layout[0], context, dt);
    UserInfo::new().render(frame, body_info_layout[1], context, dt);
    TicketInfo::new().render(frame, body_info_layout[0], context, dt);
}
