use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::constants::TITLE;

pub fn render(frame: &mut Frame) {
    let main_area = frame.area();

    render_menu_ui(frame, main_area);
}

pub fn render_menu_ui(frame: &mut Frame, main_area: Rect) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 1),
                // Constraint::Ratio(1, 5),
                // Constraint::Ratio(3, 5),
            ]
            .as_ref(),
        )
        .split(main_area);

    let title = Paragraph::new(TITLE)
        .alignment(Alignment::Center)
        .block(Block::default());

    frame.render_widget(title, main_layout[0]);
}
