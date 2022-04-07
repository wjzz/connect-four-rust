use rand::prelude::*;
use std::time::Instant;
use thousands::Separable;

use crate::board::*;

fn rollout_game(rng: &mut ThreadRng) -> GameResult {
    let mut pos = Position::new();

    loop {
        if let Some(result) = pos.is_finished() {
            return result;
        }
        let moves = pos.moves();
        pos.make_move(*moves.choose(rng).unwrap());
    }
}

pub fn benchmark_rollouts(retries: usize) {
    let mut rng = thread_rng();
    let now = Instant::now();

    for _ in 0..retries {
        let _ = rollout_game(&mut rng);
    }

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

fn simulate_game_with_move(pos: &Position, mv: Move, rng: &mut ThreadRng) -> GameResult {
    let mut pos = pos.duplicate();
    pos.make_move(mv);

    loop {
        if let Some(result) = pos.is_finished() {
            return result;
        }
        let moves = pos.moves();

        pos.make_move(*moves.choose(rng).unwrap());
    }
}

pub fn get_black_win_ratio(pos: &Position, mv: Move, rng: &mut ThreadRng, tries: usize) -> f64 {
    let mut black = 0;

    for _i in 0..tries {
        match simulate_game_with_move(pos, mv, rng) {
            GameResult::Win(Player::Black) => black += 1,
            _ => {},
        }
    }
    (black as f64) / (tries as f64)
}
