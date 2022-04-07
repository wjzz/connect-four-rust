use rand::prelude::*;

use crate::board::*;
use crate::rollouts;

fn has_open_fours(board: Board, player: Player) -> usize {
    let pp = Piece::from_player(player);

    for row in 0..ROWS {
        for col in 0..COLS {
            let base = rowcol2index(row, col);
            if board[base] == Piece::Empty {
                // row
                if col + 5 < COLS {
                    let d = 1;
                    if pp == board[base + d]
                        && pp == board[base + 2 * d]
                        && pp == board[base + 3 * d]
                        && pp == board[base + 4 * d]
                        && Piece::Empty == board[base + 5 * d]
                    {
                        return 1;
                    }
                }
                // column
                if row + 5 < ROWS {
                    let d = 15;
                    if pp == board[base + d]
                        && pp == board[base + 2 * d]
                        && pp == board[base + 3 * d]
                        && pp == board[base + 4 * d]
                        && Piece::Empty == board[base + 5 * d]
                    {
                        return 1;
                    }
                }
                // rising diagonal
                if row + 5 < ROWS && col > 5 {
                    let d = 14;
                    if pp == board[base + d]
                        && pp == board[base + 2 * d]
                        && pp == board[base + 3 * d]
                        && pp == board[base + 4 * d]
                        && Piece::Empty == board[base + 5 * d]
                    {
                        return 1;
                    }
                }
                // decreasing diagonal
                if row + 5 < ROWS && col + 5 < COLS {
                    let d = 16;
                    if pp == board[base + d]
                        && pp == board[base + 2 * d]
                        && pp == board[base + 3 * d]
                        && pp == board[base + 4 * d]
                        && Piece::Empty == board[base + 5 * d]
                    {
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
                    if pp == board[base + d]
                        && pp == board[base + 2 * d]
                        && pp == board[base + 3 * d]
                        && Piece::Empty == board[base + 4 * d]
                    {
                        result += 1;
                    }
                }
                // column
                if row + 4 < ROWS {
                    let d = 15;
                    if pp == board[base + d]
                        && pp == board[base + 2 * d]
                        && pp == board[base + 3 * d]
                        && Piece::Empty == board[base + 4 * d]
                    {
                        result += 1;
                    }
                }
                // rising diagonal
                if row + 4 < ROWS && col > 4 {
                    let d = 14;
                    if pp == board[base + d]
                        && pp == board[base + 2 * d]
                        && pp == board[base + 3 * d]
                        && Piece::Empty == board[base + 4 * d]
                    {
                        result += 1;
                    }
                }
                // decreasing diagonal
                if row + 4 < ROWS && col + 4 < COLS {
                    let d = 16;
                    if pp == board[base + d]
                        && pp == board[base + 2 * d]
                        && pp == board[base + 3 * d]
                        && Piece::Empty == board[base + 4 * d]
                    {
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
                    if pp == board[base + d]
                        && pp == board[base + 2 * d]
                        && Piece::Empty == board[base + 3 * d]
                    {
                        result += 1;
                    }
                }
                // column
                if row + 3 < ROWS {
                    let d = 15;
                    if pp == board[base + d]
                        && pp == board[base + 2 * d]
                        && Piece::Empty == board[base + 3 * d]
                    {
                        result += 1;
                    }
                }
                // rising diagonal
                if row + 3 < ROWS && col > 3 {
                    let d = 14;
                    if pp == board[base + d]
                        && pp == board[base + 2 * d]
                        && Piece::Empty == board[base + 3 * d]
                    {
                        result += 1;
                    }
                }
                // decreasing diagonal
                if row + 3 < ROWS && col + 3 < COLS {
                    let d = 16;
                    if pp == board[base + d]
                        && pp == board[base + 2 * d]
                        && Piece::Empty == board[base + 3 * d]
                    {
                        result += 1;
                    }
                }
            }
        }
    }

    result
}

fn average_centralization(board: Board, player: Player, move_count: usize) -> usize {
    // TODO: use Euclidean distance not manhattan distance

    // single stone can get from 14 (7+7) down to 0
    // we calculate the average, so the value is also in [0,14]
    let mut result = 0;
    let pp = Piece::from_player(player);

    for row in 0..ROWS {
        for col in 0..COLS {
            let base = rowcol2index(row, col);
            if board[base] == pp {
                result += 14 - (7 - (row as i32)).abs() - (7 - (col as i32)).abs();
            }
        }
    }
    // calculate the average
    (2 * result / (move_count as i32)) as usize
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

    pub fn bestmove_minmax(self: &Self, depth: usize) -> Move {
        let moves = self.moves();

        let mut current_eval = vec![];
        let mut pos = self.duplicate();
        let mut best_eval = -100000.0;
        for mv in moves {
            pos.make_move(mv);
            let evaluation = pos.evaluate_minmax(depth);
            if evaluation > best_eval {
                best_eval = evaluation;
                current_eval = vec![mv];
            } else if (evaluation - best_eval).abs() < 0.01 {
                current_eval.push(mv);
            }
            pos.unmake_move(mv);
        }

        let mut rng = thread_rng();
        *current_eval.choose(&mut rng).unwrap()
    }

    pub fn bestmove_rollout(self: &Self, tries: usize) -> Move {
        let mut rng = thread_rng();
        let moves = self.moves();

        let mut current_eval = vec![];
        let mut best_eval = -10.0;
        for mv in moves {
            let mut evaluation = rollouts::get_black_win_ratio(&self, mv, &mut rng, tries);
            if self.to_play == Player::White {
                evaluation = 1.0 - evaluation;
            }

            // println!("eval = {}", evaluation);
            if evaluation > best_eval {
                best_eval = evaluation;
                current_eval = vec![mv];
            } else if (evaluation - best_eval).abs() < 0.03 {
                current_eval.push(mv);
            }
        }

        let mut rng = thread_rng();
        *current_eval.choose(&mut rng).unwrap()
    }

    pub fn evaluate_minmax(self: &mut Self, depth: usize) -> f64 {
        if let Some(result) = self.is_finished() {
            match result {
                GameResult::Win(player) => {
                    if player == self.to_play {
                        return -10000.0;
                    } else {
                        return 10000.0;
                    }
                },
                GameResult::Draw => {
                    return 0.0;
                }
            }
        }

        if depth == 0 {
            return self.evaluate();
        }

        let mut best = -100000.0;
        let moves = self.moves();
        for mv in moves {
            self.make_move(mv);
            let evaluation = -1.0 * self.evaluate_minmax(depth-1);
            if evaluation > best {
                best = evaluation;
            }
            self.unmake_move(mv);
        }
        best
    }

    pub fn evaluate(self: &Self) -> f64 {
        let board = self.board;
        let opponent = self.to_play;
        let current_player = opponent.other();

        // let open_fours = has_open_fours(board, current_player) - has_open_fours(board, opponent);
        // let open_threes =
            // open_threes_count(board, current_player) - open_threes_count(board, opponent);
        let open_twos = open_twos_count(board, current_player) - open_twos_count(board, opponent);
        let centralization_score = average_centralization(board, current_player, self.move_count);

        let mut result = 0.0;
        result += 20.0 * open_twos as f64;
        result += centralization_score as f64;
        result
    }
}

pub enum AI {
    Random,
    Mater,
    MinMax(usize),
    Rollout(usize)
}

impl AI {
    pub fn show(self: &Self) -> String {
        match self {
            AI::Random => "random".to_string(),
            AI::Mater => "checkmater".to_string(),
            AI::MinMax(depth) => format!("minmax(depth={})", depth),
            AI::Rollout(tries) => format!("rollout(tries={})", tries),
        }
    }

    pub fn bestmove(self: &Self, pos: &Position) -> Move {
        match self {
            AI::Random => pos.bestmove_random(),
            AI::Mater => pos.bestmove_mater(),
            AI::MinMax(depth) => pos.bestmove_minmax(*depth),
            AI::Rollout(tries) => pos.bestmove_rollout(*tries),
        }
    }
}

pub fn play_game(black_ai: &AI, white_ai: &AI) -> GameResult {
    let mut pos = Position::new();
    loop {
        if let Some(result) = pos.is_finished() {
            // println!("{}", pos.ascii());
            return result;
        }
        let mv = if pos.to_play == Player::Black {
            black_ai.bestmove(&pos)
        } else {
            white_ai.bestmove(&pos)
        };
        pos.make_move(mv);
    }
}
