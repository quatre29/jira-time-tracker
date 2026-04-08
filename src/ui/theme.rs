use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone, Copy)]
pub struct ButtonTheme {
    pub text: Color,
    pub background: Color,
    pub highlight: Color,
    pub shadow: Color,
}

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

    pub fn button_blue() -> ButtonTheme {
        ButtonTheme {
            text: Color::Rgb(16, 24, 48),
            background: Color::Rgb(48, 72, 144),
            highlight: Color::Rgb(64, 96, 192),
            shadow: Color::Rgb(32, 48, 96),
        }
    }

    pub fn button_red() -> ButtonTheme {
        ButtonTheme {
            text: Color::Rgb(48, 16, 16),
            background: Color::Rgb(144, 48, 48),
            highlight: Color::Rgb(192, 64, 64),
            shadow: Color::Rgb(96, 32, 32),
        }
    }

    pub fn button_green() -> ButtonTheme {
        ButtonTheme {
            text: Color::Rgb(16, 48, 16),
            background: Color::Rgb(48, 144, 48),
            highlight: Color::Rgb(64, 192, 64),
            shadow: Color::Rgb(32, 96, 32),
        }
    }
}
