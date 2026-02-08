use ratatui::{Frame, layout::Rect};

pub trait Component {
    fn render(&self, frame: &mut Frame, area: Rect);
}
