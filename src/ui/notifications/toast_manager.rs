use crate::app::RenderContext;
use crate::ui::components::Component;
use crate::ui::notifications::toast::Toast;
use ratatui::layout::Rect;
use ratatui::Frame;
use std::time::Duration;

const TOAST_W: u16 = 42;
const TOAST_GAP: u16 = 1;

pub struct ToastManager {
    toasts: Vec<Toast>,
}

impl ToastManager {
    pub fn new() -> Self {
        Self { toasts: vec![] }
    }

    pub fn push(&mut self, toast_message: impl Into<String>) {
        self.toasts.push(Toast::new(toast_message))
    }

    pub fn tick(&mut self) {
        self.toasts.retain(|t| !t.is_expired())
    }
}

impl Component for ToastManager {
    fn render(&mut self, frame: &mut Frame, area: Rect, _context: &RenderContext, _dt: Duration) {
        let x = area.right().saturating_sub(TOAST_W + 1);
        let mut y = area.y + 1;

        for toast in &self.toasts {
            let h = toast.required_height(TOAST_W);

            if y + h > area.bottom() {
                break;
            }

            toast.render(frame, Rect { x, y, width: TOAST_W, height: h });
            y += h + TOAST_GAP;
        }
    }
}
