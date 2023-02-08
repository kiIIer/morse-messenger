use crate::client::app::components::signal::SignalComponent;
use crate::client::morse::Letter;
use tokio_stream::StreamExt;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::Frame;

pub struct TransComponent {
    received: String,
    to_send: Vec<Letter>,
    sending: String,
    sent: usize,
}

impl Default for TransComponent {
    fn default() -> Self {
        TransComponent {
            received: String::from("me kind of a test message which will be transmitted and it should be long so, hippoty hoppoty lorem ipsum my property. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum"),

            to_send: vec![],
            sending: String::from("This is some kind of a test message which will be transmitted and it should be long so, hippoty hoppoty lorem ipsum my property. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum"),
            sent: 10
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

    pub fn add_translated(&mut self, letter: &Letter) {
        let letter: char = letter.into();
        self.received.push(letter);
    }

    pub fn add_to_send(&mut self, letter: Letter) {
        self.to_send.push(letter)
    }

    pub fn pop_to_send(&mut self) {
        self.to_send.pop();
    }

    pub fn pending_add(&mut self) -> Vec<Letter> {
        let to_send: Vec<Letter> = self.to_send.drain(0..self.to_send.len()).collect();
        let mut sending_string = String::new();

        for letter in to_send.iter() {
            sending_string.push(char::from(letter));
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
