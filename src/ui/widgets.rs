use ratatui::{
    layout::Constraint,
    prelude::*,
    symbols,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use crate::backend::CheckStatus;
use crate::state::App;

const SPARKLINE_LENGTH: usize = 10;

// TODO: Improve how the table looks in general and make it interactive
pub fn render_table(frame: &mut Frame, app: &mut App) {
    let rows = create_rows(&app);

    let header = ["NAME", "STATUS", "LATENCY", "TREND"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .height(1);

    let widths = vec![
        Constraint::Min(10),
        Constraint::Min(10),
        Constraint::Min(10),
        Constraint::Min(10),
    ];

    // TODO: Make a better header
    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::bordered()
                .title(
                    Line::from("Statui ")
                        .left_aligned()
                        .style(Style::new().blue().italic()),
                )
                .border_set(symbols::border::DOUBLE),
        )
        .highlight_symbol(">> ")
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    let area = centered_rect(99, 99, frame.area());

    frame.render_stateful_widget(table, area, &mut app.table_state);
}

/// Return the endpoints as a vector of Rows to build the table.
fn create_rows(app: &App) -> Vec<Row<'static>> {
    let mut rows: Vec<Row> = Vec::new();
    for endpoint_name in &app.endpoint_order {
        let Some(state) = app.endpoint_states.get(endpoint_name) else {
            continue;
        };

        let Some(status) = &state.latest_status else {
            continue;
        };

        let Some(latency) = &state.latest_latency else {
            continue;
        };

        // If we reach this point, we are guaranteed to have
        // 'state', 'status', and 'latency' so we add them to the Rows.
        let status_message = match status {
            CheckStatus::Success { code, text } => format!("{} {}", code, text),
            CheckStatus::Error { message } => format!("Error {}", message),
        };
        let latency_message = format!("{}ms", latency.as_millis());

        let latency_length = state.latency_history.len();
        let start = latency_length.saturating_sub(SPARKLINE_LENGTH);
        let latency_slice: Vec<u64> = state.latency_history.iter().skip(start).copied().collect();
        let sparkline = generate_sparkline_string(&latency_slice);

        rows.push(
            Row::new(vec![
                Cell::from(state.name.clone()),
                Cell::from(status_message),
                Cell::from(latency_message),
                Cell::from(sparkline),
            ])
            .height(1),
        );
    }
    rows
}

/// Helper function to show a welcome/help message
pub fn render_welcome_message(frame: &mut Frame) {
    let text = vec![
        Line::from("Welcome to Statui!").style(Style::default().bold()),
        Line::from(""), // Empty line
        Line::from("No endpoints are loaded."),
        Line::from("Please create a 'statui.toml' file in this directory"),
        Line::from("and add your endpoints to it."),
        Line::from(""),
        Line::from("Press 'q' to quit."),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Welcome").borders(Borders::ALL))
        .alignment(Alignment::Center); // Center the text

    // We need to calculate a centered area to render this
    let area = centered_rect(60, 50, frame.area());
    frame.render_widget(paragraph, area);
}

/// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Helper function to create sparkline strings
fn generate_sparkline_string(data: &[u64]) -> String {
    if data.is_empty() {
        return String::from(" ");
    }

    let max = data.iter().max().copied().unwrap_or(1).max(1);

    // We define the symbols manually here.
    let unicode_bars = symbols::bar::NINE_LEVELS;
    let bars = [
        unicode_bars.one_eighth,
        unicode_bars.one_quarter,
        unicode_bars.three_eighths,
        unicode_bars.half,
        unicode_bars.five_eighths,
        unicode_bars.three_quarters,
        unicode_bars.seven_eighths,
        unicode_bars.full,
    ];

    data.iter()
        .map(|&value| {
            if value == 0 {
                return bars[0];
            }

            // Calculate ratio (0.0 to 1.0)
            let ratio = value as f64 / max as f64;

            // Map 0.0-1.0 to index 0-7
            let index = (ratio * 7.0).round() as usize;

            // Clamp index to max 7 to prevent crashes
            bars[index.min(7)]
        })
        .collect()
}
