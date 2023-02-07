use crate::client::app::components::cheat::CheatComponent;
use crate::client::app::components::home::HomeComponent;
use crate::client::app::components::signal::SignalComponent;
use crate::client::app::components::tabs::{MenuItem, TabsComponent};
use crossterm::event::Event::Key;
use crossterm::event::{Event as CEvent, KeyCode};
use std::time::Duration;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Frame;

mod components;

pub struct AppState {
    homepage: HomeComponent,
    cheatsheet: CheatComponent,
    signal: SignalComponent,
    tab: TabsComponent,
    active_tab: MenuItem,
    should_quit: bool,

    // millis
    tick_rate: i32,
}

impl AppState {
    pub fn new(tick_rate: i32) -> AppState {
        AppState {
            homepage: Default::default(),
            cheatsheet: Default::default(),
            signal: Default::default(),
            tab: Default::default(),
            active_tab: MenuItem::Home,
            tick_rate,
            should_quit: false,
        }
    }

    pub fn on_tick(&mut self) {}

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) {
        let fsize = f.size();

        let chunks_main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(2)].as_ref())
            .split(fsize);

        self.tab.draw(f, chunks_main[0], self.active_tab);
    }

    pub fn tick_rate(&self) -> i32 {
        self.tick_rate
    }
    pub fn tick_rate_d(&self) -> Duration {
        Duration::from_millis(self.tick_rate as u64)
    }

    pub fn handle_c_event(&mut self, event: CEvent) {
        match event {
            Key(key) => match key.code {
                KeyCode::Char('q') => self.should_quit = true,
                _ => {}
            },
            _ => {}
        }
    }
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}
