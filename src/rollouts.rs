use rand::prelude::*;
use std::time::Instant;
use thousands::Separable;

use crate::types::*;
use crate::board::ArrayPosition;

fn rollout_game(rng: &mut ThreadRng, v: &mut Vec<Move>) -> GameResult {
    let mut pos = ArrayPosition::new();

    loop {
        if let Some(result) = pos.result() {
            return result;
        }
        let moves = pos.moves();
        pos.make_move(*moves.choose(rng).unwrap());
    }
}

pub fn rollout_games(retries: usize) {
    let mut rng = thread_rng();
    let mut v = vec![0; SIZE];

    for _ in 0..retries {
        rollout_game(&mut rng, &mut v);
    }
}

fn rollout_game_shuffle(moves: &Vec<Move>) {
    let mut pos = ArrayPosition::new();

    for mv in moves {
        pos.make_move(*mv);
        if pos.is_finished() {
            return;
        }
    }
}

pub fn rollout_games_shuffle(retries: usize) {
    let mut rng = thread_rng();

    let mut moves = vec![];
    for _ in 0 .. ROWS {
        for i in 0 .. COLS {
            moves.push(i);
        }
    }

    for _ in 0..retries {
        moves.shuffle(&mut rng);
        rollout_game_shuffle(&moves);
    }
}

pub fn benchmark_rollouts(retries: usize) {
    let now = Instant::now();

    rollout_games_shuffle(retries);

    let mut elapsed_millisecs = now.elapsed().as_millis() as usize;
    if elapsed_millisecs == 0 {
        elapsed_millisecs = 1;
    }
    let rollouts_per_sec = retries / elapsed_millisecs;

    println!(
        "performed {} rollouts [elapsed: {}] [speed: {}K rps]",
        retries,
        elapsed_millisecs,
        rollouts_per_sec.separate_with_commas()
    );
}

fn has_a_winning_move<P: Position>(pos: &mut P, moves: &Vec<Move>) -> Option<GameResult> {
    let opp = pos.current_player();
    for &mv in moves {
        pos.make_move(mv);
        let result = pos.result();
        if let Some(GameResult::Win(player)) = result {
            if player == opp {
                return result;
            }
        }
        pos.unmake_move(mv);
    }
    None
}

fn simulate_game_with_move<P: Position>(pos: &P, mv: Move, rng: &mut ThreadRng) -> GameResult {
    let mut pos = pos.duplicate();
    pos.make_move(mv);

    loop {
        if let Some(result) = pos.result() {
            return result;
        }
        let moves = pos.moves();

        // Make a heavy rollout, ensuring we always secure a possible win
        if let Some(result) = has_a_winning_move(&mut pos, &moves) {
            return result;
        }

        pos.make_move(*moves.choose(rng).unwrap());
    }
}

pub fn get_black_win_count<P: Position>(pos: &P, mv: Move, rng: &mut ThreadRng, tries: usize) -> i32 {
    let mut black = 0;

    for _i in 0..tries {
        match simulate_game_with_move(pos, mv, rng) {
            GameResult::Win(Player::Black) => black += 1,
            _ => {}
        }
    }
    black
}
