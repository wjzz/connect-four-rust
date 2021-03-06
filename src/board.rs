use crate::types::*;
use rand::prelude::*;

pub type Board = [Piece; SIZE];
type Counts = [usize; COLS];

pub static mut LINES: Vec<Vec<usize>> = vec![];
pub static mut LINES_BY_INDEX: Vec<Vec<Vec<usize>>> = vec![];
pub static mut ZOBRIST: [[usize; SIZE]; 2] = [[0; SIZE]; 2];

pub struct ArrayPosition {
    pub board: Board,
    pub counts: Counts,
    pub to_play: Player,
    pub move_count: usize,
    pub hash: usize,
}

// Board layout (for COLS=7, ROWS=6)

//  00 01 02 03 04 05 06
//  07 08 09 10 11 12 13
//  14 15 16 17 18 19 20
//  21 22 23 24 25 26 27
//  28 29 30 31 32 33 34
//  35 36 37 38 39 40 41

impl Position for ArrayPosition {
    fn new() -> ArrayPosition {
        let board = [Piece::Empty; SIZE];
        let counts = [0; COLS];
        let to_play = Player::Black;
        let move_count = 0;
        let hash = 0;
        ArrayPosition {
            board,
            counts,
            to_play,
            move_count,
            hash,
        }
    }

    fn current_player(self: &Self) -> Player {
        self.to_play
    }

    fn hash(self: &Self) -> usize {
        self.hash
    }

    fn symm_hash(self: &Self) -> usize {
        unsafe {
            let mut hash = 0;
            for row in 0..ROWS {
                for col in 0..COLS {
                    let index = rowcol2index(row, col);
                    let piece = self.board[index];
                    if piece != Piece::Empty {
                        let player = piece as usize - 1;
                        let sym_index = rowcol2index(row, COLS - 1 - col);
                        hash ^= ZOBRIST[player][sym_index];
                    }
                }
            }
            hash
        }
    }

    fn get_lines_count(self: &ArrayPosition, mv: Move) -> i32 {
        let index = rowcol2index(self.counts[mv], mv);
        let mut count = 0;
        let pp = Piece::from_player(self.to_play);
        unsafe {
            for line in &LINES_BY_INDEX[index] {
                let brd = self.board;
                count += get_line_fitness(
                    pp,
                    brd,
                    line
                );
            }
        }
        return count;
    }

    fn duplicate(self: &Self) -> ArrayPosition {
        let board = self.board.clone();
        let counts = self.counts.clone();
        let to_play = self.to_play;
        let move_count = self.move_count;
        let hash = self.hash;
        ArrayPosition {
            board,
            counts,
            to_play,
            move_count,
            hash,
        }
    }

    fn ascii(self: &Self) -> String {
        let mut s = String::new();

        let header = " 1 2 3 4 5 6 7\n";
        s += "\n";
        s += header;
        s += "\n";

        for row in (0..ROWS).rev() {
            s += &format!(" ");
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
            s += &format!(" ");
            s += "\n";
        }
        s += "\n";
        s += header;

        s
    }

    fn is_move_legal(self: &Self, mv: Move) -> bool {
        self.counts[mv] < ROWS
    }

    fn moves(self: &Self) -> Vec<Move> {
        let mut legal_moves = vec![];

        for i in 0..COLS {
            if self.counts[i] < ROWS {
                legal_moves.push(i);
            }
        }

        legal_moves
    }

    fn make_move(self: &mut Self, mv: Move) {
        debug_assert!(mv < COLS);
        debug_assert!(self.counts[mv] < ROWS);

        let row = self.counts[mv];
        let index = rowcol2index(row, mv);
        self.board[index] = Piece::from_player(self.to_play);
        self.counts[mv] += 1;
        self.move_count += 1;
        unsafe {
            self.hash ^= ZOBRIST[self.to_play as usize][index];
        }
        self.to_play = self.to_play.other();
    }

    fn unmake_move(self: &mut Self, mv: Move) {
        debug_assert!(self.counts[mv] > 0);

        self.counts[mv] -= 1;
        let row = self.counts[mv];
        let index = rowcol2index(row, mv);
        self.board[index] = Piece::Empty;
        self.to_play = self.to_play.other();
        unsafe {
            self.hash ^= ZOBRIST[self.to_play as usize][index];
        }
        self.move_count -= 1;
    }

    fn result(self: &Self) -> Option<GameResult> {
        if self.move_count < 7 {
            None
        } else if is_win(self.board, self.to_play.other()) {
            Some(GameResult::Win(self.to_play.other()))
        } else if self.move_count == SIZE {
            Some(GameResult::Draw)
        } else {
            None
        }
    }

    fn legal_move_count(self: &Self) -> usize {
        let mut result = 0;

        for count in self.counts {
            if count < ROWS {
                result += 1
            }
        }

        result
    }
}

fn get_line_fitness(pp: Piece, brd: Board, line: &Vec<usize>) -> i32 {
    let mut line_count = 0;
    for &i in line {
        if brd[i] == pp {
            line_count += 1;
        } else if brd[i] != Piece::Empty {
            line_count -= 1;
        }
    }
    // sure win
    if line_count == 3 {
        return 10_000;
    // preventing opp win
    } else if line_count == -3 {
        return 5_000;
    } else if line_count == 2 {
        return 100;
    } else {
        return 0;
    }
}
fn is_win(board: Board, player: Player) -> bool {
    let pp = Piece::from_player(player);

    for row in 0..ROWS {
        for col in 0..COLS {
            let base = rowcol2index(row, col);
            if board[base] == pp {
                // row
                if col + 3 < COLS {
                    let d = 1;
                    if pp == board[base + d]
                        && pp == board[base + 2 * d]
                        && pp == board[base + 3 * d]
                    {
                        return true;
                    }
                }
                // column
                if row + 3 < ROWS {
                    let d = COLS;
                    if pp == board[base + d]
                        && pp == board[base + 2 * d]
                        && pp == board[base + 3 * d]
                    {
                        return true;
                    }
                }
                // rising diagonal
                if row + 3 < ROWS && col >= 3 {
                    let d = COLS - 1;
                    if pp == board[base + d]
                        && pp == board[base + 2 * d]
                        && pp == board[base + 3 * d]
                    {
                        return true;
                    }
                }
                // decreasing diagonal
                if row + 3 < ROWS && col + 3 < COLS {
                    let d = COLS + 1;
                    if pp == board[base + d]
                        && pp == board[base + 2 * d]
                        && pp == board[base + 3 * d]
                    {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn initialize_lines() {
    unsafe {
        for row in 0..ROWS {
            for col in 0..COLS {
                let base = rowcol2index(row, col);
                // rows
                if col + 3 < COLS {
                    let d = 1;
                    LINES.push(vec![base, base + d, base + 2 * d, base + 3 * d]);
                }
                // column
                if row + 3 < ROWS {
                    let d = COLS;
                    LINES.push(vec![base, base + d, base + 2 * d, base + 3 * d]);
                }
                // rising diagonal
                if row + 3 < ROWS && col >= 3 {
                    let d = COLS - 1;
                    LINES.push(vec![base, base + d, base + 2 * d, base + 3 * d]);
                }
                // decreasing diagonal
                if row + 3 < ROWS && col + 3 < COLS {
                    let d = COLS + 1;
                    LINES.push(vec![base, base + d, base + 2 * d, base + 3 * d]);
                }
            }
        }
    }
}

fn initialize_lines_by_index() {
    unsafe {
        for row in 0..ROWS {
            for col in 0..COLS {
                let base = rowcol2index(row, col);
                let mut v = vec![];

                for line in &LINES {
                    if line[0] == base || line[1] == base || line[2] == base || line[3] == base {
                        v.push(line.clone());
                    }
                }
                LINES_BY_INDEX.push(v);
            }
        }
    }
}

fn initialize_hashes() {
    let mut rng = thread_rng();
    unsafe {
        for player in 0..2 {
            for i in 0..SIZE {
                ZOBRIST[player][i] = rng.gen();
            }
        }
    }
}

pub fn initialize() {
    initialize_lines();
    initialize_lines_by_index();
    initialize_hashes();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn moves_count_empty_board() {
        let pos = ArrayPosition::new();

        let move_count = pos.moves().len();

        assert_eq!(move_count, COLS);
    }

    #[test]
    pub fn moves_count_made_moves() {
        let mut pos = ArrayPosition::new();
        for _ in 0..ROWS {
            pos.make_move(0);
        }
        let move_count = pos.moves().len();

        assert_eq!(move_count, COLS - 1);
    }

    #[test]
    pub fn example_win_row() {
        let mut pos = ArrayPosition::new();
        pos.make_move(0);
        pos.make_move(0);
        pos.make_move(1);
        pos.make_move(1);
        pos.make_move(2);
        pos.make_move(2);
        pos.make_move(3);

        assert!(pos.result() == Some(GameResult::Win(Player::Black)));
    }

    #[test]
    pub fn example_win_col() {
        let mut pos = ArrayPosition::new();
        pos.make_move(0);
        pos.make_move(1);
        pos.make_move(0);
        pos.make_move(1);
        pos.make_move(0);
        pos.make_move(1);
        pos.make_move(0);

        assert!(pos.result() == Some(GameResult::Win(Player::Black)));
    }

    #[test]
    pub fn example_win_dia() {
        let mut pos = ArrayPosition::new();
        pos.make_move(0);
        pos.make_move(1);

        pos.make_move(1);
        pos.make_move(2);

        pos.make_move(3);
        pos.make_move(2);

        pos.make_move(2);
        pos.make_move(3);

        pos.make_move(3);
        pos.make_move(0);

        pos.make_move(3);

        assert!(pos.result() == Some(GameResult::Win(Player::Black)));
    }

    #[test]
    pub fn example_win_dia2() {
        let mut pos = ArrayPosition::new();
        pos.make_move(3);
        pos.make_move(2);

        pos.make_move(2);
        pos.make_move(1);

        pos.make_move(0);
        pos.make_move(1);

        pos.make_move(1);
        pos.make_move(0);

        pos.make_move(0);
        pos.make_move(3);

        pos.make_move(0);

        assert!(pos.result() == Some(GameResult::Win(Player::Black)));
    }
}
