#![allow(dead_code)]
#![allow(unused_variables)]

mod bestmove;
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

    crate::table::Table::new();

    match &args[0][..] {
        "--perft" => {
            let depth = parse_string(args.get(1), 3);
            perft::perft::<ArrayPosition>(depth);
            // perft::perft::<BitPosition>(depth);
        }
        "--rollout" => {
            let retries = parse_string(args.get(1), 1000);
            rollouts::benchmark_rollouts(retries);
        }
        "--play" => {
            play::play();
        }
        "--solve" => {
            solve::solve_iterative_deepening();
        }
        _ => {
            show_usage_and_exit();
        }
    }
}
