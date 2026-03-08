use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::Paragraph,
};

/// Matrix rain background — falling columns of characters.
/// `occluder`: the card rect where rain should render much fainter.
pub fn render_matrix_rain(
    frame: &mut Frame,
    tick: u64,
    area: Rect,
    // TODO:  Add correct occluders on the app
    // occluder: Rect,
    col_step: u64,
    rain_speed: f32,
    rain_trail: i64,
) {
    render_matrix_rain_colored(
        frame,
        tick,
        area,
        // occluder,
        col_step,
        Color::Rgb(0x00, 0xff, 0x00),
        rain_speed,
        rain_trail,
    );
}

#[allow(clippy::too_many_arguments)]
fn render_matrix_rain_colored(
    frame: &mut Frame,
    tick: u64,
    area: Rect,
    // occluder: Rect,
    col_step: u64,
    color: Color,
    rain_speed: f32,
    rain_trail: i64,
) {
    const CHARS: &[char] = &[
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ',
        'ク', 'ケ', 'コ', '░', '▒', '▓', '╌', '╎', '┊', '┆',
    ];
    let h = area.height as u64;
    let w = area.width as u64;
    if h == 0 || w == 0 {
        return;
    }

    let hash = |a: u64, b: u64| -> u64 {
        let mut v = a.wrapping_mul(6364136223846793005).wrapping_add(b);
        v ^= v >> 33;
        v = v.wrapping_mul(0xff51afd7ed558ccd);
        v ^= v >> 33;
        v
    };

    let trail_len: i64 = rain_trail;

    let step = col_step.max(1);
    let mut col = 0u64;
    while col < w {
        let seed = hash(col, 7919);
        let base_speed = 1 + (seed % 3);
        let speed = (((base_speed as f32) * rain_speed) as u64).max(1);
        let phase = seed % (h * 2);
        let head = ((tick * speed + phase) % (h * 2)) as i64;

        for row in 0..h as i64 {
            let dist = head - row;
            if dist < 0 || dist > trail_len {
                continue;
            }
            let ch_seed = hash(col * 1000 + (row as u64), tick / 2);
            let ch = CHARS[(ch_seed % (CHARS.len() as u64)) as usize];

            // Extract base RGB from the color parameter.
            let (cr, cg, cb) = match color {
                Color::Rgb(r, g, b) => (r as f32, g as f32, b as f32),
                _ => (0.0, 255.0, 0.0), // fallback green
            };
            let (r, g, b_val) = if dist == 0 {
                // Head: bright, washed-out version of the color
                let mix = |c: f32| -> u8 { (c * 0.8 + 255.0 * 0.2).min(255.0) as u8 };
                (mix(cr), mix(cg), mix(cb))
            } else if dist <= 4 {
                let intensity =
                    ((0x44 + (0x66u64).saturating_sub((dist as u64) * 0x14)) as f32) / 255.0;
                (
                    (cr * intensity) as u8,
                    (cg * intensity) as u8,
                    (cb * intensity) as u8,
                )
            } else {
                let fade = ((dist - 4) as f32).min(16.0);
                let intensity = ((0x22 as f32) - fade).max(0.0) / 255.0;
                (
                    (cr * intensity) as u8,
                    (cg * intensity) as u8,
                    (cb * intensity) as u8,
                )
            };

            let x = area.x + (col as u16);
            let y = area.y + (row as u16);

            // // Skip rendering inside the card area
            // if x >= occluder.x
            //     && x < occluder.x + occluder.width
            //     && y >= occluder.y
            //     && y < occluder.y + occluder.height
            // {
            //     continue;
            // }

            frame.render_widget(
                Paragraph::new(Span::styled(
                    ch.to_string(),
                    Style::default()
                        .fg(Color::Rgb(r, g, b_val))
                        .bg(Color::Rgb(0x06, 0x06, 0x0a)),
                )),
                Rect {
                    x,
                    y,
                    width: 1,
                    height: 1,
                },
            );
        }
        col += step;
    }
}
