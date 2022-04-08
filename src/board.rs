use crate::types::*;

pub type Board = [Piece; SIZE];

static mut LINES: Vec<Vec<usize>> = vec![];

pub struct ArrayPosition {
    pub board: Board,
    pub to_play: Player,
    pub move_count: usize,
}

impl Position for ArrayPosition {
    fn new() -> ArrayPosition {
        let board = [Piece::Empty; SIZE];
        let to_play = Player::Black;
        let move_count = 0;
        ArrayPosition {
            board,
            to_play,
            move_count,
        }
    }

    fn current_player(self: &Self) -> Player {
        self.to_play
    }

    fn duplicate(self: &Self) -> ArrayPosition {
        let board = self.board.clone();
        let to_play = self.to_play;
        let move_count = self.move_count;
        ArrayPosition {
            board,
            to_play,
            move_count,
        }
    }

    fn ascii(self: &Self) -> String {
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

    fn moves(self: &Self) -> Vec<Move> {
        let mut legal_moves = vec![];

        for i in 0..SIZE {
            if self.board[i] == Piece::Empty {
                legal_moves.push(i);
            }
        }

        legal_moves
    }

    fn make_move(self: &mut Self, mv: Move) {
        debug_assert!(mv < SIZE);
        debug_assert_eq!(self.board[mv], Piece::Empty);

        self.board[mv] = Piece::from_player(self.to_play);
        self.to_play = self.to_play.other();
        self.move_count += 1;
    }

    fn unmake_move(self: &mut Self, mv: Move) {
        debug_assert!(mv < SIZE);

        self.board[mv] = Piece::Empty;
        self.to_play = self.to_play.other();
        self.move_count -= 1;
    }

    fn result(self: &Self) -> Option<GameResult> {
        // TODO: use `is_win_smarter` once it's fully implemented
        if self.move_count < 9 {
            None
        } else if is_win(self.board, self.to_play.other()) {
            Some(GameResult::Win(self.to_play.other()))
        } else if self.move_count == SIZE {
            Some(GameResult::Draw)
        } else {
            None
        }
    }

    fn move_count(self: &Self) -> usize {
        let mut result = 0;
        for i in 0..SIZE {
            if self.board[i] == Piece::Empty {
                result += 1;
            }
        }
        result
    }
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

fn is_win_smarter(board: Board, player: Player) -> bool {
    let pp = Piece::from_player(player);

    // rows
    for col in 0 .. COLS {
        let mut count = 0;
        for row in 0 .. (ROWS - 4) {
            let index = rowcol2index(row, col);
            if board[index] == pp {
                count += 1;
                if count == 5 {
                    return true;
                }
            } else {
                count = 0;
            }
        }
    }

    // cols
    for row in 0 .. ROWS {
        let mut count = 0;
        for col in 0 .. (COLS - 4) {
            let index = rowcol2index(row, col);
            if board[index] == pp {
                count += 1;
                if count == 5 {
                    return true;
                }
            } else {
                count = 0;
            }
        }
    }

    // TODO: diagonals

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn moves_count_empty_board() {
        let pos = ArrayPosition::new();

        let move_count = pos.moves().len();

        assert_eq!(move_count, SIZE);
    }

    #[test]
    pub fn moves_count_made_moves() {
        let mut pos = ArrayPosition::new();
        pos.make_move(0);
        pos.make_move(1);

        let move_count = pos.moves().len();

        assert_eq!(move_count, SIZE - 2);
    }
}
