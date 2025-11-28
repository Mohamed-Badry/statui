use ratatui::{
    layout::Constraint,
    prelude::*,
    symbols,
    widgets::{Block, Cell, Row, Table},
};

use crate::{backend::CheckStatus, ui::theme::Theme};
use crate::{state::App, ui::util};

const SPARKLINE_LENGTH: usize = 15;

pub fn render_table(frame: &mut Frame, app: &mut App, chunk: Rect) {
    let header = Row::new(vec![
        Line::from("NAME").centered(),
        Line::from("STATUS").centered(),
        Line::from("LATENCY").centered(),
        Line::from("TREND").centered(),
    ])
    .style(
        Style::default()
            .fg(Theme::BORDER_FOCUSED)
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::UNDERLINED),
    );

    let rows = create_rows(&app);

    let widths = vec![
        Constraint::Percentage(30),
        Constraint::Percentage(25),
        Constraint::Percentage(15),
        Constraint::Percentage(30),
    ];

    let title =
        util::wrap_with_brackets("Statui", Theme::table_header(), Theme::table_border_style());

    let block = Block::bordered()
        .border_set(Theme::PANEL_BORDER)
        .border_style(Theme::table_border_style())
        .title(title)
        .title_alignment(Alignment::Left);

    let table = Table::new(rows, widths)
        .header(header)
        .block(block)
        .highlight_symbol(Theme::HIGHLIGHT_SYMBOL)
        .row_highlight_style(Theme::table_highlight());

    frame.render_stateful_widget(table, chunk, &mut app.table_state);
}

/// Return the endpoints as a vector of Rows to build the table.
fn create_rows(app: &App) -> Vec<Row<'static>> {
    let mut rows: Vec<Row> = Vec::new();
    let selected_idx = app.table_state.selected();

    for (i, endpoint_name) in app.endpoint_order.iter().enumerate() {
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
        // 'state', 'status', and 'latency' so we build up our rows.
        // Create an appropriate status message and get its color.
        let (status_message, status_color) = match status {
            CheckStatus::Success { code, text } => {
                let color = Theme::color_code(code);
                (format!("{:<3} {}", code, text), color)
            }
            CheckStatus::Error { message } => (format!("ERR {}", message), Theme::STATUS_ERROR),
        };

        let latency_message = format!("{}ms", latency.as_millis());
        let latency_color = Theme::latency_color(latency);

        // Take the last 'SPARKLINE_LENGTH' data points from the latency_history
        // and create a sparkline string.
        let latency_length = state.latency_history.len();
        let start = latency_length.saturating_sub(SPARKLINE_LENGTH);
        let latency_slice: Vec<u64> = state.latency_history.iter().skip(start).copied().collect();
        let sparkline = generate_sparkline_string(&latency_slice);

        // Handle selected row color reversal without reversing the sparkline cell
        let is_selected = Some(i) == selected_idx;
        let cell_style = if is_selected {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
        };

        rows.push(
            Row::new(vec![
                Cell::from(state.name.clone()).style(cell_style),
                Cell::from(Line::from(status_message).left_aligned())
                    .style(cell_style.fg(status_color)),
                Cell::from(Line::from(latency_message).centered())
                    .style(cell_style.fg(latency_color)),
                Cell::from(sparkline).fg(latency_color),
            ])
            .height(1),
        );
    }
    rows
}

/// Helper function to create sparkline strings
fn generate_sparkline_string(data: &[u64]) -> String {
    if data.is_empty() {
        return String::from(" ");
    }

    let max = data.iter().max().copied().unwrap_or(1).max(1);

    // Only using 7 out of 9 levels here so there's a small gap between
    // the sparklines in the rows I don't want empty levels.
    const N_LEVELS: usize = 7;

    // We define the symbols manually here.
    let unicode_bars = symbols::bar::NINE_LEVELS;
    let bars: [&str; N_LEVELS] = [
        unicode_bars.one_eighth,
        unicode_bars.one_quarter,
        unicode_bars.three_eighths,
        unicode_bars.half,
        unicode_bars.five_eighths,
        unicode_bars.three_quarters,
        unicode_bars.seven_eighths,
    ];

    data.iter()
        .map(|&value| {
            if value == 0 {
                return bars[0];
            }

            // Calculate ratio (0.0 to 1.0)
            let ratio = value as f64 / max as f64;

            // Map 0.0-1.0 to index 0-(N_LEVELS - 1)
            let index = (ratio * (N_LEVELS - 1) as f64).round() as usize;

            // Clamp index to max (N_LEVELS - 1) to prevent crashes
            bars[index.min(N_LEVELS - 1)]
        })
        .collect()
}
