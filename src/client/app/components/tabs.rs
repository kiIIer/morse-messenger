use crate::client::app::Mode;
use std::any::Any;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Tabs};
use tui::Frame;

#[derive(Copy, Clone)]
pub enum MenuItem {
    Home,
    Signal,
    Cheat,
    Trans,
}

impl From<MenuItem> for usize {
    fn from(value: MenuItem) -> Self {
        match value {
            MenuItem::Home => 0,
            MenuItem::Signal => 1,
            MenuItem::Cheat => 2,
            MenuItem::Trans => 3,
        }
    }
}

pub struct TabsComponent {
    titles: Vec<String>,
}

impl Default for TabsComponent {
    fn default() -> Self {
        TabsComponent {
            titles: vec![
                "Home".to_string(),
                "Signal".to_string(),
                "Cheat sheet".to_string(),
                "Translator".to_string(),
            ],
        }
    }
}

impl TabsComponent {
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect, active: MenuItem, mode: &Mode) {
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(10), Constraint::Length(6)].as_ref())
            .split(area);

        let tab_titles = self
            .titles
            .iter()
            .enumerate()
            .map(|(i, t)| {
                Spans::from(vec![
                    Span::styled(format!("{}. ", i), Style::default().fg(Color::Red)),
                    Span::styled(t, Style::default().fg(Color::White)),
                ])
            })
            .collect();

        let tabs = Tabs::new(tab_titles)
            .select(active.into())
            .block(
                Block::default()
                    .title("Tabs")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White)),
            )
            .highlight_style(Style::default().fg(Color::LightRed));

        f.render_widget(tabs, main_chunks[0]);

        let mode_text = match mode {
            Mode::Normal => "NORM",
            Mode::Input => "INPT",
        };

        let mode = Paragraph::new(vec![Spans::from(Span::styled(
            mode_text,
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ))])
        .block(Block::default().borders(Borders::ALL).title("Mode"));

        f.render_widget(mode, main_chunks[1]);
    }
}
