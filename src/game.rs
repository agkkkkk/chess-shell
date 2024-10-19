use std::error;

use dirs::home_dir;

// Game result Type
pub type GameResult<T> = std::result::Result<T, Box<dyn error::Error>>;

// Game Struct
pub struct Game {
    pub running: bool, // is game running
    // pub board: Board,
    // pub screen: Screen,
    pub show_help: bool,                   // used to show popup help
    pub cursor: u8,                        // menu current cursor
    pub chess_engine_path: Option<String>, // path of chess engine
}

impl Default for Game {
    fn default() -> Self {
        Self {
            running: true,
            show_help: false,
            cursor: 0,
            chess_engine_path: None,
        }
    }
}
