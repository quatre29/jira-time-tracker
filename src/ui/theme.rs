use ratatui::style::{Color, Modifier, Style};

pub struct Theme;

impl Theme {
    pub fn title() -> Style {
        Style::default()
            .fg(Color::Rgb(0xff, 0xb0, 0x00))
            .add_modifier(Modifier::BOLD)
    }

    pub fn border() -> Style {
        Style::default().fg(Color::White)
    }

    pub fn default_border_color() -> Color {
        Color::Rgb(0x44, 0x44, 0x66)
    }

    pub fn focused_border_color() -> Color {
        Color::Rgb(57, 255, 20)
    }

    pub fn primary_color() -> Color {
        Color::Rgb(0xff, 0xb0, 0x00)
    }

    pub fn panel_background() -> Color {
        Color::Rgb(0x0a, 0x0a, 0x10)
    }

    pub fn selected() -> Style {
        Style::default()
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD)
    }

    pub fn ticket_key() -> Style {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    }

    pub fn instruction_key() -> Style {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    }

    pub fn dimmed() -> Style {
        Style::default().fg(Color::DarkGray)
    }
}
