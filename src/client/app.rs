use std::time::Duration;
use tui::backend::Backend;
use tui::Terminal;

mod components;

pub(crate) struct AppState {
    // millis
    tick_rate: i32,
}

impl AppState {
    pub(crate) fn new(tick_rate: i32) -> AppState {
        AppState { tick_rate }
    }
}

pub(crate) fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app_state: AppState,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
