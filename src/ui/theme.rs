use std::time::Duration;

use ratatui::{style::{Color, Modifier, Style}, symbols::border};

// I should probably move these somewhere else but I'll
// keep them here for now
pub const VERY_POOR_LATENCY: u128 = 500;
pub const POOR_LATENCY: u128 = 200;

pub struct Theme;

impl Theme {
    // Backgrounds
    pub const APP_BG: Color = Color::Reset;
    pub const PANEL_BG: Color = Color::Reset;

    // Borders
    pub const BORDER_UNFOCUSED: Color = Color::DarkGray;
    pub const BORDER_FOCUSED: Color = Color::Cyan;
    pub const PANEL_BORDER: border::Set = border::THICK;

    // Table
    pub fn table_header() -> Style {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    }

    pub const HIGHLIGHT_SYMBOL: &str = ">> ";
    pub fn table_highlight() -> Style {
        Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Black)
            // .add_modifier(Modifier::REVERSED)
    }

    pub fn table_border_style() -> Style {
        Style::default()
            .fg(Theme::BORDER_FOCUSED)
    }

    // Status indicators
    pub const STATUS_OK: Color = Color::Green;
    pub const STATUS_REDIRECT: Color = Color::Blue;
    pub const STATUS_WARN: Color = Color::Yellow;
    pub const STATUS_ERROR: Color = Color::Red;

    pub fn latency_color(latency: &Duration) -> Color {
        if latency.as_millis() > VERY_POOR_LATENCY {
            Theme::STATUS_ERROR
        } else if latency.as_millis() > POOR_LATENCY {
            Theme::STATUS_WARN
        } else {
            Theme::STATUS_OK
        }
    }

    pub fn color_code(code: &u16) -> Color {
        match code {
            200..=299 => Theme::STATUS_OK,
            300..=399 => Theme::STATUS_REDIRECT,
            _ => Theme::STATUS_ERROR,
        }
    }

    // Footer
    pub const KEYBINDING_FG: Color = Color::Black;
    pub const KEYBINDING_BG: Color = Color::Cyan;

    pub fn footer_keys() -> Style {
        Style::default()
            .fg(Theme::KEYBINDING_FG)
            .bg(Theme::KEYBINDING_BG)
            .add_modifier(Modifier::BOLD)
    }

    pub const DESC_FG: Color = Color::DarkGray;
    pub const DESC_BG: Color = Color::Reset;

    pub fn footer_desc() -> Style {
        Style::default().fg(Theme::DESC_FG).bg(Theme::DESC_BG)
    }
}
