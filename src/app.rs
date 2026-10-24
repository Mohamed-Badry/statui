use color_eyre::{Result, eyre};
use std::time::{Duration, Instant};

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
    state::App,
    ui::{self, fx::FxManager},
};

const FPS: usize = 60;

/// TUI entry point that handles drawing the ui, handling input, and displaying
/// results of a check.
pub async fn run_app(
    app: &mut App,
    terminal: &mut Terminal<impl Backend>,
    mut rx: Receiver<CheckResult>,
) -> Result<()> {
    let mut fx_manager = FxManager::new();
    fx_manager.trigger_startup();

    // I'll run the inspector open animation only first time it opens for now
    fx_manager.trigger_open_inspector();

    let mut last_frame = Instant::now();
    loop {
        let elapsed = last_frame.elapsed();
        last_frame = Instant::now();

        // 1. Draw the UI
        terminal.draw(|frame| {
            ui::render_ui(frame, app, &mut fx_manager, elapsed);
            fx_manager.render_global(frame, frame.area(), elapsed.into());
        })?;

        // 2. Handle input
        let km: KeyMap = keymap::default_keymap();
        if event::poll(Duration::from_millis(1000 / FPS as u64))?
            && let Event::Key(key) = event::read()? {
                let Some(action) = handle_key_event(key, &km) else {
                    continue;
                };

                // Exit if true is returned
                if handle_action(&action, app, &mut fx_manager) {
                    return eyre::Ok(());
                };
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
