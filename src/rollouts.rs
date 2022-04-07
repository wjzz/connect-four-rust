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

fn simulate_game_with_starting_move(mv: Move, rng: &mut ThreadRng) -> GameResult {
    let mut pos = Position::new();
    pos.make_move(mv);

    loop {
        if let Some(result) = pos.is_finished() {
            return result;
        }
        let moves = pos.moves();

        pos.make_move(*moves.choose(rng).unwrap());
    }
}

fn get_win_percentage(mv: Move, rng: &mut ThreadRng, tries: usize) -> f64 {
    let mut draws = 0;
    let mut black = 0;
    let mut white = 0;

    for _i in 0..tries {
        match simulate_game_with_starting_move(mv, rng) {
            GameResult::Draw => draws += 1,
            GameResult::Win(Player::Black) => black += 1,
            GameResult::Win(Player::White) => white += 1,
        }
    }
    100.0 * (black as f64) / (tries as f64)
}

fn find_best_move() {
    let mut rng = thread_rng();
    let mut win_ratios = vec![];

    for mv in 0..SIZE {
        let percentage = get_win_percentage(mv, &mut rng, 1000);
        win_ratios.push(percentage as usize);
    }

    for row in 0..ROWS {
        for col in 0..COLS {
            print!("{} ", win_ratios[rowcol2index(row, col)]);
        }
        println!();
    }
}
