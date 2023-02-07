use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;

pub struct TransComponent {}

impl Default for TransComponent {
    fn default() -> Self {
        TransComponent {}
    }
}

impl TransComponent {
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {}
}
