use bishop::Bishop;
use king::King;
use knight::Knight;
use pawn::Pawn;
use queen::Queen;
use rook::Rook;

use crate::{board::Coordinate, constants::DisplayMode};

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ChessPiece {
    Pawn,
    Rook,
    Bishop,
    King,
    Queen,
    Knight,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PieceMove {
    pub piece_type: ChessPiece,
    pub from: Coordinate,
    pub to: Coordinate,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceColor {
    White = 0,
    Black = 1,
}

impl ChessPiece {
    pub fn piece_type_to_string_enum(
        piece_type: Option<ChessPiece>,
        display_mode: &DisplayMode,
    ) -> &'static str {
        match piece_type {
            Some(ChessPiece::Queen) => Queen::to_string(display_mode),
            Some(ChessPiece::King) => King::to_string(display_mode),
            Some(ChessPiece::Pawn) => Pawn::to_string(display_mode),
            Some(ChessPiece::Rook) => Rook::to_string(display_mode),
            Some(ChessPiece::Knight) => Knight::to_string(display_mode),
            Some(ChessPiece::Bishop) => Bishop::to_string(display_mode),
            None => " ",
        }
    }
}
