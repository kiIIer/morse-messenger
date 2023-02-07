use crate::client::app::components::signal::SignalComponent;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;

pub struct CheatComponent {}

impl Default for CheatComponent {
    fn default() -> Self {
        CheatComponent {}
    }
}

impl CheatComponent {
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect, sig: SignalComponent) {}
}
