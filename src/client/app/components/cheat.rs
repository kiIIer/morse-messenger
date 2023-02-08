use crate::client::app::components::signal::SignalComponent;
use crate::client::morse::Letter::*;
use crate::client::morse::{Letter, Morse};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, Cell, Row, Table};
use tui::Frame;

pub struct CheatComponent {
    alphabet: Vec<Letter>,
}

impl Default for CheatComponent {
    fn default() -> Self {
        CheatComponent {
            alphabet: vec![
                A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, N0,
                N1, N2, N3, N4, N5, N6, N7, N8, N9, Dot
            ],
        }
    }
}

impl CheatComponent {
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect, sig: &SignalComponent) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(17), Constraint::Min(2)].as_ref())
            .split(area);

        let alpha_data: Vec<Row> = self
            .alphabet
            .iter()
            .map(|l| {
                let letter: char = l.into();
                let morse: Vec<Morse> = l.into();
                let mut morse_text = String::new();

                for m in morse {
                    morse_text.push(m.into());
                    morse_text.push(' ');
                }

                morse_text.pop();

                Row::new(vec![
                    Cell::from(Span::raw(letter.to_string())),
                    Cell::from(morse_text),
                ])
            })
            .collect();

        let table = Table::new(alpha_data)
            .block(
                Block::default()
                    .style(Style::default().fg(Color::White))
                    .borders(Borders::ALL)
                    .title("Cheat Sheet"),
            )
            .header(Row::new(vec![
                Cell::from(Span::styled(
                    "Char",
                    Style::default().add_modifier(Modifier::BOLD).fg(Color::Red),
                )),
                Cell::from(Span::styled(
                    "Morse",
                    Style::default().add_modifier(Modifier::BOLD).fg(Color::Red),
                )),
            ]))
            .widths(&[Constraint::Percentage(20), Constraint::Percentage(80)]);

        f.render_widget(table, chunks[0]);
        sig.draw(f, chunks[1]);
    }
}
