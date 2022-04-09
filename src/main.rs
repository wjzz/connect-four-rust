#![allow(dead_code)]
#![allow(unused_variables)]

mod board;
mod perft;
mod play;
mod solve;
mod position;
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

    match &args[0][..] {
        "--perft" => {
            let depth = parse_string(args.get(1), 3);
            perft::perft::<ArrayPosition>(depth);
            // perft::perft::<BitPosition>(depth);
        }
        "--play" => {
            play::play();
        }
        _ => {
            show_usage_and_exit();
        }
    }
}
