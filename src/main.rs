#![allow(dead_code)]
#![allow(unused_variables)]

mod bestmove;
mod board;
mod perft;
mod play;
mod rollouts;
mod util;

use bestmove::*;
use board::*;
use util::*;

fn generate_weights_population(size: usize) -> Vec<AI> {
    let mut results = vec![];

    for i in 0 .. size {
        let mut v: Vec<f64> = vec![ rand(), rand(), rand(), rand()];
        let total: f64 = v.iter().sum();
        for a in v.iter_mut() {
            *a /= total;
        }
        println!("{} = [{:+.2}, {:+.2}, {:+.2}, {:+.2}]", i, v[0], v[1], v[2], v[3]);
        results.push(AI::Eval(v));
    }
    results
}

fn result2score(result: GameResult) -> (f64, f64) {
    match result {
        GameResult::Draw => (0.5, 0.5),
        GameResult::Win(player) =>
            match player {
                Player::Black => (1.0, 0.0),
                Player::White => (0.0, 1.0),
            }
    }
}

fn tournament(ais: Vec<AI>) {
    let mut scores = vec![0.0; ais.len()];

    for (i, first_ai) in ais.iter().enumerate() {
        for (j, second_ai) in ais.iter().enumerate() {
            if i < j {
                let first_before = scores[i];
                let second_before = scores[j];
                let tries = 2;
                for k in 0 .. tries {
                    if k % 2 == 0 {
                        let result = play_game(first_ai, second_ai);
                        let (score1, score2) = result2score(result);
                        scores[i] += score1;
                        scores[j] += score2;
                    } else {
                        let result = play_game(second_ai, first_ai);
                        let (score2, score1) = result2score(result);
                        scores[i] += score1;
                        scores[j] += score2;
                    }
                }
                println!("{:2} vs {:2}: {:2} - {:2}", i, j, scores[i] - first_before, scores[j] - second_before);
            }
        }
    }
    let mut v: Vec<(usize, usize)> = scores.iter().enumerate().map(|p| (*p.1 as usize, p.0)).collect();
    v.sort();
    for (score, i) in v {
        println!("{} -> {}", i, score);
    }
}

const USAGE: &'static str = "\n
--perft N   <N = max depth>
";

fn show_usage_and_exit() {
    println!("Usage: {}", USAGE);
    std::process::exit(1);
}

fn main() {
    // find_best_move();
    // let result = play_game(AI::Eval, AI::Eval);
    // println!("Result = {:?}", result);

    // let ais = generate_weights_population(10);
    // tournament(ais);

    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        show_usage_and_exit();
    }

    match &args[0][..] {
        "--perft" => {
            let depth = parse_string(args.get(1), 3);
            perft::perft(depth);
        },
        "--rollout" => {
            let retries = parse_string(args.get(1), 1000);
            rollouts::benchmark_rollouts(retries);
        },
        _ => {
            show_usage_and_exit();
        }
    }

    // play::play();
}
