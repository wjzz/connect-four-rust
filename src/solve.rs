use std::io::Write;
use std::time::Instant;
use thousands::Separable;

use crate::table::Table;
use crate::types::*;
use crate::board::ArrayPosition;

const VERBOSE_OUTPUT_SETTING: Option<&'static str> = option_env!("VERBOSE_OUTPUT");

lazy_static! {
    static ref VERBOSE_OUTPUT: bool = VERBOSE_OUTPUT_SETTING.map(|s| s.parse().unwrap()).unwrap_or(true);
}

static mut NODE_COUNT: usize = 0;

pub fn solve_iterative_deepening() {

    unsafe {
        if *VERBOSE_OUTPUT {
            println!("");
            println!("");
        }

        let depth = SIZE + 1;
        let now = Instant::now();
        let result = solve(&mut ArrayPosition::new(), depth);
        let mut elapsed_millisecs = now.elapsed().as_millis() as usize;
        if elapsed_millisecs == 0 {
            elapsed_millisecs = 1;
        }
        let nps = NODE_COUNT / elapsed_millisecs;

        let result = match result {
            0 => -10, // white win
            1 => 0,   // draw
            2 => 10,  // black win
            _ => { panic!("wrong result"); }
        };

        if *VERBOSE_OUTPUT {
            println!(
                "\n{} x {} | result = {:6} | nodes = {:12} | [elapsed: {}] [speed: {}K nps]",
                ROWS,
                COLS,
                result,
                NODE_COUNT.separate_with_commas(),
                elapsed_millisecs,
                nps.separate_with_commas(),
            );
        } else {
            println!("result,{},{},{},{},{}", ROWS, COLS, result, NODE_COUNT, elapsed_millisecs);
        }
    }
}

pub fn solve(pos: &mut ArrayPosition, depth: usize) -> usize {
    unsafe {
        NODE_COUNT = 0;
    }
    let mut hashmap = Table::new();
    let result = solve_iter(pos, &mut hashmap, depth, LOSS, WIN);
    println!("\ncollissions = {}", hashmap.collissions.separate_with_commas());
    println!("inserts = {}", hashmap.inserts.separate_with_commas());
    println!("uppers = {}", hashmap.uppers.separate_with_commas());
    println!("lowers = {}", hashmap.lowers.separate_with_commas());
    println!("gets = {}", hashmap.gets.separate_with_commas());
    println!("get_misses = {}", hashmap.get_misses.separate_with_commas());
    return result;
}

const LOSS: usize = 0;
const DRAW: usize = 1;
const WIN: usize = 2;
const DRAW_LOWERBOUND: usize = 3;
const DRAW_UPPERBOUND: usize = 4;

type Entry = usize;

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

impl GameResult {
    pub fn to_eval(self, to_play: Player) -> usize {
        match self {
            GameResult::Draw => return DRAW,
            GameResult::Win(player) =>
                if player == to_play {
                    return WIN;
                } else {
                    return LOSS;
                }
        }
    }
}

const MIN_DEPTH: usize = 1;

fn solve_iter(pos: &mut ArrayPosition, hashmap: &mut Table, depth: usize, mut alpha: usize, mut beta: usize) -> usize {
    unsafe {
        NODE_COUNT += 1;

        if let Some(result) = pos.result() {
            return result.to_eval(pos.to_play);
        } else {
            let before = NODE_COUNT;

            let orig_alpha = alpha;
            if depth >= MIN_DEPTH {
                if let Some(entry) = hashmap.get(pos.hash()) {
                    if entry == DRAW_UPPERBOUND {
                        beta = beta.min(DRAW);
                    } else if entry == DRAW_LOWERBOUND {
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
                if *VERBOSE_OUTPUT {
                    progress_bar(depth, mv);
                }
                pos.make_move(mv);
                let eval = 2 - solve_iter(pos, hashmap, depth-1, 2-beta, 2-alpha);
                pos.unmake_move(mv);

                if eval > alpha {
                    alpha = eval;
                    if beta <= alpha {
                        break;
                    }
                }
            }

            if depth >= MIN_DEPTH {
                let nodes = NODE_COUNT - before;
                let work: usize = 64 - (nodes.leading_zeros() as usize);

                let mut value = alpha;
                if alpha == DRAW {
                    if alpha <= orig_alpha {
                        value = DRAW_UPPERBOUND;
                    } else if alpha >= beta {
                        value = DRAW_LOWERBOUND;
                    }
                }

                hashmap.insert(pos.hash(), value, work);
            }
            return alpha;
        }
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