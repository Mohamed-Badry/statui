use ratatui::{
    layout::Constraint,
    prelude::*,
    widgets::{
        Block, Borders, Cell, Paragraph, Row, Table,
    },
};

use crate::app::App;
use crate::{
    backend::CheckStatus,
};

/// Renders the UI widgets for the application.
pub fn render_ui(frame: &mut Frame, app: &App) {
    if app.endpoint_order.is_empty() {
        render_welcome_message(frame);
        return;
    }

    render_table(frame, &app)
}

fn render_table(frame: &mut Frame, app: &App) {
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

    let table = Table::new(rows, widths).header(header).block(
        Block::bordered()
            .title(
                Line::from("Statui ")
                    .left_aligned()
                    .style(Style::new().blue().italic()),
            )
            .border_set(symbols::border::DOUBLE),
    );
    let area = centered_rect(99, 99, frame.area());
    frame.render_widget(table, area);
}

/// Return the endpoints as a vector of Rows to build the table.
fn create_rows(app: &App) -> Vec<Row<'_>> {
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

        rows.push(Row::new(vec![
            state.name.clone(),
            status_message,
            latency_message,
        ]));
    }
    rows
}


/// Helper function to show a welcome/help message
fn render_welcome_message(frame: &mut Frame) {
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
