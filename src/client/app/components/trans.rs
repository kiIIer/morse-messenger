use crate::client::app::components::signal::SignalComponent;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::Frame;

pub struct TransComponent {}

impl Default for TransComponent {
    fn default() -> Self {
        TransComponent {}
    }
}

impl TransComponent {
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect, signal: &SignalComponent) {
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(10), Constraint::Min(2)].as_ref())
            .split(area);

        signal.draw(f, main_chunks[0]);
    }
}
