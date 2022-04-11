use std::collections::HashMap;
use std::io::Write;
use std::time::Instant;
use thousands::Separable;

use crate::types::*;
use crate::board::ArrayPosition;

static mut NODE_COUNT: usize = 0;

pub fn solve_iterative_deepening() {
    unsafe {
        println!("");
        println!("");

        let depth = SIZE + 1;
        let now = Instant::now();
        let result = solve(&mut ArrayPosition::new(), depth);
        let mut elapsed_millisecs = now.elapsed().as_millis() as usize;
        if elapsed_millisecs == 0 {
            elapsed_millisecs = 1;
        }
        let nps = NODE_COUNT / elapsed_millisecs;

        println!(
            "\ndepth = {:2} | result = {:6} | nodes = {:12} | [elapsed: {}] [speed: {}K nps]",
            depth,
            result,
            NODE_COUNT.separate_with_commas(),
            elapsed_millisecs,
            nps.separate_with_commas(),
        );
    }
}

const MIN_DEPTH: usize = 4;
const MAX_DEPTH: usize = 18;

pub fn solve(pos: &mut ArrayPosition, depth: usize) -> i32 {
    unsafe {
        NODE_COUNT = 0;
    }
    let mut hashmap = HashMap::new();
    return solve_iter(pos, &mut hashmap, depth, -MY_WIN, MY_WIN);
}

const UNKNOWN: i32 = 10_000;
const MY_WIN: i32 = 10;

struct Entry {
    flag: i32,
    value: i32,
}

const EXACT: i32 = 0;
const LOWERBOUND: i32 = 1;
const UPPERBOUND: i32 = 2;

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
            if depth >= SIZE - MAX_DEPTH && depth <= SIZE - MIN_DEPTH {
                if let Some(entry) = hashmap.get(&pos.hash()) {
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
            let mut moves = pos.moves();
            order_moves(pos, &mut moves);
            for mv in moves {
                if depth == SIZE+1 {
                    println!("\x1b[2A\r1st = {}\n", mv);
                } else if depth == SIZE {
                    println!("\x1b[1A\r2nd = {}", mv);
                } else if depth == SIZE-1 {
                    print!("\x1bm\r3rd = {}", mv);
                    std::io::stdout().flush().unwrap();
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
        if depth >= SIZE - MAX_DEPTH && depth <= SIZE - MIN_DEPTH {
            let mut flag = EXACT;
            if alpha <= orig_alpha {
                flag = UPPERBOUND;
            } else if alpha >= beta {
                flag = LOWERBOUND;
            }

            let value = alpha;
            let entry = Entry { value, flag };
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
            let mut line_count = 0;
            for i in line {
                if pos.board[*i] == pp {
                    line_count += 1;
                } else if pos.board[*i] != Piece::Empty {
                    line_count -= 1;
                }
            }
            // sure win
            if line_count == 3 {
                count += 10_000;
            } else if line_count == 2 {
                count += 100;
            } else {
                count += line_count.max(0);
            }
        }
    }
    return -count;
}

fn order_moves(pos: &ArrayPosition, moves: &mut Vec<Move>) {
    moves.sort_by_cached_key(|mv| get_lines_count(pos, *mv));
}