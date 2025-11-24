use ratatui::{
    Frame,
    layout::Alignment,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::ui::util;

pub fn render_welcome_message(frame: &mut Frame) {
    let text = vec![
        Line::from("Welcome to Statui!").style(Style::default().bold()),
        Line::from(""),
        Line::from("No endpoints are loaded."),
        Line::from("Please create a 'statui.toml' file in this directory"),
        Line::from("and add your endpoints to it."),
        Line::from(""),
        Line::from("Press 'q' to quit."),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Welcome").borders(Borders::ALL))
        .alignment(Alignment::Center);

    // We need to calculate a centered area to render this
    let area = util::centered_rect(60, 50, frame.area());
    frame.render_widget(paragraph, area);
}
