use crate::board::{rowcol2index, Move, Piece, Player, GameResult, COLS, ROWS, SIZE};

#[allow(non_camel_case_types)]
type u256 = [u128; 2];

type ColorBoard = u256;
pub type BitBoard = [ColorBoard; 2];

type BitPlayer = usize;
const PL_BLACK: BitPlayer = 0;
const PL_WHITE: BitPlayer = 1;

pub static mut WIN_PATTERNS: Vec<ColorBoard> = vec![];

pub fn other_player(player: BitPlayer) -> BitPlayer {
    1 - player
}

fn to_player(player: BitPlayer) -> Player {
    if player == PL_BLACK {
        Player::Black
    } else {
        Player::White
    }
}

pub struct BitPosition {
    board: BitBoard,
    to_play: BitPlayer,
    move_count: usize,
}

impl BitPosition {
    pub fn new() -> BitPosition {
        let board = [[0, 0], [0, 0]];
        let to_play = PL_BLACK;
        let move_count = 0;

        BitPosition {
            board,
            to_play,
            move_count,
        }
    }

    pub fn duplicate(self: &Self) -> BitPosition {
        let brd = self.board;
        let board = [
            [brd[PL_BLACK][0], brd[PL_BLACK][1]],
            [brd[PL_WHITE][0], brd[PL_WHITE][1]],
        ];
        let to_play = self.to_play;
        let move_count = self.move_count;

        BitPosition {
            board,
            to_play,
            move_count,
        }
    }

    pub fn get_piece_at(self: &Self, field: usize) -> Piece {
        let page = field / 128;
        let index = field % 128;
        if self.board[PL_BLACK][page] & 1 << index != 0 {
            Piece::Black
        } else if self.board[PL_WHITE][page] & 1 << index != 0 {
            Piece::White
        } else {
            Piece::Empty
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
                let ch = match self.get_piece_at(base) {
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

    pub fn make_move(self: &mut Self, mv: Move) {
        let player = self.to_play;

        let page = mv / 128;
        let index = mv % 128;
        self.board[player][page] |= 1 << index;

        self.to_play = other_player(player);
        self.move_count += 1;
    }

    pub fn unmake_move(self: &mut Self, mv: Move) {
        let player = other_player(self.to_play);

        let page = mv / 128;
        let index = mv % 128;
        self.board[player][page] &= !(1 << index);

        self.to_play = player;
        self.move_count -= 1;
    }

    // TODO: we could store the move list as a single color board
    pub fn moves(self: &Self) -> Vec<Move> {
        let mut result = vec![];

        let player = self.to_play;
        let opp = other_player(player);

        let board1 = self.board[player];
        let board2 = self.board[opp];

        let empty_fields: ColorBoard = [!(board1[0] | board2[0]), !(board1[1] | board2[1])];

        for i in 0..128 {
            if 1 << i & empty_fields[0] != 0 {
                result.push(i);
            }
        }

        for i in 128..SIZE {
            if 1 << (i - 128) & empty_fields[1] != 0 {
                result.push(i);
            }
        }

        result
    }

    pub fn is_finished(self: &Self) -> Option<GameResult> {
        let opp = other_player(self.to_play);
        if is_win(self.board[opp]) {
            Some(GameResult::Win(to_player(opp)))
        } else if self.move_count == SIZE {
            Some(GameResult::Draw)
        } else {
            None
        }
    }

    pub fn perft(self: &mut Self, depth: usize) -> usize {
        if depth == 0 {
            return 1;
        }

        let mut result = 0;

        if depth > 0 && self.is_finished().is_none() {
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

fn line(points: [usize; 5]) -> ColorBoard {
    let mut result: ColorBoard = [0; 2];

    for index in points {
        let page = index / 128;
        let index = index % 128;
        result[page] |= 1 << index;
    }

    result
}

pub fn initialize_winning_patterns() {
    unsafe {
        for row in 0..ROWS {
            for col in 0..COLS {
                let index = rowcol2index(row, col);
                // row
                if col + 4 < COLS {
                    let d = 1;

                    WIN_PATTERNS.push(line([
                        index,
                        index + d,
                        index + 2 * d,
                        index + 3 * d,
                        index + 4 * d,
                    ]));
                }
                // column
                if row + 4 < ROWS {
                    let d = 15;
                    WIN_PATTERNS.push(line([
                        index,
                        index + d,
                        index + 2 * d,
                        index + 3 * d,
                        index + 4 * d,
                    ]));
                }
                // rising diagonal
                if row + 4 < ROWS && col >= 4 {
                    let d = 14;
                    WIN_PATTERNS.push(line([
                        index,
                        index + d,
                        index + 2 * d,
                        index + 3 * d,
                        index + 4 * d,
                    ]));
                }
                // decreasing diagonal
                if row + 4 < ROWS && col + 4 < COLS {
                    let d = 16;
                    WIN_PATTERNS.push(line([
                        index,
                        index + d,
                        index + 2 * d,
                        index + 3 * d,
                        index + 4 * d,
                    ]));
                }
            }
        }
    }
}

pub fn is_win(player_board: ColorBoard) -> bool {
    unsafe {
        debug_assert!(!WIN_PATTERNS.is_empty());

        for win in &WIN_PATTERNS {
            if (win[0] & player_board[0] == win[0]) && (win[1] & player_board[1] == win[1]) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_on_empty_board() {
        let board = BitPosition::new();
        let moves = board.moves();

        assert_eq!(moves.len(), SIZE);
    }

    #[test]
    fn move_count_after_a_few_moves() {
        let mut board = BitPosition::new();
        board.make_move(0);
        board.make_move(15);
        board.make_move(15 * 5);
        println!("{}", board.ascii());

        let moves = board.moves();
        assert_eq!(moves.len(), SIZE - 3);
    }

    #[test]
    fn move_count_after_a_few_moves_and_unmakes() {
        let mut board = BitPosition::new();
        board.make_move(0);
        board.make_move(100);
        board.make_move(200);
        board.unmake_move(200);
        board.unmake_move(100);
        board.unmake_move(0);
        println!("{}", board.ascii());

        let moves = board.moves();
        assert_eq!(moves.len(), SIZE);
    }
}
