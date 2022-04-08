use crate::types::*;

pub trait Position {
    fn new() -> Self;
    fn duplicate(self: &Self) -> Self;
    fn current_player(self: &Self) -> Player;
    fn ascii(self: &Self) -> String;
    fn moves(self: &Self) -> Vec<Move>;
    fn make_move(self: &mut Self, mv: Move);
    fn unmake_move(self: &mut Self, mv: Move);
    fn is_finished(self: &Self) -> Option<GameResult>;
    fn move_count(self: &Self) -> usize;

    fn perft(self: &mut Self, depth: usize) -> usize {
        if self.is_finished().is_none() {
            if depth == 1 {
                return self.move_count();
            } else {
                let moves = self.moves();
                let mut result = 0;
                for mv in moves {
                    self.make_move(mv);
                    result += self.perft(depth - 1);
                    self.unmake_move(mv);
                }
                return result;
            }
        } else {
            return 0;
        }
    }
}
