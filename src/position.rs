use crate::types::*;

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

    fn is_finished(self: &Self) -> bool {
        self.result().is_some()
    }

    fn perft(self: &mut Self, depth: usize) -> usize {
        self.perft_iter(depth, 0)
    }

    fn perft_iter(self: &mut Self, depth: usize, level: usize) -> usize {
        if self.result().is_none() {
            if depth == 1 {
                return self.move_count();
            } else {
                let moves = self.moves();
                let mut result = 0;
                for mv in moves {
                    self.make_move(mv);
                    result += self.perft_iter(depth - 1, level + 1);
                    self.unmake_move(mv);
                }
                return result;
            }
        } else {
            return 0;
        }
    }
}
