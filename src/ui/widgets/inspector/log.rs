use chrono::{DateTime, Local};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Row, Table},
};

use crate::{backend::CheckStatus, state::EndpointState, ui::theme::Theme};

/// Render a logs panel for an endpoint.
pub fn render_log(
    endpoint_state: &EndpointState,
    frame: &mut Frame,
    area: Rect,
    status_color: Color,
) {
    let header = Row::new(vec![
        Line::from("TIME").centered(),
        Line::from("|"),
        Line::from("CODE").centered(),
        Line::from("|"),
        Line::from("MESSAGE").centered(),
        Line::from("|"),
        Line::from("LATENCY").centered(),
    ])
    .style(
        Style::default()
            .fg(status_color)
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::UNDERLINED),
    );
    let widths = vec![
        Constraint::Min(10),
        Constraint::Length(1),
        Constraint::Min(4),
        Constraint::Length(1),
        Constraint::Min(15),
        Constraint::Length(1),
        Constraint::Min(15),
    ];

    let block = Block::new();
    let logs = create_log_lines(endpoint_state);

    let log_table = Table::new(logs, widths).header(header).block(block);

    frame.render_widget(log_table, area);
}

fn create_log_lines(endpoint_state: &EndpointState) -> Vec<Row<'static>> {
    let style = Style::default();

    endpoint_state
        .recent_checks
        .iter()
        .map(|(st, result)| {
            // Format the time
            let datetime: DateTime<Local> = (*st).into();
            let time_str = datetime.format("%H:%M:%S").to_string();
            let time_span =
                Span::styled(time_str, style.fg(Theme::INSPECTOR_TEXT_FG)).into_centered_line();

            // Format the status
            let (status_code_str, status_message_str, status_color) = match &result.status {
                CheckStatus::Success { code, text } => (
                    format!("{:<3}", code),
                    format!("{}", text),
                    Theme::color_code(&code),
                ),
                CheckStatus::Error { message } => (
                    "ERR".to_string(),
                    format!("{}", message),
                    Theme::STATUS_ERROR,
                ),
            };
            let status_code_span =
                Span::styled(status_code_str, style.fg(status_color)).into_centered_line();
            let status_message_span =
                Span::styled(status_message_str, style.fg(status_color)).into_centered_line();

            // Format the latency
            let (latency_str, latency_color) = (
                format!("{}ms", &result.latency.as_millis()),
                Theme::latency_color(&result.latency),
            );
            let latency_span =
                Span::styled(latency_str, style.fg(latency_color)).into_centered_line();

            Row::new(vec![
                time_span,
                Line::from("|"),
                status_code_span,
                Line::from("|"),
                status_message_span,
                Line::from("|"),
                latency_span,
            ])
        })
        .collect()
}
