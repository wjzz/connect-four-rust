use rand::prelude::*;

use crate::types::*;
use crate::board::ArrayPosition;


impl ArrayPosition {
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

    pub fn evaluate_minmax(self: &mut Self, depth: usize) -> f64 {
        if let Some(result) = self.result() {
            match result {
                GameResult::Win(player) => {
                    if player == self.to_play {
                        return -10000.0;
                    } else {
                        return 10000.0;
                    }
                }
                GameResult::Draw => {
                    return 0.0;
                }
            }
        }

        if depth == 0 {
            return 0.0;
            // return self.evaluate();
        }

        let mut best = -100000.0;
        let moves = self.moves();
        for mv in moves {
            self.make_move(mv);
            let evaluation = -1.0 * self.evaluate_minmax(depth - 1);
            if evaluation > best {
                best = evaluation;
            }
            self.unmake_move(mv);
        }
        best
    }
}

pub enum AI {
    MinMax(usize),
}

impl AI {
    pub fn show(self: &Self) -> String {
        match self {
            AI::MinMax(depth) => format!("minmax(depth={})", depth),
        }
    }

    pub fn bestmove(self: &Self, pos: &ArrayPosition) -> Move {
        match self {
            AI::MinMax(depth) => pos.bestmove_minmax(*depth),
        }
    }
}

pub fn play_game(black_ai: &AI, white_ai: &AI) -> GameResult {
    let mut pos = ArrayPosition::new();
    loop {
        if let Some(result) = pos.result() {
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
