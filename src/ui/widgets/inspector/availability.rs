use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::{
    state::EndpointState,
    ui::{theme::Theme, util},
};

pub fn render_availability_stats(endpoint_state: &EndpointState, frame: &mut Frame, area: Rect) {
    let block = Block::new();
    let avail_stats_lines = create_avail_stats_lines(endpoint_state);
    let h = avail_stats_lines.len() as u16;
    let w = avail_stats_lines
        .iter()
        .map(|line| line.width())
        .max()
        .unwrap_or(0) as u16;

    let par = Paragraph::new(avail_stats_lines).block(block).centered();

    frame.render_widget(par, util::centered_area(h, w, area));
}

fn create_avail_stats_lines(endpoint_state: &EndpointState) -> Vec<Line<'static>> {
    let stats = &endpoint_state.availability_stats;

    // Calculate Uptime Color
    // 99%+ is Green, 90-99% is Yellow, below is Red
    let uptime_val = stats.uptime_percent.unwrap_or(0.0);
    let uptime_color = if uptime_val >= 99.0 {
        Theme::STATUS_OK
    } else if uptime_val >= 90.0 {
        Theme::STATUS_WARN
    } else {
        Theme::STATUS_ERROR
    };

    // Calculate Fails Color
    // 0 Fails is Green, >0 is Red
    let fail_color = if stats.number_of_fails > 0 {
        Theme::STATUS_ERROR
    } else {
        Theme::STATUS_OK
    };

    let uptime_str = match stats.uptime_percent {
        Some(uptime_val) => format!("{:.1}%", uptime_val),
        None => "-".to_string(),
    };

    vec![
        Line::from(vec![
            Span::styled(
                format!("{:<8}", "Uptime:"),
                Style::default().fg(Theme::INSPECTOR_TEXT_FG),
            ),
            Span::styled(
                uptime_str,
                Style::default()
                    .fg(uptime_color)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
        .left_aligned(),
        Line::from(vec![
            Span::styled(
                format!("{:<8}", "Checks:"),
                Style::default().fg(Theme::INSPECTOR_TEXT_FG),
            ),
            // No style for the number of checks
            Span::raw(stats.number_of_checks.to_string()),
        ])
        .left_aligned(),
        Line::from(vec![
            Span::styled(
                format!("{:<8}", "Fails:"),
                Style::default().fg(Theme::INSPECTOR_TEXT_FG),
            ),
            Span::styled(
                stats.number_of_fails.to_string(),
                Style::default().fg(fail_color),
            ),
        ])
        .left_aligned(),
    ]
}
