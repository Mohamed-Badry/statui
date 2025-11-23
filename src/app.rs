use color_eyre::{Result, eyre};
use std::time::Duration;

use ratatui::{
    Terminal,
    backend::Backend,
    crossterm::event::{self, Event},
};

use tokio::sync::mpsc::Receiver;

use crate::{
    actions::handle_action,
    backend::CheckResult,
    keymap::{self, KeyMap, handle_key_event},
};
use crate::{state::App, ui};

/// TUI entry point that handles drawing the ui, handling input, and displaying
/// results of a check.
pub async fn run_app(
    mut app: &mut App,
    terminal: &mut Terminal<impl Backend>,
    mut rx: Receiver<CheckResult>,
) -> Result<()> {
    loop {
        // 1. Draw the UI
        terminal.draw(|f| ui::render_ui(f, &mut app))?;

        // 2. Handle input
        let km: KeyMap = keymap::default_keymap();
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                let Some(action) = handle_key_event(key, &km) else {
                    continue;
                };

                // Exit if true is returned
                if handle_action(&action, &mut app) {
                    return eyre::Ok(());
                };
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
