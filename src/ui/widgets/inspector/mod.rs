mod availability;
mod header;
mod latency;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    widgets::{Block, Borders},
};

use crate::{
    state::{App, EndpointState},
    ui::{theme::Theme, util},
};

pub fn render_inspector(frame: &mut Frame, app: &mut App, chunk: Rect) {
    let Some(selected) = app.table_state.selected() else {
        // if no endpoint is selected in the table render an empty
        // inspector with title "Pick an endpoint"
        let empty_inspector = create_title_block("Pick an endpoint", Theme::BORDER_UNFOCUSED);
        frame.render_widget(empty_inspector, chunk);
        return;
    };

    let Some(endpoint_name) = app.endpoint_order.get(selected) else {
        return;
    };

    let Some(endpoint_state) = app.endpoint_states.get(endpoint_name) else {
        return;
    };

    // Calculate areas
    let outer_area = chunk;
    let inner_area = chunk.inner(Margin::new(1, 1));

    // Render the outer border block
    let status_color = util::get_status_color(&endpoint_state.latest_status);
    let outer_block = create_title_block(&endpoint_name, status_color);
    render_outer_block(frame, outer_block, outer_area);

    // Split the inner area into 3 chunks (with 2 separators)
    // Top: Header
    // Separator Line
    // Middle: Latency Stats | Availability Stats
    // Separator Line
    // Bottom: Not sure yet
    let layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(4),
            Constraint::Length(1),
            Constraint::Length(5),
            Constraint::Length(1),
            Constraint::Min(10),
        ],
    )
    .split(inner_area);

    // Split the middle area into 2 chunks (with 1 separator)
    // Latency Stats | Availability Stats
    let stats_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(50),
            Constraint::Length(1),
            Constraint::Percentage(50),
        ],
    )
    .split(layout[2]);

    // Top: Header
    header::render_inspector_header(&endpoint_state, frame, layout[0]);

    render_separator(frame, layout[1], status_color);

    // Middle: Latency Stats | Availability Stats
    latency::render_latency_stats(&endpoint_state, frame, stats_layout[0]);
    let stats_separator = Block::default()
        .borders(Borders::LEFT)
        .border_set(Theme::PANEL_BORDER)
        .border_style(status_color);
    frame.render_widget(stats_separator, stats_layout[1]);
    availability::render_availability_stats(&endpoint_state, frame, stats_layout[2]);

    render_separator(frame, layout[3], status_color);

    // Bottom: Not sure yet
    render_temp_bottom(&endpoint_state, frame, layout[4]);
}

fn render_outer_block(frame: &mut Frame, outer_block: Block, area: Rect) {
    frame.render_widget(outer_block, area);
}

fn render_separator(frame: &mut Frame, area: Rect, border_color: Color) {
    let separator = Block::default()
        .borders(Borders::BOTTOM)
        .border_set(Theme::PANEL_BORDER)
        .border_style(border_color);
    frame.render_widget(separator, area);
}

// Not sure what to put in the bottom layout yet.
fn render_temp_bottom(_endpoint_state: &EndpointState, frame: &mut Frame, area: Rect) {
    frame.render_widget(Block::new(), area);
}

fn create_title_block(endpoint_name: &str, status_color: Color) -> Block<'static> {
    let title = util::wrap_with_brackets(
        &format!("Inspector: {}", endpoint_name),
        Style::default().fg(status_color),
        Style::default().fg(status_color),
    );

    Block::bordered()
        .title(title)
        .border_set(Theme::PANEL_BORDER)
        .border_style(status_color)
        .title_style(status_color)
        .title_alignment(Alignment::Left)
}
