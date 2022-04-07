use rand::prelude::*;

use crate::board::*;

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
    let opp = to_play.other();

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
