use crate::board::Coordinate;

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
