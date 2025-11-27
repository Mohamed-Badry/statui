use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
};

use crate::{
    state::EndpointState,
    ui::{theme::Theme, util},
};

pub fn render_inspector_header(endpoint_state: &EndpointState, frame: &mut Frame, area: Rect) {
    let block = Block::new();
    let header_lines = create_header_lines(endpoint_state);

    let h = header_lines.len() as u16;
    let w = header_lines
        .iter()
        .map(|line| line.width())
        .max()
        .unwrap_or(0) as u16;

    let inspector_header = Paragraph::new(header_lines)
        .block(block)
        .centered()
        .wrap(Wrap { trim: true });

    frame.render_widget(inspector_header, util::centered_area(h, w, area));
}

fn create_header_lines(endpoint_state: &EndpointState) -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::styled("URL:    ", Style::default().fg(Theme::INSPECTOR_TEXT_FG)),
            Span::styled(
                endpoint_state.url.to_owned(),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::UNDERLINED),
            ),
        ])
        .left_aligned(),
        Line::from(vec![
            Span::styled("Method: ", Style::default().fg(Theme::INSPECTOR_TEXT_FG)),
            Span::styled(
                endpoint_state.method.to_owned(),
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
        .left_aligned(),
    ]
}
