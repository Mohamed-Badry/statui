use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Paragraph, Wrap},
};

use crate::{
    backend::CheckStatus,
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

    let border_color = match &endpoint_state.latest_status {
        Some(CheckStatus::Success { code, .. }) => Theme::color_code(code),
        Some(CheckStatus::Error { .. }) => Theme::STATUS_ERROR,
        None => Theme::BORDER_UNFOCUSED,
    };

    let title_block = create_title_block(endpoint_name, border_color);

    let par = Paragraph::new(create_lines(endpoint_state))
        .block(title_block)
        .gray()
        .centered()
        .wrap(Wrap { trim: true });

    frame.render_widget(par, chunk);
}

fn create_lines(endpoint_state: &EndpointState) -> Vec<Line<'static>> {
    let latency_stats = &endpoint_state.latency_stats;

    vec![
        Line::from(format!("URL: {}", endpoint_state.url)).left_aligned(),
        Line::from(format!("Method: {}", endpoint_state.method)).left_aligned(),
        Line::from(""),
        Line::from(""),
        // ]
        // vec![
        Line::from(format!("Min: {}ms", latency_stats.min.unwrap_or_default())),
        Line::from(format!("Max: {}ms", latency_stats.max.unwrap_or_default())),
        Line::from(format!("Avg: {}ms", latency_stats.avg.unwrap_or_default())),
    ]
}

// fn create_latency_stats(endpoint_state: &EndpointState) -> Vec<Line<'static>> {
// }

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
