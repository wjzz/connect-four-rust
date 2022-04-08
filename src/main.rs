#![allow(dead_code)]
#![allow(unused_variables)]

mod bestmove;
mod bitboard;
mod board;
mod perft;
mod play;
mod position;
mod rollouts;
mod tournament;
mod types;
mod util;

use util::*;
// use board::ArrayPosition;
use bitboard::BitPosition;

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

    println!("Initializing bitboards");
    bitboard::initialize_winning_patterns();

    match &args[0][..] {
        "--perft" => {
            let depth = parse_string(args.get(1), 3);
            // perft::perft::<ArrayPosition>(depth);
            perft::perft::<BitPosition>(depth);
        }
        "--rollout" => {
            let retries = parse_string(args.get(1), 1000);
            rollouts::benchmark_rollouts(retries);
        }
        "--play" => {
            play::play();
        }
        "--tournament" => {
            let tries = parse_string(args.get(1), 10);
            tournament::compare_ais(tries);
        }
        _ => {
            rollouts::rollout_central_move::<BitPosition>();
            // show_usage_and_exit();
        }
    }
}
