use rand::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn opposite(self: Self) -> Player {
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
    board: Board,
    pub to_play: Player,
    move_count: usize,
}

pub type Move = usize;

pub fn parse_move(mut s: String) -> Option<Move> {
    s.pop(); // remove newline
    let chars: Vec<char> = s.to_ascii_uppercase().chars().collect();
    let col = (chars[0] as u8 - ('A' as u8)) as usize;
    let row: usize = 15 - String::from(&s[1..]).parse::<usize>().unwrap();
    println!("<s> => (<{}>,<{}>)", row, col);

    Some(rowcol2index(row, col))
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GameResult {
    Draw,
    Win(Player)
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
                    if pp == board[base+d] && pp == board[base+2*d] && pp == board[base+3*d] && pp == board[base+4*d] && pp == board[base+4*d] {
                        return true;
                    }
                }
                // column
                if row + 4 < ROWS {
                    let d = 15;
                    if pp == board[base+d] && pp == board[base+2*d] && pp == board[base+3*d] && pp == board[base+4*d] && pp == board[base+4*d] {
                        return true;
                    }
                }
                // rising diagonal
                if row + 4 < ROWS && col > 4 {
                    let d = 14;
                    if pp == board[base+d] && pp == board[base+2*d] && pp == board[base+3*d] && pp == board[base+4*d] && pp == board[base+4*d] {
                        return true;
                    }
                }
                // decreasing diagonal
                if row + 4 < ROWS && col + 4 < COLS {
                    let d = 16;
                    if pp == board[base+d] && pp == board[base+2*d] && pp == board[base+3*d] && pp == board[base+4*d] && pp == board[base+4*d] {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn has_open_fours(board: Board, player: Player) -> usize {
    let pp = Piece::from_player(player);

    for row in 0..ROWS {
        for col in 0..COLS {
            let base = rowcol2index(row, col);
            if board[base] == Piece::Empty {
                // row
                if col + 5 < COLS {
                    let d = 1;
                    if pp == board[base+d] && pp == board[base+2*d] && pp == board[base+3*d] && pp == board[base+4*d] && Piece::Empty == board[base+5*d] {
                        return 1;
                    }
                }
                // column
                if row + 5 < ROWS {
                    let d = 15;
                    if pp == board[base+d] && pp == board[base+2*d] && pp == board[base+3*d] && pp == board[base+4*d] && Piece::Empty == board[base+5*d] {
                        return 1;
                    }
                }
                // rising diagonal
                if row + 5 < ROWS && col > 5 {
                    let d = 14;
                    if pp == board[base+d] && pp == board[base+2*d] && pp == board[base+3*d] && pp == board[base+4*d] && Piece::Empty == board[base+5*d] {
                        return 1;
                    }
                }
                // decreasing diagonal
                if row + 5 < ROWS && col + 5 < COLS {
                    let d = 16;
                    if pp == board[base+d] && pp == board[base+2*d] && pp == board[base+3*d] && pp == board[base+4*d] && Piece::Empty == board[base+5*d] {
                        return 1;
                    }
                }
            }
        }
    }

    0
}

fn open_threes_count(board: Board, player: Player) -> usize {
    let mut result = 0;
    let pp = Piece::from_player(player);

    for row in 0..ROWS {
        for col in 0..COLS {
            let base = rowcol2index(row, col);
            if board[base] == Piece::Empty {
                // row
                if col + 4 < COLS {
                    let d = 1;
                    if pp == board[base+d] && pp == board[base+2*d] && pp == board[base+3*d] && Piece::Empty == board[base+4*d] {
                        result += 1;
                    }
                }
                // column
                if row + 4 < ROWS {
                    let d = 15;
                    if pp == board[base+d] && pp == board[base+2*d] && pp == board[base+3*d] && Piece::Empty == board[base+4*d] {
                        result += 1;
                    }
                }
                // rising diagonal
                if row + 4 < ROWS && col > 4 {
                    let d = 14;
                    if pp == board[base+d] && pp == board[base+2*d] && pp == board[base+3*d] && Piece::Empty == board[base+4*d] {
                        result += 1;
                    }
                }
                // decreasing diagonal
                if row + 4 < ROWS && col + 4 < COLS {
                    let d = 16;
                    if pp == board[base+d] && pp == board[base+2*d] && pp == board[base+3*d] && Piece::Empty == board[base+4*d] {
                        result += 1;
                    }
                }
            }
        }
    }

    result
}

fn open_twos_count(board: Board, player: Player) -> usize {
    let mut result = 0;
    let pp = Piece::from_player(player);

    for row in 0..ROWS {
        for col in 0..COLS {
            let base = rowcol2index(row, col);
            if board[base] == Piece::Empty {
                // row
                if col + 3 < COLS {
                    let d = 1;
                    if pp == board[base+d] && pp == board[base+2*d] && Piece::Empty == board[base+3*d] {
                        result += 1;
                    }
                }
                // column
                if row + 3 < ROWS {
                    let d = 15;
                    if pp == board[base+d] && pp == board[base+2*d] && Piece::Empty == board[base+3*d] {
                        result += 1;
                    }
                }
                // rising diagonal
                if row + 3 < ROWS && col > 3 {
                    let d = 14;
                    if pp == board[base+d] && pp == board[base+2*d] && Piece::Empty == board[base+3*d] {
                        result += 1;
                    }
                }
                // decreasing diagonal
                if row + 3 < ROWS && col + 3 < COLS {
                    let d = 16;
                    if pp == board[base+d] && pp == board[base+2*d] && Piece::Empty == board[base+3*d] {
                        result += 1;
                    }
                }
            }
        }
    }

    result
}

fn centralization(board: Board, player: Player) -> usize {
    let mut result = 0;
    let pp = Piece::from_player(player);

    for row in 0..ROWS {
        for col in 0..COLS {
            let base = rowcol2index(row, col);
            if board[base] == pp {
                result += 8 - (8 - (row as i32)).abs() + 8 - (8 - (col as i32)).abs();
            }
        }
    }
    result as usize
}


impl Position {
    pub fn new() -> Position {
        let board = [Piece::Empty; SIZE];
        let to_play = Player::Black;
        let move_count = 0;
        Position { board, to_play, move_count }
    }

    pub fn duplicate(self: &Self) -> Position {
        let board = self.board.clone();
        let to_play = self.to_play;
        let move_count = self.move_count;
        Position { board, to_play, move_count }
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
                    Piece::Empty =>
                    if row == 7 && col == 7 {
                        ','
                    } else {
                        '.'
                    },
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
        self.to_play = self.to_play.opposite();
        self.move_count += 1;
    }

    pub fn unmake_move(self: &mut Self, mv: Move) {
        debug_assert!(mv < SIZE);

        self.board[mv] = Piece::Empty;
        self.to_play = self.to_play.opposite();
        self.move_count -= 1;
    }

    pub fn perft(self: &mut Self, depth: i32) -> usize {

        if depth == 0 {
            return 1;
        }

        let mut result = 0;

        if depth > 0 && self.is_finished().is_none() {
            let moves = self.moves();
            for mv in moves {
                self.make_move(mv);
                result += self.perft(depth-1);
                self.unmake_move(mv);
            }
        }
        result
    }

    pub fn is_finished(self: &Self) -> Option<GameResult> {
        if self.move_count == SIZE {
            Some(GameResult::Draw)
        } else if is_win(self.board, self.to_play.opposite()) {
            Some(GameResult::Win(self.to_play.opposite()))
        } else {
            None
        }
    }

    pub fn bestmove_random(self: &Self) -> Move {
        let moves = self.moves();
        let mut rng = thread_rng();

        *moves.choose(&mut rng).unwrap()
    }

    pub fn bestmove_mater(self: &Self) -> Move {
        let moves = self.moves();

        let mut pos = self.duplicate();
        for mv in moves {
            pos.make_move(mv);
            if let Some(GameResult::Win(player)) = pos.is_finished() {
                if player == self.to_play {
                    return mv;
                }
            }
            pos.unmake_move(mv);
        }

        self.bestmove_random()
    }

    pub fn bestmove_evaluator(self: &Self, weights: &Vec<f64>) -> Move {
        let moves = self.moves();

        let mut current_eval = vec![];
        let mut pos = self.duplicate();
        let mut best_eval = -100000.0;
        for mv in moves {
            pos.make_move(mv);
            if let Some(GameResult::Win(player)) = pos.is_finished() {
                if player == self.to_play {
                    return mv;
                }
            }
            let evaluation = pos.evaluate(weights);
            if evaluation > best_eval || current_eval.is_empty() {
                best_eval = evaluation;
                current_eval = vec![mv];
            } else if (evaluation - best_eval).abs() < 0.01 {
                current_eval.push(mv);
            } else {
                // println!("{}", evaluation);
            }
            pos.unmake_move(mv);
        }

        // println!("Best eval = {}", best_eval);
        // println!("Candidates# = {}", current_eval.len());

        if best_eval >= 99.0 {
            // println!("{}", pos.ascii());
        }

        let mut rng = thread_rng();
        *current_eval.choose(&mut rng).unwrap()
    }


    pub fn evaluate(self: &Self, weights: &Vec<f64>) -> f64 {
        let board = self.board;
        let to_play = self.to_play;
        let opp = to_play.opposite();

        let metrics = vec![
            has_open_fours(board, opp) - has_open_fours(board, to_play),
            open_threes_count(board, opp) - open_threes_count(board, to_play),
            open_twos_count(board, opp) - open_twos_count(board, to_play),
            // centralization(board, opp)
            0
        ];

        let mut result = 0.0;
        for (i, value) in metrics.iter().enumerate() {
            result += (*value as f64) * weights[i];
        }
        result
    }
}

pub enum AI {
    Random,
    Mater,
    Eval(Vec<f64>),
}

pub fn bestmove(ai: &AI, pos: &Position) -> Move {
    match ai {
        AI::Random => pos.bestmove_random(),
        AI::Mater => pos.bestmove_mater(),
        AI::Eval(weights) => pos.bestmove_evaluator(weights),
    }
}

pub fn play_game(black_ai: &AI, white_ai: &AI) -> GameResult {
    let mut pos = Position::new();
    loop {
        if let Some(result) = pos.is_finished() {
            println!("{}", pos.ascii());
            return result;
        }
        let mv = if pos.to_play == Player::Black {
            bestmove(&black_ai, &pos)
        } else {
            bestmove(&white_ai, &pos)
        };
        pos.make_move(mv);
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