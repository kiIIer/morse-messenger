use crate::{client::app::components::signal::SignalComponent, client::morse::Letter};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub struct TransComponent {
    received: String,
    to_send: Vec<Letter>,
    sending: String,
    sent: usize,
}

impl Default for TransComponent {
    fn default() -> Self {
        TransComponent {
            received: String::new(),

            to_send: vec![],
            sending: String::new(),
            sent: 0,
        }
    }
}

impl TransComponent {
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect, signal: &SignalComponent) {
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(10), Constraint::Min(2)].as_ref())
            .split(area);

        signal.draw(f, main_chunks[0]);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
            .split(main_chunks[1]);

        self.draw_receiving(f, chunks[1]);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(2)].as_ref())
            .split(chunks[0]);

        self.draw_sending(f, left_chunks[1]);
        self.draw_to_send(f, left_chunks[0]);
    }

    pub fn add_translated(&mut self, letter: Letter) {
        let letter: char = letter.into();
        self.received.push(letter);
    }

    pub fn add_to_send(&mut self, letter: Letter) {
        self.to_send.push(letter)
    }

    pub fn pop_to_send(&mut self) {
        self.to_send.pop();
    }

    pub fn count_word(&mut self) {
        self.sent += 1;
    }

    pub fn pending_add(&mut self) -> Vec<Letter> {
        let mut to_send: Vec<Letter> = self.to_send.drain(0..self.to_send.len()).collect();
        let mut sending_string = String::new();

        for letter in to_send.iter() {
            sending_string.push(char::from(letter));
        }

        if !to_send.is_empty() && to_send[to_send.len() - 1] != Letter::Space {
            to_send.push(Letter::Space);
            sending_string.push(' ');
        }

        self.sending.push_str(sending_string.as_str());

        to_send
    }

    fn draw_receiving<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let max_line_len = area.width - 2;
        let max_lines = area.height - 2;
        let max_mes_len = max_line_len * max_lines;
        let mut scroll = 0;

        while (self.received.len() as u16 - (scroll * max_line_len)) as f64 / max_mes_len as f64
            > 0.75
        {
            scroll += 1;
        }

        let received = Paragraph::new(vec![Spans::from(Span::raw(&self.received))])
            .block(Block::default().borders(Borders::ALL).title("Receiving..."))
            .wrap(Wrap { trim: true })
            .scroll((scroll, 0));

        f.render_widget(received, area);
    }

    fn draw_sending<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let max_line_len = area.width - 2;
        let max_lines = area.height - 2;
        let max_mes_len = max_line_len * max_lines;
        let mut scroll = 0;

        while (self.sending.len() as u16 - (scroll * max_line_len)) as f64 / max_mes_len as f64
            > 0.75
        {
            scroll += 1;
        }

        let (sent, pending) = self.sending.split_at(self.sent);
        let sending = Paragraph::new(vec![Spans::from(vec![
            Span::styled(sent, Style::default().fg(Color::DarkGray)),
            Span::styled(pending, Style::default()),
        ])])
        .block(Block::default().title("Sending...").borders(Borders::ALL))
        .wrap(Wrap { trim: true })
        .scroll((scroll, 0));

        f.render_widget(sending, area);
    }

    fn draw_to_send<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let to_send = self
            .to_send
            .iter()
            .map(|l| l.into())
            .fold(String::new(), |mut acc, l| {
                acc.push(l);
                acc
            });
        let to_send = Paragraph::new(vec![Spans::from(Span::styled(
            to_send,
            Style::default().fg(Color::Red),
        ))])
        .block(Block::default().borders(Borders::ALL).title("Your message"));

        f.render_widget(to_send, area);
    }
}
