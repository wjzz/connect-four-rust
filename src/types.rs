pub use crate::position::Position;

pub const ROWS: usize = 7;
pub const COLS: usize = 6;
pub const SIZE: usize = ROWS * COLS;

pub fn rowcol2index(row: usize, col: usize) -> usize {
    COLS * row + col
}

pub fn colrow2index(col: usize, row: usize) -> usize {
    (ROWS + 1) * col + row
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

    pub fn from_usize(pl: usize) -> Player {
        if pl == 0 {
            Player::Black
        } else {
            Player::White
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

// A move is the column number
pub type Move = usize;

pub fn parse_move(s: String) -> Move {
    return s.parse::<usize>().unwrap() - 1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GameResult {
    Draw,
    Win(Player),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rowcol() {
        for row in 0..ROWS {
            for col in 0..COLS {
                let index = rowcol2index(row, col);
                assert!(index < SIZE);
            }
        }
    }
}
