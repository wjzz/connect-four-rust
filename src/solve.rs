use std::io::Write;
use std::time::Instant;
use thousands::Separable;

use crate::table::Table;
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

pub fn solve(pos: &mut ArrayPosition, depth: usize) -> i32 {
    unsafe {
        NODE_COUNT = 0;
    }
    let mut hashmap = Table::new();
    let result = solve_iter(pos, &mut hashmap, depth, -WIN, WIN);
    println!("\ncollissions = {}", hashmap.collissions);
    return result;
}

const UNKNOWN: i32 = 10_000;
const DRAW: i32 = 0;
const WIN: i32 = 10;
const LOSS: i32 = -WIN;
const DRAW_OR_WIN: i32 = 5;
const DRAW_OR_LOSE: i32 = -DRAW_OR_WIN;

type Entry = i32;

fn progress_bar(depth: usize, mv: Move) {
    if depth == SIZE+1 {
        println!("\x1b[2A\r1st = {}\n", mv);
    } else if depth == SIZE {
        println!("\x1b[1A\r2nd = {}", mv);
    } else if depth == SIZE-1 {
        print!("\x1bm\r3rd = {}", mv);
        std::io::stdout().flush().unwrap();
    }
}

fn solve_iter(pos: &mut ArrayPosition, hashmap: &mut Table, depth: usize, mut alpha: i32, mut beta: i32) -> i32 {
    unsafe {
        NODE_COUNT += 1;
    }
    if let Some(result) = pos.result() {
        match result {
            GameResult::Draw => return DRAW,
            GameResult::Win(player) =>
                if player == pos.to_play {
                    return WIN;
                } else {
                    return -WIN;
                }
        }
    } else {
        assert!(alpha < beta);
        let orig_alpha = alpha;
        if depth > 0 {
            if depth >= 1 {
                if let Some(entry) = hashmap.get(pos.hash()) {
                    if entry == DRAW_OR_LOSE {
                        // oa = DRAW, beta = WIN
                        beta = beta.min(DRAW);
                    } else if entry == DRAW_OR_WIN {
                        // oa = LOSS, beta = DRAW
                        alpha = alpha.max(DRAW);
                    } else {
                        return entry;
                    }
                    if alpha >= beta {
                        return DRAW;
                    }
                }
                }
            let mut moves = pos.moves();
            order_moves(pos, &mut moves);
            for mv in moves {
                progress_bar(depth, mv);

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
        if depth >= 1 {
            let mut entry = alpha;
            if alpha == DRAW {
                if orig_alpha == DRAW {
                    entry = DRAW_OR_LOSE;
                } else if beta == DRAW {
                    alpha = DRAW_OR_WIN;
                }
            }
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