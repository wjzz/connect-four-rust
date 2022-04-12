use std::collections::HashMap;

use crate::types::*;

const MIN_DEPTH: usize = 3;
const MAX_DEPTH: usize = 12;

pub trait Position {
    fn new() -> Self;
    fn duplicate(self: &Self) -> Self;
    fn current_player(self: &Self) -> Player;
    fn ascii(self: &Self) -> String;
    fn moves(self: &Self) -> Vec<Move>;
    fn make_move(self: &mut Self, mv: Move);
    fn unmake_move(self: &mut Self, mv: Move);
    fn result(self: &Self) -> Option<GameResult>;
    fn fast_result(self: &Self, mv: Move) -> Option<GameResult>;
    fn move_count(self: &Self) -> usize;
    fn hash(self: &Self) -> usize;
    fn symm_hash(self: &Self) -> usize; /* hash of the symmetric image (y axis) */

    fn is_finished(self: &Self) -> bool {
        self.result().is_some()
    }

    fn perft(self: &mut Self, depth: usize) -> usize {
        let mut hashmap = HashMap::new();

        self.perft_iter(depth, &mut hashmap)
    }

    fn perft_iter(self: &mut Self, depth: usize, hashmap: &mut HashMap<usize, usize>) -> usize {
        if depth == 1 {
            if self.result().is_none() {
                return self.move_count();
            } else {
                return 0;
            }
        } else {
            if depth >= MIN_DEPTH && depth <= MAX_DEPTH {
                if let Some(result) = hashmap.get(&self.hash()) {
                    return *result;
                }
            }
            if self.result().is_some() {
                return 0;
            } else {
                let moves = self.moves();
                let mut result = 0;
                for mv in moves {
                    self.make_move(mv);
                    result += self.perft_iter(depth - 1, hashmap);
                    self.unmake_move(mv);
                }
                if depth >= MIN_DEPTH && depth <= MAX_DEPTH {
                    hashmap.insert(self.hash(), result);
                }
                return result;
            }
        }

        // if self.result().is_none() {
        //     if depth == 1 {
        //         return self.move_count();
        //     } else {
        //         if depth >= MIN_DEPTH && depth <= MAX_DEPTH {
        //             if let Some(result) = hashmap.get(&self.hash()) {
        //                 return *result;
        //             }
        //         }
        //         let moves = self.moves();
        //         let mut result = 0;
        //         for mv in moves {
        //             self.make_move(mv);
        //             result += self.perft_iter(depth - 1, hashmap);
        //             self.unmake_move(mv);
        //         }
        //         if depth >= MIN_DEPTH && depth <= MAX_DEPTH {
        //             hashmap.insert(self.hash(), result);
        //         }
        //         return result;
        //     }
        // } else {
        //     return 0;
        // }
    }
}
