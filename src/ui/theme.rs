use ratatui::style::{Color, Modifier, Style};

/// Cyberpunk matrix theme — blue-violet rain, amber warmth,
/// green only for selection/focus. Easy on the eyes.
pub struct Theme;

// ── Rain — cool blue-violet cascade ──────────────────────────
const RAIN:           Color = Color::Rgb(0x44, 0x88, 0xcc); // the falling glyphs
const RAIN_FALLBACK:  (f32, f32, f32) = (68.0, 136.0, 204.0);

// ── Backgrounds — deep navy-black ────────────────────────────
const BG:             Color = Color::Rgb(0x08, 0x08, 0x12); // base terminal bg
const BG_PANEL:       Color = Color::Rgb(0x0c, 0x0c, 0x18); // panel surfaces
const BG_POPUP:       Color = Color::Rgb(0x10, 0x10, 0x1e); // popup surfaces
const BG_SELECT:      Color = Color::Rgb(0x14, 0x28, 0x14); // selected row — green tint

// ── Borders ──────────────────────────────────────────────────
const BORDER_DEFAULT: Color = Color::Rgb(0x28, 0x2c, 0x3a); // muted slate, blends in
const BORDER_FOCUS:   Color = Color::Rgb(0x33, 0xcc, 0x55); // vivid green — selected only

// ── Text — warm neutrals with amber highlights ───────────────
const TEXT:           Color = Color::Rgb(0xbb, 0xbb, 0xcc); // body text, soft lavender-grey
const TEXT_BRIGHT:    Color = Color::Rgb(0xe8, 0xe0, 0xd0); // warm white, titles
const TEXT_DIM:       Color = Color::Rgb(0x3c, 0x3c, 0x50); // placeholders, ghosts
const AMBER:          Color = Color::Rgb(0xe0, 0xa0, 0x40); // warm gold accent
const AMBER_DIM:      Color = Color::Rgb(0x88, 0x66, 0x30); // muted gold

// ── Greens — only for interactive focus ──────────────────────
const GREEN:          Color = Color::Rgb(0x33, 0xcc, 0x55); // focused borders, active buttons
const GREEN_DIM:      Color = Color::Rgb(0x22, 0x77, 0x33); // subtle green

// ── Status ───────────────────────────────────────────────────
const RED:            Color = Color::Rgb(0xdd, 0x44, 0x44);
const RED_DIM:        Color = Color::Rgb(0x88, 0x22, 0x22);
const WARN:           Color = Color::Rgb(0xdd, 0x99, 0x33);
const CYAN:           Color = Color::Rgb(0x55, 0xbb, 0xcc); // instruction keys

impl Theme {
    // ── Rain ──────────────────────────────────────────────────
    pub fn rain_color() -> Color { RAIN }
    pub fn rain_bg() -> Color { BG }

    // ── Backgrounds ───────────────────────────────────────────
    pub fn bg() -> Color { BG }
    pub fn panel_background() -> Color { BG_PANEL }
    pub fn popup_background() -> Color { BG_POPUP }

    // ── Borders ───────────────────────────────────────────────
    pub fn default_border_color() -> Color { BORDER_DEFAULT }
    pub fn focused_border_color() -> Color { BORDER_FOCUS }

    pub fn border() -> Style {
        Style::default().fg(BORDER_DEFAULT)
    }
    pub fn border_focused() -> Style {
        Style::default().fg(BORDER_FOCUS)
    }
    pub fn popup_border() -> Style {
        Style::default().fg(AMBER).add_modifier(Modifier::BOLD)
    }

    // ── Titles ────────────────────────────────────────────────
    pub fn title() -> Style {
        Style::default().fg(AMBER).add_modifier(Modifier::BOLD)
    }
    pub fn panel_title() -> Style {
        Style::default().fg(AMBER_DIM).bg(BG_PANEL).add_modifier(Modifier::BOLD)
    }
    pub fn popup_title() -> Style {
        Style::default().fg(TEXT_BRIGHT).add_modifier(Modifier::BOLD)
    }

    // ── Primary / accent ──────────────────────────────────────
    pub fn primary_color() -> Color { AMBER }
    pub fn accent() -> Style { Style::default().fg(AMBER) }

    // ── Text ──────────────────────────────────────────────────
    pub fn text() -> Style { Style::default().fg(TEXT) }
    pub fn text_bright() -> Style { Style::default().fg(TEXT_BRIGHT) }
    pub fn dimmed() -> Style { Style::default().fg(TEXT_DIM) }
    pub fn placeholder() -> Style { Style::default().fg(TEXT_DIM) }

    // ── Selection — green highlight ───────────────────────────
    pub fn selected() -> Style {
        Style::default()
            .fg(GREEN)
            .bg(BG_SELECT)
            .add_modifier(Modifier::BOLD)
    }

    // ── Buttons ───────────────────────────────────────────────
    pub fn button_active() -> Style {
        Style::default().fg(GREEN).add_modifier(Modifier::BOLD)
    }
    pub fn button_inactive() -> Style {
        Style::default().fg(TEXT_DIM)
    }

    // ── Domain ────────────────────────────────────────────────
    pub fn ticket_key() -> Style {
        Style::default().fg(AMBER).add_modifier(Modifier::BOLD)
    }
    pub fn instruction_key() -> Style {
        Style::default().fg(CYAN).add_modifier(Modifier::BOLD)
    }

    // ── Status ────────────────────────────────────────────────
    pub fn error() -> Style { Style::default().fg(RED) }
    pub fn error_dim() -> Style { Style::default().fg(RED_DIM) }
    pub fn warning() -> Style { Style::default().fg(WARN) }
    pub fn success() -> Style {
        Style::default().fg(GREEN).add_modifier(Modifier::BOLD)
    }

    // ── Input ─────────────────────────────────────────────────
    pub fn input_border() -> Style { Style::default().fg(AMBER_DIM) }
    pub fn input_cursor() -> Style {
        Style::default().fg(BG).bg(AMBER)
    }

    // ── Charts ────────────────────────────────────────────────
    pub fn chart_spent() -> Color { GREEN }
    pub fn chart_remaining() -> Color { BORDER_DEFAULT }
}
