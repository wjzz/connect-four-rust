use std::io::Write;
use std::time::Instant;
use thousands::Separable;

use crate::table::Table;
use crate::types::*;
use crate::position::Position;

const VERBOSE_OUTPUT_SETTING: Option<&'static str> = option_env!("VERBOSE_OUTPUT");

lazy_static! {
    static ref VERBOSE_OUTPUT: bool = VERBOSE_OUTPUT_SETTING.map(|s| s.parse().unwrap()).unwrap_or(true);
}

static mut NODE_COUNT: usize = 0;

const MIN_DEPTH: usize = 1;
const SYMMETRY_CUTOFF: usize = 10;

const LOSS: usize = 0;
const DRAW: usize = 1;
const WIN: usize = 2;
const DRAW_LOWERBOUND: usize = 3;
const DRAW_UPPERBOUND: usize = 4;

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

pub fn solve_game<P:Position>() {

    unsafe {
        if *VERBOSE_OUTPUT {
            println!("");
            println!("");
        }

        let depth = SIZE + 1;

        let now = Instant::now();
        let result = solve_top(&mut P::new(), depth);

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

pub fn solve_top<P: Position>(pos: &mut P, depth: usize) -> usize {
    unsafe {
        NODE_COUNT = 0;
    }
    let mut hashmap = Table::new();
    let result = solve_iter(pos, &mut hashmap, depth, LOSS, WIN);
    hashmap.print_stats();

    return result;
}

fn lookup_table<P:Position>(pos: &mut P, hashmap: &mut Table, depth: usize) -> Option<usize> {
    match hashmap.get(pos.hash()) {
        Some(result) => Some(result),
        None => {
            if SIZE - depth <= SYMMETRY_CUTOFF {
                hashmap.get(pos.symm_hash())
            } else {
                None
            }
        }
    }
}

fn solve_iter<P: Position>(pos: &mut P, hashmap: &mut Table, depth: usize, mut alpha: usize, mut beta: usize) -> usize {
    unsafe {
        NODE_COUNT += 1;

        if let Some(result) = pos.result() {
            return result.to_eval(pos.current_player());
        } else {
            let before = NODE_COUNT;

            let orig_alpha = alpha;
            if depth >= MIN_DEPTH {
                if let Some(entry) = lookup_table(pos, hashmap, depth) {
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

fn order_moves<P: Position>(pos: &P, moves: &mut Vec<Move>) {
    moves.sort_by_cached_key(|mv| pos.get_lines_count(*mv));
}

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
