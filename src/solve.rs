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
const MAX_DEPTH: usize = 10;

pub fn solve(pos: &mut ArrayPosition, depth: usize) -> i32 {
    unsafe {
        NODE_COUNT = 0;
    }
    let mut hashmap = HashMap::new();
    return solve_iter(pos, &mut hashmap, depth, -UNKNOWN, UNKNOWN);
}

const UNKNOWN: i32 = 10_000;
const MY_WIN: i32 = 10;

struct Entry {
    depth: usize,
    flag: usize,
    value: i32,
}

const EXACT: usize = 0;
const LOWERBOUND: usize = 1;
const UPPERBOUND: usize = 2;

fn solve_iter(pos: &mut ArrayPosition, hashmap: &mut HashMap<usize, Entry>, depth: usize, mut alpha: i32, mut beta: i32) -> i32 {
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
        let orig_alpha = alpha;
        if depth > 0 {
            if depth >= MIN_DEPTH && depth <= MAX_DEPTH {
                if let Some(entry) = hashmap.get(&pos.hash()) {
                    if entry.depth >= depth {
                        if entry.flag == EXACT {
                            return entry.value;
                        } else if entry.flag == LOWERBOUND {
                            alpha = alpha.max(entry.value);
                        } else {
                            // upperbound
                            beta = beta.min(entry.value);
                        }
                        if alpha >= beta {
                            return entry.value;
                        }
                    }
                }
            }
            let mut moves = pos.moves();
            order_moves(pos, &mut moves);
            for mv in moves {
                if depth == SIZE+1 {
                    println!("mv = {}", mv);
                } else if depth == SIZE {
                    println!("  mv = {}", mv);
                }
                pos.make_move(mv);
                let eval = -solve_iter(pos, hashmap, depth-1, -beta, -alpha);
                pos.unmake_move(mv);

                if eval > alpha {
                    alpha = eval;
                    if beta <= alpha {
                        break;
                    }
                }
            }
        }
        if depth >= MIN_DEPTH && depth <= MAX_DEPTH {
            let mut flag = EXACT;
            if alpha <= orig_alpha {
                flag = UPPERBOUND;
            } else if alpha >= beta {
                flag = LOWERBOUND;
            }

            let value = alpha;
            let entry = Entry { depth, value, flag };
            hashmap.insert(pos.hash(), entry);
        }
        return alpha;
    }
}

fn get_lines_count(pos: &ArrayPosition, mv: Move) -> i32 {
    let index = rowcol2index(pos.counts[mv], mv);
    let mut count = 0;
    let pp = Piece::from_player(pos.to_play);
    unsafe {
        for line in &crate::board::LINES_BY_INDEX[index] {
            for i in line {
                if pos.board[*i] == pp {
                    count += 1;
                }
            }
        }
    }
    return -count;
}

fn order_moves(pos: &ArrayPosition, moves: &mut Vec<Move>) {
    moves.sort_by_cached_key(|mv| get_lines_count(pos, *mv));
}