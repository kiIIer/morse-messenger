use crate::client::app::components::home::HomeComponent;
use std::time::Duration;
use tui::backend::Backend;
use tui::Terminal;

mod components;

pub struct AppState {
    homepage: HomeComponent,

    // millis
    tick_rate: i32,
}

impl AppState {
    pub fn new(tick_rate: i32) -> AppState {
        AppState {
            homepage: HomeComponent::default(),
            tick_rate,
        }
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app_state: AppState,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
