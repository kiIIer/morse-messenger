use crate::client::app::components::cheat::CheatComponent;
use crate::client::app::components::home::HomeComponent;
use crate::client::app::components::signal::SignalComponent;
use crate::client::app::components::tabs::{MenuItem, TabsComponent};
use std::time::Duration;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::layout::{Constraint, Direction, Layout};
use tui::Frame;

mod components;

pub struct AppState {
    homepage: HomeComponent,
    cheatsheet: CheatComponent,
    signal: SignalComponent,
    tabs: TabsComponent,
    active_tab: MenuItem,

    // millis
    tick_rate: i32,
}

impl AppState {
    pub fn new(tick_rate: i32) -> AppState {
        AppState {
            homepage: Default::default(),
            cheatsheet: Default::default(),
            signal: Default::default(),
            tabs: Default::default(),
            active_tab: MenuItem::Home,
            tick_rate,
        }
    }

    pub fn on_tick(&mut self) {}

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) {
        let fsize = f.size();

        let chunks_main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(2)].as_ref())
            .split(fsize);

        self.homepage.draw(f, chunks_main[0]);
    }

    pub fn tick_rate(&self) -> i32 {
        self.tick_rate
    }
    pub fn tick_rate_d(&self) -> Duration {
        Duration::from_millis(self.tick_rate as u64)
    }
}
