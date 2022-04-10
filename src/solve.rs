use std::collections::HashMap;
use std::time::Instant;
use thousands::Separable;

use crate::types::*;
use crate::board::ArrayPosition;

static mut NODE_COUNT: usize = 0;

pub fn solve_iterative_deepening() {
    unsafe {
        let depth = SIZE + 1;
        let now = Instant::now();
        let result = solve(&mut ArrayPosition::new(), depth);
        let mut elapsed_millisecs = now.elapsed().as_millis() as usize;
        if elapsed_millisecs == 0 {
            elapsed_millisecs = 1;
        }
        let nps = NODE_COUNT / elapsed_millisecs;

        println!(
            "depth = {:2} | result = {:6} | nodes = {:12} | [elapsed: {}] [speed: {}K nps]",
            depth,
            result,
            NODE_COUNT.separate_with_commas(),
            elapsed_millisecs,
            nps.separate_with_commas(),
        );
    }
}

const MIN_DEPTH: usize = 3;
const MAX_DEPTH: usize = 12;

pub fn solve(pos: &mut ArrayPosition, depth: usize) -> i32 {
    unsafe {
        NODE_COUNT = 0;
    }
    let mut hashmap = HashMap::new();
    return solve_iter(pos, &mut hashmap, depth);
}

const UNKNOWN: i32 = 10_000;
const MY_WIN: i32 = 10;

fn solve_iter(pos: &mut ArrayPosition, hashmap: &mut HashMap<usize, i32>, depth: usize) -> i32 {
    unsafe {
        NODE_COUNT += 1;
    }
    if let Some(result) = pos.result() {
        match result {
            GameResult::Draw => return 0,
            GameResult::Win(player) =>
                if player == pos.to_play {
                    return MY_WIN;
                } else {
                    return -MY_WIN;
                }
        }
    } else {
        let mut best_eval = -UNKNOWN;
        if depth > 0 {
            if depth >= MIN_DEPTH && depth <= MAX_DEPTH {
                if let Some(result) = hashmap.get(&pos.hash()) {
                    return *result;
                }
            }
            let moves = pos.moves();
            for mv in moves {
                if depth == SIZE+1 {
                    println!("mv = {}", mv);
                } else if depth == SIZE {
                    println!("  mv = {}", mv);
                }
                pos.make_move(mv);
                let eval = -solve_iter(pos, hashmap, depth-1);
                if eval > best_eval {
                    best_eval = eval;
                }
                pos.unmake_move(mv);

                if eval == MY_WIN {
                    break;
                }
            }
        }
        if depth >= MIN_DEPTH && depth <= MAX_DEPTH {
            hashmap.insert(pos.hash(), best_eval);
        }
        return best_eval;
    }
}