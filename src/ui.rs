use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};

use crate::{
    constants::{DisplayMode, TITLE},
    game::{self, Game},
};

pub fn render(game: &mut Game, frame: &mut Frame) {
    let main_area = frame.area();

    // render_menu_ui(frame, game, main_area);

    render_game_ui(frame, game, main_area);
}

pub fn render_menu_ui(frame: &mut Frame, game: &Game, main_area: Rect) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 1),
                Constraint::Ratio(1, 5),
                Constraint::Ratio(3, 5),
            ]
            .as_ref(),
        )
        .split(main_area);

    let title = Paragraph::new(TITLE)
        .alignment(Alignment::Center)
        .block(Block::default());

    frame.render_widget(title, main_layout[0]);

    let display_mode_menu = {
        let display_mode = match game.board.display_mode {
            DisplayMode::Default => "Default",
            DisplayMode::ASCII => "ASCII",
        };
        format!("Display mode: {display_mode}")
    };

    let menu_items = ["Normal Game", "Play with Bot", &display_mode_menu, "Help"];

    let mut menu_body: Vec<Line<'_>> = vec![];

    for (i, menu_item) in menu_items.iter().enumerate() {
        menu_body.push(Line::from(""));

        let mut text = if game.cursor == i as u8 {
            "> ".to_string()
        } else {
            String::new()
        };

        text.push_str(menu_item);
        menu_body.push(Line::from(text));
    }

    let menu = Paragraph::new(menu_body)
        .bold()
        .alignment(Alignment::Center)
        .block(Block::default());

    frame.render_widget(menu, main_layout[2]);
}

pub fn render_game_ui(frame: &mut Frame, game: &Game, main_area: Rect) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 18),
                Constraint::Ratio(16, 18),
                Constraint::Ratio(1, 18),
            ]
            .as_ref(),
        )
        .split(main_area);

    let board_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(2, 17),
                Constraint::Ratio(9, 17),
                Constraint::Ratio(1, 17),
                Constraint::Ratio(5, 17),
            ]
            .as_ref(),
        )
        .split(main_layout[1]);

    let board_block = Block::default().style(Style::default());

    frame.render_widget(board_block, board_layout[1]);

    game.board.render_board(board_layout[1], frame);
}
