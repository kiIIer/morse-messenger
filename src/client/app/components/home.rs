use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
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
                Style::default().fg(Color::Black),
            )),
        ])
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
        // TODO: Write proper doc
        let tutorial =
            String::from("Well this is supposed to be a quick tutorial over how the app works.");
        HomeComponent {
            heading: String::from("Morse Code Messenger"),
            tutorial,
        }
    }
}
