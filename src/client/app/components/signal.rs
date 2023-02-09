use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset},
    symbols,
    Frame
};

pub struct SignalComponent {
    signal: bool,
    signal_data: Vec<(f64, f64)>,
    window: [f64; 2],
}

impl Default for SignalComponent {
    fn default() -> Self {
        SignalComponent {
            signal: false,
            signal_data: (0..200).into_iter().map(|i| (i as f64, 0.0)).collect(),
            window: [0.0, 200.0],
        }
    }
}

impl SignalComponent {
    pub fn on_tick(&mut self) {
        for _ in 0..8 {
            self.signal_data.remove(0);
        }
        let chart_state = match self.signal {
            true => 1.0,
            false => 0.0,
        };
        let last_end = (self.signal_data[self.signal_data.len() - 1].0 + 1.0) as i64;
        let a = last_end..last_end + 8;
        self.signal_data
            .extend(a.into_iter().map(|x| (x as f64, chart_state)));

        self.window[0] += 8.0;
        self.window[1] += 8.0;
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let x_labels = vec![
            Span::styled(
                format!("{:.1}", (self.window[0] / 20.0)),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!("{:.1}", (self.window[0] + self.window[1]) / 40.0)),
            Span::styled(
                format!("{:.1}", (self.window[1] / 20.0)),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ];
        let datasets = vec![Dataset::default()
            .name("Signal")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Red))
            .data(&self.signal_data)];

        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .title(Span::styled("Signal", Style::default().fg(Color::White)))
                    .borders(Borders::ALL),
            )
            .x_axis(
                Axis::default()
                    .title("Time")
                    .style(Style::default().fg(Color::Gray))
                    .labels(x_labels)
                    .bounds(self.window),
            )
            .y_axis(
                Axis::default()
                    .title("Magnitude")
                    .style(Style::default().fg(Color::Gray))
                    .labels(vec![
                        Span::styled("-0.5", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(""),
                        Span::styled("1.5", Style::default().add_modifier(Modifier::BOLD)),
                    ])
                    .bounds([-0.5, 1.5]),
            );
        f.render_widget(chart, area);
    }

    pub fn signal(&self) -> bool {
        self.signal
    }

    pub fn set_signal(&mut self, signal: bool) {
        self.signal = signal;
    }
}
