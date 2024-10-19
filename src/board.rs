use crate::{
    constants::DisplayMode,
    pieces::{ChessPiece, PieceColor, PieceMove},
};

use uci::Engine;

pub type GameBoard = [[Option<(ChessPiece, PieceColor)>; 8]; 8];

#[derive(PartialEq, Clone, Debug, Eq, PartialOrd, Ord, Copy)]
pub struct Coordinate {
    pub row: u8,
    pub col: u8,
}

pub struct Board {
    pub board: GameBoard,
    pub cursor_coordinate: Coordinate,
    pub selected_coordinate: Coordinate,
    pub selected_piece_cursor: i8,
    pub old_curosr_position: Coordinate,
    pub player_turn: PieceColor,
    pub move_history: Vec<PieceMove>,
    pub draw: bool,
    pub checkmate: bool,
    pub promotion: bool,
    pub promotion_cursor: i8,
    pub consecutive_non_pawn_or_capture: i32,
    pub engine: Option<Engine>,
    pub is_game_against_bot: bool,
    pub display_mode: DisplayMode,
}
