#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate lazy_static;

mod bestmove;
mod bitboard;
mod board;
mod perft;
mod play;
mod rollouts;
mod solve;
mod position;
mod table;
mod types;
mod util;

use util::*;
use bitboard::BitPosition;
use board::ArrayPosition;

const USAGE: &'static str = "\n
--perft N   <N = max depth>
";

fn show_usage_and_exit() {
    println!("Usage: {}", USAGE);
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        show_usage_and_exit();
    }

    board::initialize();

    match &args[0][..] {
        "--perft" => {
            let depth = parse_string(args.get(1), 3);

            println!("Fast array");
            perft::perft::<ArrayPosition>(depth);
            println!("Fast bitboard");
            perft::perft::<BitPosition>(depth);

            println!("Slow array");
            perft::perft_naive::<ArrayPosition>(depth);
            println!("Slow bitboard");
            perft::perft_naive::<BitPosition>(depth);
        }
        "--rollout" => {
            let retries = parse_string(args.get(1), 1000);
            rollouts::benchmark_rollouts(retries);
        }
        "--play" => {
            play::play();
        }
        "--solve" => {
            println!("ArrayPosition");
            solve::solve_game::<ArrayPosition>();

            println!("BitPosition");
            solve::solve_game::<BitPosition>();
        }
        _ => {
            show_usage_and_exit();
        }
    }
}
