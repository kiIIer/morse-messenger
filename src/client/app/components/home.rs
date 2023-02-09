use tui::layout::{Constraint, Layout};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::Wrap,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub struct HomeComponent {
    heading: String,
    tutorial: String,
}

impl HomeComponent {
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect) {
        let p = Paragraph::new(vec![
            Spans::from(Span::styled(
                &self.heading,
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )),
            Spans::from(""),
            Spans::from(Span::styled(
                &self.tutorial,
                Style::default().fg(Color::White),
            )),
        ])
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        );

        f.render_widget(p, rect);
    }
}

impl Default for HomeComponent {
    fn default() -> Self {
        let tutorial =
            String::from("This is a small tutorial of how this app works. There are 3 main tabs. You can navigate them with '1', '2', '3' keys. Signal is a visual representation of current signal in app. The cheat sheet basically shows you morse code representation of supported symbols so you can learn and use them. The last tab is translator. We'll get to that. You send signal in normal mode by pressing 'Space'. The current mode is displayed on top right part of the screen. You can enter edit mode with 'e' key. Then you can write a message to send in translate tab on the left. When you press enter it will be scheduled to be sent in box below. The grey colored letters are already sent. The app will try to translate incoming signal in the box on the right. To exit the edit mode please press 'esc' and to quit the app press 'q'. When you are transmitting something, all people connected to server will hear that. And... plz don't try to send all at once. Nothing good will happen^^'");
        HomeComponent {
            heading: String::from("Morse Code Messenger"),
            tutorial,
        }
    }
}
