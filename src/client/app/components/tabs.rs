use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;

pub enum MenuItem {
    Home,
    Signal,
    Cheat,
    Trans,
}

impl From<MenuItem> for usize {
    fn from(value: MenuItem) -> Self {
        match value {
            MenuItem::Home => 0,
            MenuItem::Signal => 1,
            MenuItem::Cheat => 2,
            MenuItem::Trans => 3,
        }
    }
}

pub struct TabsComponent {}

impl Default for TabsComponent {
    fn default() -> Self {
        TabsComponent {}
    }
}

impl TabsComponent {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {}
}
