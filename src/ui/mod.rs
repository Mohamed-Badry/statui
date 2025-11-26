pub mod fx;
mod theme;
mod util;
mod widgets;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};
use std::time::Duration;

use crate::{
    state::{App, AppMode},
    ui::fx::FxManager,
};
use widgets::{
    footer::render_footer, inspector::render_inspector, table::render_table,
    welcome::render_welcome_message,
};

/// Renders the UI widgets for the application.
pub fn render_ui(
    frame: &mut Frame,
    mut app: &mut App,
    fx_manager: &mut FxManager,
    elapsed: Duration,
) {
    if app.endpoint_order.is_empty() {
        render_welcome_message(frame);
        return;
    }

    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(2)])
        .split(frame.area());

    let workspace_area = root[0];
    let footer_area = root[1];

    let constraints = match app.mode {
        AppMode::Normal => vec![Constraint::Percentage(100)],
        AppMode::Inspecting => vec![Constraint::Percentage(50), Constraint::Percentage(50)],
    };

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(workspace_area);

    render_table(frame, &mut app, chunks[0]);

    if app.mode == AppMode::Inspecting {
        render_inspector(frame, &mut app, chunks[1]);
        fx_manager.render_inspector(frame, chunks[1], elapsed.into());
    }

    render_footer(frame, footer_area);
}
