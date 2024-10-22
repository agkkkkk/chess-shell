use crate::{
    constants::{DisplayMode, BLACK, UNDEFINED_POSITION, WHITE},
    pieces::{ChessPiece, PieceColor, PieceMove},
    utils::get_chess_pieces,
};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    widgets::Block,
    Frame,
};
use uci::Engine;

pub type GameBoard = [[Option<(ChessPiece, PieceColor)>; 8]; 8];

#[derive(PartialEq, Clone, Debug, Eq, PartialOrd, Ord, Copy)]
pub struct Coordinate {
    pub row: u8,
    pub col: u8,
}

impl Coordinate {
    pub fn new<U1: Into<u8>, U2: Into<u8>>(row: U1, col: U2) -> Self {
        Coordinate {
            row: row.into(),
            col: col.into(),
        }
    }

    pub fn opt_new<U1: TryInto<u8>, U2: TryInto<u8>>(row: U1, col: U2) -> Option<Self> {
        let row: u8 = row.try_into().ok()?;
        let col: u8 = col.try_into().ok()?;

        let coords = Coordinate { row, col };
        if coords.is_valid() {
            Some(coords)
        } else {
            None
        }
    }

    pub fn undefined() -> Self {
        Coordinate {
            row: UNDEFINED_POSITION,
            col: UNDEFINED_POSITION,
        }
    }

    pub fn is_valid(&self) -> bool {
        (0..8).contains(&self.row) && (0..8).contains(&self.col)
    }
}

impl std::ops::Index<&Coordinate> for GameBoard {
    type Output = Option<(ChessPiece, PieceColor)>;

    fn index(&self, index: &Coordinate) -> &Self::Output {
        &self[index.row as usize][index.col as usize]
    }
}

impl std::ops::IndexMut<&Coordinate> for GameBoard {
    fn index_mut(&mut self, index: &Coordinate) -> &mut Self::Output {
        &mut self[index.row as usize][index.col as usize]
    }
}

pub struct Board {
    pub board: GameBoard,
    pub cursor_coordinate: Coordinate,
    pub selected_coordinate: Coordinate,
    pub selected_piece_cursor: i8,
    pub old_cursor_position: Coordinate,
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

impl Default for Board {
    fn default() -> Self {
        Self {
            board: [
                [
                    Some((ChessPiece::Rook, PieceColor::Black)),
                    Some((ChessPiece::Knight, PieceColor::Black)),
                    Some((ChessPiece::Bishop, PieceColor::Black)),
                    Some((ChessPiece::Queen, PieceColor::Black)),
                    Some((ChessPiece::King, PieceColor::Black)),
                    Some((ChessPiece::Bishop, PieceColor::Black)),
                    Some((ChessPiece::Knight, PieceColor::Black)),
                    Some((ChessPiece::Rook, PieceColor::Black)),
                ],
                [
                    Some((ChessPiece::Pawn, PieceColor::Black)),
                    Some((ChessPiece::Pawn, PieceColor::Black)),
                    Some((ChessPiece::Pawn, PieceColor::Black)),
                    Some((ChessPiece::Pawn, PieceColor::Black)),
                    Some((ChessPiece::Pawn, PieceColor::Black)),
                    Some((ChessPiece::Pawn, PieceColor::Black)),
                    Some((ChessPiece::Pawn, PieceColor::Black)),
                    Some((ChessPiece::Pawn, PieceColor::Black)),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some((ChessPiece::Pawn, PieceColor::White)),
                    Some((ChessPiece::Pawn, PieceColor::White)),
                    Some((ChessPiece::Pawn, PieceColor::White)),
                    Some((ChessPiece::Pawn, PieceColor::White)),
                    Some((ChessPiece::Pawn, PieceColor::White)),
                    Some((ChessPiece::Pawn, PieceColor::White)),
                    Some((ChessPiece::Pawn, PieceColor::White)),
                    Some((ChessPiece::Pawn, PieceColor::White)),
                ],
                [
                    Some((ChessPiece::Rook, PieceColor::White)),
                    Some((ChessPiece::Knight, PieceColor::White)),
                    Some((ChessPiece::Bishop, PieceColor::White)),
                    Some((ChessPiece::Queen, PieceColor::White)),
                    Some((ChessPiece::King, PieceColor::White)),
                    Some((ChessPiece::Bishop, PieceColor::White)),
                    Some((ChessPiece::Knight, PieceColor::White)),
                    Some((ChessPiece::Rook, PieceColor::White)),
                ],
            ],
            cursor_coordinate: Coordinate::new(4, 4),
            selected_coordinate: Coordinate::undefined(),
            selected_piece_cursor: 0,
            old_cursor_position: Coordinate::undefined(),
            player_turn: PieceColor::White,
            move_history: vec![],
            draw: false,
            checkmate: false,
            promotion: false,
            promotion_cursor: 0,
            consecutive_non_pawn_or_capture: 0,
            engine: None,
            is_game_against_bot: false,
            display_mode: DisplayMode::Default,
        }
    }
}

impl Board {
    pub fn render_board(&self, area: Rect, frame: &mut Frame) {
        let width = area.width / 8;
        let height = area.height / 8;
        let border_height = area.height / 2 - (4 * height);
        let border_width = area.width / 2 - (4 * width);

        let board_columns = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(border_height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(border_height),
                ]
                .as_ref(),
            )
            .split(area);

        for i in 0..8u8 {
            let lines = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(border_width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(border_width),
                    ]
                    .as_ref(),
                )
                .split(board_columns[i as usize + 1]);

            for j in 0..8u8 {
                let mut cell_color = if ((i + j) % 2 == 0) { WHITE } else { BLACK };

                let square = lines[j as usize + 1];

                let mut cell = Block::default();
                cell = match self.display_mode {
                    DisplayMode::Default => cell.bg(cell_color),
                    DisplayMode::ASCII => match cell_color {
                        WHITE => cell.bg(Color::White).fg(Color::Black),
                        BLACK => cell.bg(Color::Black).fg(Color::White),
                        _ => cell.bg(cell_color),
                    },
                };

                frame.render_widget(cell.clone(), square);

                let coords = Coordinate::new(i, j);
                let piece = get_chess_pieces(self, &coords, square);

                frame.render_widget(piece, square);
            }
        }
    }
}
