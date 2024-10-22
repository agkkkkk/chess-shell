use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, Padding, Paragraph},
};

use crate::{
    board::{Board, Coordinate, GameBoard},
    constants::DisplayMode,
    pieces::{ChessPiece, PieceColor},
};

pub fn get_chess_pieces<'a>(
    board: &'a Board,
    cell_coordinates: &'a Coordinate,
    rect: Rect,
) -> Paragraph<'a> {
    let piece_color = get_piece_color(board.board, cell_coordinates);
    let piece_type = get_piece_type(board.board, cell_coordinates);
    let piece_enum = ChessPiece::piece_type_to_string_enum(piece_type, &board.display_mode);

    let paragraph = match board.display_mode {
        DisplayMode::Default => {
            let color_enum = color_enum(piece_color);

            // Place piece on board
            Paragraph::new(piece_enum).fg(color_enum)
        }
        DisplayMode::ASCII => {
            let paragraph = match piece_color {
                Some(PieceColor::Black) => Paragraph::new(piece_enum.to_lowercase()),
                Some(PieceColor::White) => Paragraph::new(piece_enum.to_lowercase().underlined()),
                None => Paragraph::new(piece_enum),
            };

            // Place piece on board
            paragraph.block(Block::new().padding(Padding::vertical(rect.height / 2)))
        }
    };

    paragraph.alignment(Alignment::Center)
}

pub fn get_piece_color(board: GameBoard, coordinate: &Coordinate) -> Option<PieceColor> {
    if !coordinate.is_valid() {
        return None;
    }

    board[coordinate].map(|(_, piece_color)| piece_color)
}

pub fn get_piece_type(board: GameBoard, coordinate: &Coordinate) -> Option<ChessPiece> {
    if !coordinate.is_valid() {
        return None;
    }
    board[coordinate].map(|(piece_type, _)| piece_type)
}

pub fn color_enum(piece_color: Option<PieceColor>) -> Color {
    match piece_color {
        Some(PieceColor::Black) => Color::Black,
        Some(PieceColor::White) => Color::White,
        None => Color::Red,
    }
}
