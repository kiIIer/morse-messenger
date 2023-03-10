use crate::{
    client::{
        app::components::{
            cheat::CheatComponent,
            home::HomeComponent,
            signal::SignalComponent,
            tabs::{MenuItem, TabsComponent},
            trans::TransComponent,
        },
        morse::{convert, Letter},
    },
    morser::Signal,
};
use crossterm::event::{Event as CEvent, Event::Key, KeyCode, KeyCode::Char, KeyEventKind};
use std::time::Duration;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

mod components;

pub enum Mode {
    Input,
    Normal,
}

pub struct AppState {
    tx_server: UnboundedSender<Signal>,
    rx_server: UnboundedReceiver<Signal>,
    tx_sound: UnboundedSender<bool>,
    tx_letter: UnboundedSender<Letter>,
    // millis
    tick_rate: i32,
    precision: f64,
    time_unit: i32,

    homepage: HomeComponent,
    cheatsheet: CheatComponent,
    signal: SignalComponent,
    trans: TransComponent,
    tab: TabsComponent,

    active_tab: MenuItem,
    mode: Mode,

    should_quit: bool,
}

impl AppState {
    pub fn new(
        tick_rate: i32,
        time_unit: i32,
        precision: f64,
        tx_sound: UnboundedSender<bool>,
        tx_server: UnboundedSender<Signal>,
        tx_letter: UnboundedSender<Letter>,
        rx_server: UnboundedReceiver<Signal>,
    ) -> AppState {
        AppState {
            tx_server,
            rx_server,
            tx_letter,
            homepage: Default::default(),
            cheatsheet: Default::default(),
            signal: Default::default(),
            trans: Default::default(),
            tab: Default::default(),
            active_tab: MenuItem::Home,
            tick_rate,
            tx_sound,
            should_quit: false,
            time_unit,
            mode: Mode::Normal,
            precision,
        }
    }

    pub fn on_tick(&mut self) {
        self.signal.on_tick();
    }

    pub fn set_signal(&mut self, state: bool) {
        self.signal.set_signal(state);
        self.tx_sound.send(state).expect("Couldn't send sound");
    }

    pub fn count_word(&mut self) {
        self.trans.count_word();
    }

    pub fn signal_on(&mut self) {
        if let Mode::Normal = self.mode {
            self.tx_server
                .send(Signal { state: true })
                .expect("Couldn't send sound");
        }
    }

    pub fn signal_off(&mut self) {
        if let Mode::Normal = self.mode {
            self.tx_server
                .send(Signal { state: false })
                .expect("Couldn't send sound");
        }
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) {
        let fsize = f.size();

        let chunks_main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(2)].as_ref())
            .split(fsize);

        self.tab
            .draw(f, chunks_main[0], self.active_tab, &self.mode);
        match self.active_tab {
            MenuItem::Home => self.homepage.draw(f, chunks_main[1]),
            MenuItem::Signal => self.signal.draw(f, chunks_main[1]),
            MenuItem::Cheat => self.cheatsheet.draw(f, chunks_main[1], &self.signal),
            MenuItem::Trans => self.trans.draw(f, chunks_main[1], &self.signal),
        }
    }

    pub fn tick_rate_d(&self) -> Duration {
        Duration::from_millis(self.tick_rate as u64)
    }

    pub fn add_to_send(&mut self, letter: Letter) {
        self.trans.add_to_send(letter)
    }

    pub fn pop_to_send(&mut self) {
        self.trans.pop_to_send()
    }

    pub fn handle_c_event(&mut self, event: CEvent) {
        match self.mode {
            Mode::Normal => {
                if let Key(key) = event {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            Char('q') => self.should_quit = true,
                            Char('e') => self.mode = Mode::Input,
                            Char('0') => self.active_tab = MenuItem::Home,
                            Char('1') => self.active_tab = MenuItem::Signal,
                            Char('2') => self.active_tab = MenuItem::Cheat,
                            Char('3') => self.active_tab = MenuItem::Trans,
                            _ => {}
                        }
                    }
                }
            }

            Mode::Input => match event {
                Key(key) => match key.code {
                    KeyCode::Esc => self.mode = Mode::Normal,
                    KeyCode::Backspace => self.pop_to_send(),
                    KeyCode::Enter => {
                        let letters = self.trans.pending_add();

                        for letter in letters {
                            self.tx_letter.send(letter).expect("Couldn't send letter");
                        }
                    }
                    Char(symbol) => {
                        if let Some(letter) = convert(symbol) {
                            self.add_to_send(letter)
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
        }
    }

    pub fn add_letter(&mut self, letter: Letter) {
        self.trans.add_translated(letter);
    }
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
    pub fn rx_server(&mut self) -> &mut UnboundedReceiver<Signal> {
        &mut self.rx_server
    }
    pub fn time_unit_d(&self) -> Duration {
        Duration::from_millis(self.time_unit as u64)
    }
    pub fn precision(&self) -> f64 {
        self.precision
    }
}
