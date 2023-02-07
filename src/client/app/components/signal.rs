use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;

pub struct SignalComponent {
    signal: bool,
    signal_data: Vec<(f64, f64)>,
    window: [f64; 2],
}

impl Default for SignalComponent {
    fn default() -> Self {
        SignalComponent {
            signal: false,
            signal_data: (0..100).into_iter().map(|i| (i as f64, 0.0)).collect(),
            window: [0.0, 100.0],
        }
    }
}

impl SignalComponent {
    fn on_tick(&mut self) {
        for _ in 0..4 {
            self.signal_data.remove(0);
        }
        let chart_state = match self.signal {
            true => 1.0,
            false => 0.0,
        };
        let last_end = (self.signal_data[self.signal_data.len() - 1].0 + 1.0) as i64;
        let a = last_end..last_end + 4;
        self.signal_data
            .extend(a.into_iter().map(|x| (x as f64, chart_state)));

        self.window[0] += 4.0;
        self.window[1] += 4.0;
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {}

    pub fn signal(&self) -> bool {
        self.signal
    }

    pub fn set_signal(&mut self, signal: bool) {
        self.signal = signal;
    }
}
