use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
};

use crate::{backend::CheckStatus, ui::theme::Theme};

/// Helper function to create a centered rectangle
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

/// Helper function to return a centered area with dynamic margins
pub fn centered_area(height: u16, width: u16, r: Rect) -> Rect {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(width),
            Constraint::Min(0),
        ])
        .split(layout[1])[1]
}

/// Helper function to wrap titles with brackets
pub fn wrap_with_brackets(title: &str, title_style: Style, bracket_style: Style) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{} ", Theme::TITLE_BRACKETS[0]), bracket_style),
        Span::styled(title.to_owned(), title_style),
        Span::styled(format!(" {}", Theme::TITLE_BRACKETS[1]), bracket_style),
    ])
}

/// Helper function to get the color for a given status based on the code.
pub fn get_status_color(status: &Option<CheckStatus>) -> Color {
    match status {
        Some(CheckStatus::Success { code, .. }) => Theme::color_code(code),
        Some(CheckStatus::Error { .. }) => Theme::STATUS_ERROR,
        None => Theme::BORDER_UNFOCUSED,
    }
}
