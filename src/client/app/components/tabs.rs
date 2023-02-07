use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;

pub struct TabsComponent {}

impl Default for TabsComponent {
    fn default() -> Self {
        TabsComponent {}
    }
}
impl TabsComponent {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {}
}
