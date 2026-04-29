use ratatui::style::{Color, Modifier, Style};

// ── Matrix Rain ──────────────────────────────────────────────────
const RAIN: Color = Color::Rgb(0x44, 0x88, 0xcc);
const RAIN_FALLBACK: (f32, f32, f32) = (68.0, 136.0, 204.0);

// ── Backgrounds ──────────────────────────────────────────────────
const BG: Color = Color::Rgb(0x08, 0x08, 0x12);
const BG_PANEL: Color = Color::Rgb(0x0c, 0x0c, 0x18);
const BG_POPUP: Color = Color::Rgb(0x10, 0x10, 0x1e);
const BG_SELECT: Color = Color::Rgb(0x22, 0x18, 0x08);

// ── Borders ──────────────────────────────────────────────────
const BORDER_DEFAULT: Color = Color::Rgb(0x28, 0x2c, 0x3a);
const BORDER_FOCUS: Color = AMBER;
const BORDER_ERROR: Color = Color::Rgb(255, 85, 85);

// ── Text  ──────────────────────────────────────────────────
const TEXT: Color = Color::Rgb(0xbb, 0xbb, 0xcc);
const TEXT_BRIGHT: Color = Color::Rgb(0xe8, 0xe0, 0xd0);
const TEXT_DIM: Color = Color::Rgb(0x3c, 0x3c, 0x50);
const AMBER: Color = Color::Rgb(0xe0, 0xa0, 0x40);
const AMBER_DIM: Color = Color::Rgb(0x88, 0x66, 0x30);

// ── Greens ──────────────────────────────────────────────────
const GREEN: Color = Color::Rgb(0x33, 0xcc, 0x55);
const GREEN_DIM: Color = Color::Rgb(0x22, 0x77, 0x33);

// ── Status ───────────────────────────────────────────────────
const RED: Color = Color::Rgb(0xdd, 0x44, 0x44);
const RED_DIM: Color = Color::Rgb(0x88, 0x22, 0x22);
const WARN: Color = Color::Rgb(0xdd, 0x99, 0x33);
const CYAN: Color = Color::Rgb(0x55, 0xbb, 0xcc);


#[derive(Debug, Clone, Copy)]
pub struct ButtonTheme {
    pub text: Color,
    pub background: Color,
    pub selected_background: Color,
    pub highlight: Color,
    pub shadow: Color,
}

pub struct Theme;

impl Theme {
    pub fn rain_color() -> Color { RAIN }
    pub fn rain_bg() -> Color { BG }

    pub fn bg() -> Color { BG }
    pub fn panel_background() -> Color { BG_PANEL }
    pub fn popup_background() -> Color { BG_POPUP }

    pub fn border_default() -> Style {
        Style::default().fg(BORDER_DEFAULT)
    }
    pub fn border_focused() -> Style {
        Style::default().fg(BORDER_FOCUS)
    }
    pub fn border_error() -> Style {
        Style::default().fg(BORDER_ERROR)
    }
    pub fn popup_border() -> Style {
        Style::default().fg(AMBER).add_modifier(Modifier::BOLD)
    }

    pub fn title() -> Style {
        Style::default().fg(AMBER).add_modifier(Modifier::BOLD)
    }

    pub fn panel_title() -> Style {
        Style::default().fg(AMBER_DIM).bg(BG_PANEL).add_modifier(Modifier::BOLD)
    }
    pub fn popup_title() -> Style {
        Style::default().fg(TEXT_BRIGHT).add_modifier(Modifier::BOLD)
    }

    pub fn primary_color() -> Color { AMBER }
    pub fn accent() -> Style { Style::default().fg(AMBER) }

    pub fn text() -> Style { Style::default().fg(TEXT) }
    pub fn text_bright() -> Style { Style::default().fg(TEXT_BRIGHT) }
    pub fn dimmed() -> Style { Style::default().fg(TEXT_DIM) }
    pub fn placeholder() -> Style { Style::default().fg(TEXT_DIM) }

    pub fn selected() -> Style {
        Style::default()
            .fg(AMBER)
            .bg(BG_SELECT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn button_active() -> Style {
        Style::default().fg(CYAN).add_modifier(Modifier::BOLD)
    }
    pub fn button_inactive() -> Style {
        Style::default().fg(TEXT_DIM)
    }

    pub fn ticket_key() -> Style {
        Style::default().fg(AMBER).add_modifier(Modifier::BOLD)
    }

    pub fn instruction_key() -> Style {
        Style::default().fg(CYAN).add_modifier(Modifier::BOLD)
    }

    pub fn button_confirm() -> ButtonTheme {
        ButtonTheme {
            text: Color::Rgb(16, 24, 48),
            background: Color::Rgb(48, 72, 144),
            selected_background: Color::Rgb(80, 120, 230),
            highlight: Color::Rgb(64, 96, 192),
            shadow: Color::Rgb(32, 48, 96),
        }
    }

    pub fn button_cancel() -> ButtonTheme {
        ButtonTheme {
            text: TEXT_BRIGHT,
            background: BG_PANEL,
            selected_background: Color::Rgb(0x30, 0x34, 0x4e),
            highlight: BORDER_DEFAULT,
            shadow: BG,
        }
    }

    pub fn button_danger() -> ButtonTheme {
        ButtonTheme {
            text: TEXT_BRIGHT,
            background: RED,
            selected_background: Color::Rgb(0xff, 0x55, 0x55),
            highlight: Color::Rgb(0xff, 0x66, 0x66),
            shadow: RED_DIM,
        }
    }

    pub fn button_primary() -> ButtonTheme {
        ButtonTheme {
            text: TEXT_BRIGHT,
            background: AMBER,
            selected_background: Color::Rgb(0xff, 0xc0, 0x60),
            highlight: Color::Rgb(0xff, 0xc0, 0x60),
            shadow: AMBER_DIM,
        }
    }

    // TOAST ERROR
    pub fn toast_error_bg() -> Color { Color::Rgb(0x1a, 0x0e, 0x0e) }
    pub fn toast_error_border() -> Style {
        Style::default().fg(RED_DIM).remove_modifier(Modifier::DIM)
    }
    pub fn toast_error_text() -> Style {
        Style::default().fg(TEXT_BRIGHT).remove_modifier(Modifier::DIM)
    }
    pub fn toast_error_progress_filled() -> Style {
        Style::default().fg(RED).bg(Color::Rgb(0x1a, 0x0e, 0x0e)).remove_modifier(Modifier::DIM)
    }
    pub fn toast_error_progress_empty() -> Style {
        Style::default().bg(Color::Rgb(0x1a, 0x0e, 0x0e)).remove_modifier(Modifier::DIM)
    }

    // TOAST WARN
    pub fn toast_warn_bg() -> Color { Color::Rgb(0x18, 0x12, 0x06) }
    pub fn toast_warn_border() -> Style {
        Style::default().fg(Color::Rgb(0x88, 0x55, 0x22)).remove_modifier(Modifier::DIM)
    }
    pub fn toast_warn_text() -> Style {
        Style::default().fg(TEXT_BRIGHT).remove_modifier(Modifier::DIM)
    }
    pub fn toast_warn_progress_filled() -> Style {
        Style::default().fg(WARN).bg(Color::Rgb(0x18, 0x12, 0x06)).remove_modifier(Modifier::DIM)
    }
    pub fn toast_warn_progress_empty() -> Style {
        Style::default().bg(Color::Rgb(0x18, 0x12, 0x06)).remove_modifier(Modifier::DIM)
    }

    // TOAST SUCCESS
    pub fn toast_success_bg() -> Color { Color::Rgb(0x0a, 0x16, 0x0c) }
    pub fn toast_success_border() -> Style {
        Style::default().fg(GREEN_DIM).remove_modifier(Modifier::DIM)
    }
    pub fn toast_success_text() -> Style {
        Style::default().fg(TEXT_BRIGHT).remove_modifier(Modifier::DIM)
    }
    pub fn toast_success_progress_filled() -> Style {
        Style::default().fg(GREEN).bg(Color::Rgb(0x0a, 0x16, 0x0c)).remove_modifier(Modifier::DIM)
    }
    pub fn toast_success_progress_empty() -> Style {
        Style::default().bg(Color::Rgb(0x0a, 0x16, 0x0c)).remove_modifier(Modifier::DIM)
    }

    pub fn error() -> Style { Style::default().fg(RED) }
    pub fn error_dim() -> Style { Style::default().fg(RED_DIM) }
    pub fn warning() -> Style { Style::default().fg(WARN) }
    pub fn success() -> Style {
        Style::default().fg(CYAN).add_modifier(Modifier::BOLD)
    }

    pub fn footer_key() -> Style {
        Style::default()
            .fg(AMBER)
            .bg(Color::Rgb(0x28, 0x2c, 0x3a))
            .add_modifier(Modifier::BOLD)
    }
    pub fn footer_desc() -> Style {
        Style::default().fg(TEXT)
    }
    pub fn footer_separator() -> Style {
        Style::default().fg(TEXT_DIM)
    }

    pub fn input_border() -> Style { Style::default().fg(AMBER_DIM) }
    pub fn input_cursor() -> Style {
        Style::default().fg(BG).bg(AMBER)
    }
    pub fn input_cursor_inactive() -> Style {
        Style::default().fg(BG).bg(TEXT_DIM)
    }

    pub fn chart_spent() -> Color { AMBER }
    pub fn chart_remaining() -> Color { CYAN }
}
