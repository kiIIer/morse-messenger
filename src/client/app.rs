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
use crate::client::app::components::trans::TransComponent;

mod components;

pub struct AppState {
    homepage: HomeComponent,
    cheatsheet: CheatComponent,
    signal: SignalComponent,
    trans: TransComponent,
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
            trans: Default::default(),
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
        match self.active_tab {
            MenuItem::Home => self.homepage.draw(f, chunks_main[1]),
            MenuItem::Signal => self.signal.draw(f, chunks_main[1]),
            MenuItem::Cheat => self.cheatsheet.draw(f, chunks_main[1]),
            MenuItem::Trans => self.trans.draw(f, chunks_main[1]),
        }
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
                KeyCode::Char('0') => self.active_tab = MenuItem::Home,
                KeyCode::Char('1') => self.active_tab = MenuItem::Signal,
                KeyCode::Char('2') => self.active_tab = MenuItem::Cheat,
                KeyCode::Char('3') => self.active_tab = MenuItem::Trans,
                _ => {}
            },
            _ => {}
        }
    }
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}
