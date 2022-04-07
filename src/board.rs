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

pub const ROWS: usize = 15;
pub const COLS: usize = 15;
pub const SIZE: usize = ROWS * COLS;

pub type Board = [Piece; SIZE];

pub struct Position {
    pub board: Board,
    pub to_play: Player,
    pub move_count: usize,
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

pub fn rowcol2index(row: usize, col: usize) -> usize {
    ROWS * row + col
}

fn is_win(board: Board, player: Player) -> bool {
    let pp = Piece::from_player(player);

    for row in 0..ROWS {
        for col in 0..COLS {
            let base = rowcol2index(row, col);
            if board[base] == pp {
                // row
                if col + 4 < COLS {
                    let d = 1;
                    if pp == board[base + d]
                        && pp == board[base + 2 * d]
                        && pp == board[base + 3 * d]
                        && pp == board[base + 4 * d]
                        && pp == board[base + 4 * d]
                    {
                        return true;
                    }
                }
                // column
                if row + 4 < ROWS {
                    let d = 15;
                    if pp == board[base + d]
                        && pp == board[base + 2 * d]
                        && pp == board[base + 3 * d]
                        && pp == board[base + 4 * d]
                        && pp == board[base + 4 * d]
                    {
                        return true;
                    }
                }
                // rising diagonal
                if row + 4 < ROWS && col >= 4 {
                    let d = 14;
                    if pp == board[base + d]
                        && pp == board[base + 2 * d]
                        && pp == board[base + 3 * d]
                        && pp == board[base + 4 * d]
                        && pp == board[base + 4 * d]
                    {
                        return true;
                    }
                }
                // decreasing diagonal
                if row + 4 < ROWS && col + 4 < COLS {
                    let d = 16;
                    if pp == board[base + d]
                        && pp == board[base + 2 * d]
                        && pp == board[base + 3 * d]
                        && pp == board[base + 4 * d]
                        && pp == board[base + 4 * d]
                    {
                        return true;
                    }
                }
            }
        }
    }
    false
}

impl Position {
    pub fn new() -> Position {
        let board = [Piece::Empty; SIZE];
        let to_play = Player::Black;
        let move_count = 0;
        Position {
            board,
            to_play,
            move_count,
        }
    }

    pub fn duplicate(self: &Self) -> Position {
        let board = self.board.clone();
        let to_play = self.to_play;
        let move_count = self.move_count;
        Position {
            board,
            to_play,
            move_count,
        }
    }

    pub fn ascii(self: &Self) -> String {
        let mut s = String::new();

        let header = "   A B C D E F G H I J K L M N O\n";
        s += header;

        for row in 0..ROWS {
            let row_num = ROWS - row;
            s += &format!("{:2} ", row_num);
            for col in 0..COLS {
                let base = rowcol2index(row, col);
                let ch = match self.board[base] {
                    Piece::Empty => {
                        if row == 7 && col == 7 {
                            ','
                        } else {
                            '.'
                        }
                    }
                    Piece::Black => 'x',
                    Piece::White => 'o',
                };
                s += &format!("{} ", ch);
            }
            s += &format!("{:2} ", row_num);
            s += "\n";
        }
        s += header;

        s
    }

    pub fn move_count(self: &Self) -> usize {
        let mut result = 0;
        for i in 0..SIZE {
            if self.board[i] == Piece::Empty {
                result += 1;
            }
        }
        result
    }

    pub fn moves(self: &Self) -> Vec<Move> {
        let mut legal_moves = vec![];

        for i in 0..SIZE {
            if self.board[i] == Piece::Empty {
                legal_moves.push(i);
            }
        }

        legal_moves
    }

    pub fn make_move(self: &mut Self, mv: Move) {
        debug_assert!(mv < SIZE);
        debug_assert_eq!(self.board[mv], Piece::Empty);

        self.board[mv] = Piece::from_player(self.to_play);
        self.to_play = self.to_play.other();
        self.move_count += 1;
    }

    pub fn unmake_move(self: &mut Self, mv: Move) {
        debug_assert!(mv < SIZE);

        self.board[mv] = Piece::Empty;
        self.to_play = self.to_play.other();
        self.move_count -= 1;
    }

    pub fn is_finished(self: &Self) -> Option<GameResult> {
        if is_win(self.board, self.to_play.other()) {
            Some(GameResult::Win(self.to_play.other()))
        } else if self.move_count == SIZE {
            Some(GameResult::Draw)
        } else {
            None
        }
    }

    pub fn perft(self: &mut Self, depth: usize) -> usize {
        let mut result = 0;

        if self.is_finished().is_none() {
            if depth == 1 {
                return self.move_count();
            }
            let moves = self.moves();
            for mv in moves {
                self.make_move(mv);
                result += self.perft(depth - 1);
                self.unmake_move(mv);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn moves_count_empty_board() {
        let pos = Position::new();

        let move_count = pos.moves().len();

        assert_eq!(move_count, SIZE);
    }

    #[test]
    pub fn moves_count_made_moves() {
        let mut pos = Position::new();
        pos.make_move(0);
        pos.make_move(1);

        let move_count = pos.moves().len();

        assert_eq!(move_count, SIZE - 2);
    }
}
