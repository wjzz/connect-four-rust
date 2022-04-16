#[macro_use]
extern crate lazy_static;

mod bitboard;
mod board;
mod perft;
mod position;
mod solve;
mod table;
mod types;
mod util;

use bitboard::BitPosition;
use board::ArrayPosition;
use util::*;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        std::process::exit(1);
    }

    board::initialize();
    bitboard::initialize_lines();

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
        "--solve" => {
            println!("ArrayPosition");
            solve::solve_game::<ArrayPosition>();

            println!("BitPosition");
            solve::solve_game::<BitPosition>();
        }
        _ => {
            std::process::exit(1);
        }
    }
}
