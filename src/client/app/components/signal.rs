use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;

pub struct SignalComponent {}

impl Default for SignalComponent {
    fn default() -> Self {
        SignalComponent {}
    }
}

impl SignalComponent {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {}
}
