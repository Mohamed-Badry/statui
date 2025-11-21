use color_eyre::Result;
use std::{
    time::Duration,
};

use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    backend::Backend,
    Terminal,
};

use tokio::sync::mpsc::Receiver;

use crate::{state::App, ui};
use crate::{backend::CheckResult};

/// TUI entry point that handles drawing the ui, handling input, and displaying
/// results of a check.
pub async fn run_app(
    app: &mut App,
    terminal: &mut Terminal<impl Backend>,
    mut rx: Receiver<CheckResult>,
) -> Result<()> {
    loop {
        // 1. Draw the UI
        terminal.draw(|f| ui::render_ui(f, &app))?;

        // 2. Handle input
        // Simple input handling to quit on q
        // TODO: replace with keymap and actions model later
        // to handle all types of key inputs
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    // 'q' was pressed, so we quit
                    return Ok(());
                }
            }
        }

        // 3. Handle messages from the backend
        // After drawing and handling input, we check for new messages.
        // We use 'try_recv' in a loop to drain the channel of all
        // pending messages without blocking.
        while let Ok(result) = rx.try_recv() {
            app.on_result(result);
        }
    }
}
