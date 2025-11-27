use std::time::Duration;

use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::{
    state::EndpointState,
    ui::{theme::Theme, util},
};

pub fn render_latency_stats(endpoint_state: &EndpointState, frame: &mut Frame, area: Rect) {
    let block = Block::new();
    let latency_stats_lines = create_latency_stats_lines(endpoint_state);
    let h = latency_stats_lines.len() as u16;
    let w = latency_stats_lines
        .iter()
        .map(|line| line.width())
        .max()
        .unwrap_or(0) as u16;

    let par = Paragraph::new(latency_stats_lines).block(block).centered();

    frame.render_widget(par, util::centered_area(h, w, area));
}

fn create_latency_stats_lines(endpoint_state: &EndpointState) -> Vec<Line<'static>> {
    let latency_stats = &endpoint_state.latency_stats;

    // Helper closure to format and colorize a single latency value
    let format_latency = |val: Option<u64>| -> Span {
        match val {
            Some(ms) => {
                let color = Theme::latency_color(&Duration::from_millis(ms));
                Span::styled(format!("{}ms", ms), Style::default().fg(color))
            }
            None => Span::styled("-", Style::default().fg(Theme::INSPECTOR_TEXT_FG)),
        }
    };

    vec![
        Line::from(vec![
            // Pad label to 5 chars so numbers align vertically
            Span::styled(
                format!("{:<5}", "Min:"),
                Style::default().fg(Theme::INSPECTOR_TEXT_FG),
            ),
            format_latency(latency_stats.min),
        ])
        .left_aligned(),
        Line::from(vec![
            Span::styled(
                format!("{:<5}", "Max:"),
                Style::default().fg(Theme::INSPECTOR_TEXT_FG),
            ),
            format_latency(latency_stats.max),
        ])
        .left_aligned(),
        Line::from(vec![
            Span::styled(
                format!("{:<5}", "Avg:"),
                Style::default().fg(Theme::INSPECTOR_TEXT_FG),
            ),
            format_latency(latency_stats.avg),
        ])
        .left_aligned(),
    ]
}
