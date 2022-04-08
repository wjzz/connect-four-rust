pub use crate::position::Position;

pub const ROWS: usize = 15;
pub const COLS: usize = 15;
pub const SIZE: usize = ROWS * COLS;

pub fn rowcol2index(row: usize, col: usize) -> usize {
    ROWS * row + col
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn other(self: Self) -> Player {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Piece {
    Empty,
    Black,
    White,
}

impl Piece {
    pub fn from_player(player: Player) -> Piece {
        match player {
            Player::Black => Piece::Black,
            Player::White => Piece::White,
        }
    }
}

pub type Move = usize;

pub fn parse_move(s: String) -> Option<Move> {
    let chars: Vec<char> = s.to_ascii_uppercase().chars().collect();
    let col = (chars[0] as u8 - ('A' as u8)) as usize;
    let row: usize = 15 - String::from(&s[1..]).parse::<usize>().unwrap();
    println!("<s> => (<{}>,<{}>)", row, col);

    Some(rowcol2index(row, col))
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GameResult {
    Draw,
    Win(Player),
}
