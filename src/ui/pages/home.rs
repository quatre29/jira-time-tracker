use std::time::Duration;

use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use tui_piechart::{PieChart, PieSlice, border_style};

use crate::{
    app::App,
    ui::{
        components::{Component, Header, TicketList},
        matrix_rain,
        theme::Theme,
    },
};

pub fn render(frame: &mut Frame, app: &App, dt: Duration) {
    let area = frame.area();

    frame.render_widget(
        Block::default().style(Style::default().bg(Color::Rgb(0x06, 0x06, 0x0a))),
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
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(70),
            Constraint::Percentage(20),
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

    let matrix_rain_ocluders = [
        body_layout[0],
        body_info_layout[0],
        body_info_layout[1],
        outer_layout[2],
        title_horizontal_layout[1],
    ];

    matrix_rain::render_matrix_rain(frame, app.tick, area, &matrix_rain_ocluders, 1, 0.3, 24);

    frame.render_widget(
        Paragraph::new("Body 1").block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(
                    Style::default()
                        .fg(Theme::default_border_color())
                        .add_modifier(Modifier::BOLD),
                ),
        ),
        body_layout[0],
    );

    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(Theme::default_border_color()))
            .title(Span::styled(
                "Ticket Info",
                Style::default()
                    .fg(Theme::primary_color())
                    .bg(Theme::panel_background())
                    .add_modifier(Modifier::BOLD),
            ))
            .style(Style::default().bg(Theme::panel_background())),
        body_info_layout[0],
    );

    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(Theme::default_border_color()))
            .title(Span::styled(
                "User Info",
                Style::default()
                    .fg(Theme::primary_color())
                    .bg(Theme::panel_background())
                    .add_modifier(Modifier::BOLD),
            ))
            .style(Style::default().bg(Theme::panel_background())),
        body_info_layout[1],
    );

    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(Theme::default_border_color()))
            .title(Span::styled(
                "Footer",
                Style::default()
                    .fg(Theme::primary_color())
                    .bg(Theme::panel_background())
                    .add_modifier(Modifier::BOLD),
            ))
            .style(Style::default().bg(Theme::panel_background())),
        outer_layout[2],
    );

    Header::new("Jira Time Tracker").render(app, frame, title_horizontal_layout[1], dt);
    TicketList::new().render(app, frame, body_layout[0], dt);

    let slices = vec![
        PieSlice::new("Spent", 45.0, Color::Green),
        PieSlice::new("Remaining", 55.0, Color::White),
    ];

    let pie_chart = PieChart::new(slices).high_resolution(true);

    frame.render_widget(pie_chart, body_info_layout[0]);
}
