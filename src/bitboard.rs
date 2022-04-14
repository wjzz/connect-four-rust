use crate::types::*;

type BitBoard = usize;

pub struct BitPosition {
    bbs: [BitBoard; 2],
    counts: [usize; COLS],
    move_count: usize,
    hash: usize,
}

// Example bitboard layout (for ROWS = 6, COLS = 7)
//
//
//         TOP
// .  .  .   .  .  .  .
// 05 12 19 26 32 39 46
// 04 11 18 25 31 38 45
// 03 10 17 24 31 37 44
// 02 09 16 23 30 36 43
// 01 08 15 22 29 35 42
// 00 07 14 21 28 34 41
//          BOTTOM

const COLS1: usize = COLS + 1;
const COLS2: usize = COLS + 2;
const COLSM1: usize = COLS - 1;
const ROWS1: usize = ROWS + 1;
const ROWS2: usize = ROWS + 2;

const COL1: usize = (1 << COLS1) - 1;
const ALL1: usize = !0;
const BOTTOM: usize = ALL1 / COL1;

fn check_win_delta(bb: BitBoard, delta: usize) -> bool {
    let d = bb & (bb >> delta);
    d & (d >> 2 * delta) != 0
}

fn check_win(bb: BitBoard) -> bool {
    check_win_delta(bb, 1) || check_win_delta(bb, ROWS1) || check_win_delta(bb, ROWS2) || check_win_delta(bb, ROWS)
}

// fn count_twos_delta(bb: BitBoard, delta: usize) -> usize {
//     let d = bb & (bb >> delta);
//     d.count_ones() as usize
// }

// fn count_twos(bb: BitBoard) -> usize {
//     count_twos_delta(bb, 1) + count_twos_delta(bb, ROWS1) + count_twos_delta(bb, ROWS2) + count_twos_delta(bb, ROWS)
// }

fn count_threes_delta(bb: BitBoard, delta: usize) -> usize {
    let d = bb & (bb >> delta);
    let d2 = d & (d >> delta);
    d2.count_ones() as usize
}

fn count_threes(bb: BitBoard) -> usize {
    count_threes_delta(bb, 1) + count_threes_delta(bb, ROWS1) + count_threes_delta(bb, ROWS2) + count_threes_delta(bb, ROWS)
}


impl BitPosition {
pub fn _hash(self: &Self) -> usize {

    // trick from phourstones:
    // self.bbs[0] + self.bbs[1] + BOTTOM has 1s at the first empty place in each colum
    // thus hash() is a perfect encoding of the position
    self.bbs[0] + self.bbs[1] + BOTTOM + self.bbs[self.move_count & 1]
}
}

impl Position for BitPosition {
    fn new() -> Self {
        let bbs = [0; 2];
        let mut counts = [0; COLS];
        let move_count = 0;
        for i in 0..COLS {
            counts[i] = i * ROWS1;
        }
        let hash = 0;
        BitPosition { bbs, counts, move_count, hash }
    }

    fn duplicate(self: &Self) -> Self {
        let bbs = self.bbs.clone();
        let counts = self.counts.clone();
        let move_count = self.move_count;
        let hash = self.hash;
        BitPosition { bbs, counts, move_count, hash }
    }

    fn current_player(self: &Self) -> Player {
        if self.move_count % 2 == 0 {
            Player::Black
        } else {
            Player::White
        }
    }

    fn is_move_legal(self: &Self, mv: Move) -> bool {
        // TODO: check if possible to do quicker
        self.counts[mv] < (mv+1)*ROWS1 - 1
    }

    fn moves(self: &Self) -> Vec<Move> {
        let mut legal_moves = vec![];

        for i in 0..COLS {
            if self.counts[i] < (i+1)*ROWS1 - 1 {
                legal_moves.push(i);
            }
        }

        legal_moves
    }

    fn make_move(self: &mut Self, mv: Move) {
        self.bbs[self.move_count & 1] |= 1 << self.counts[mv];
        self.counts[mv] += 1;
        self.move_count += 1;
        self.hash = self._hash();
    }

    fn unmake_move(self: &mut Self, mv: Move) {
        self.counts[mv] -= 1;
        self.move_count -= 1;
        self.bbs[self.move_count & 1] &= !(1 << self.counts[mv]);
        self.hash = self._hash();
    }

    fn legal_move_count(self: &Self) -> usize {
        // TODO: use some bit tricks to make this faster
        self.moves().len()
    }

    fn result(self: &Self) -> Option<GameResult> {
        if self.move_count < 7 {
            None
        } else if check_win(self.bbs[1 - (self.move_count & 1)]) {
            Some(GameResult::Win(Player::from_usize(1- (self.move_count & 1))))
        } else if self.move_count == SIZE {
            Some(GameResult::Draw)
        } else {
            None
        }
    }

    fn is_finished(self: &Self) -> bool {
        if self.move_count < 7 {
            false
        } else if check_win(self.bbs[1 - (self.move_count & 1)]) {
            true
        } else if self.move_count == SIZE {
            true
        } else {
            false
        }
    }

    fn ascii(self: &Self) -> String {
        panic!("Not implemented yet");
    }

    fn hash(self: &Self) -> usize {
        self.hash
    }

    fn symm_hash(self: &Self) -> usize {
        panic!("Not implemented yet");
    }

    fn get_lines_count(self: &Self, mv: Move) -> i32 {
        let mut bb = self.bbs[self.move_count & 1];
        bb |= 1 << self.counts[mv];

        // TODO: we can probably refactor the common part
        // of counting twos and threes
        let threes = count_threes(bb);
        let count = 10000 * threes;
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ones() {
        assert_eq!(ALL1, 0b1111111111111111111111111111111111111111111111111111111111111111);
    }

    #[test]
    fn test_column_of_ones() {
        // println!("COL1 = {:b}", COL1);
        assert_eq!(COL1, 0b0000000000000000000000000000000000000000000000000000000011111111);
    }

    #[test]
    fn test_bottom() {
        assert_eq!(BOTTOM, 0b0000000100000001000000010000000100000001000000010000000100000001);
    }

    #[test]
    fn test_hash() {
        let pos = BitPosition::new();
        assert_eq!(pos.hash(), BOTTOM);
    }

    #[test]
    fn test_hash_move_unmove() {
        let mut pos = BitPosition::new();
        for mv in pos.moves() {
            pos.make_move(mv);
            assert_ne!(pos.hash(), BOTTOM);
            pos.unmake_move(mv);
            assert_eq!(pos.hash(), BOTTOM);
        }
    }

    #[test]
    pub fn example_win_row() {
        let mut pos = BitPosition::new();
        pos.make_move(0);
        pos.make_move(0);
        pos.make_move(1);
        pos.make_move(1);
        pos.make_move(2);
        pos.make_move(2);
        pos.make_move(3);

        assert_eq!(pos.result(), Some(GameResult::Win(Player::Black)));
    }

    #[test]
    pub fn example_win_col() {
        let mut pos = BitPosition::new();
        pos.make_move(0);
        pos.make_move(1);
        pos.make_move(0);
        pos.make_move(1);
        pos.make_move(0);
        pos.make_move(1);
        pos.make_move(0);

        assert_eq!(pos.result(), Some(GameResult::Win(Player::Black)));
    }


    #[test]
    pub fn example_win_dia() {
        let mut pos = BitPosition::new();
        pos.make_move(0);
        pos.make_move(1);

        pos.make_move(1);
        pos.make_move(2);

        pos.make_move(3);
        pos.make_move(2);

        pos.make_move(2);
        pos.make_move(3);

        pos.make_move(3);
        pos.make_move(0);

        pos.make_move(3);

        println!("BB = {:b}", pos.bbs[0]);
        // 1101000
        // 0100000
        // 0010000
        // 0001
        assert!(pos.result() == Some(GameResult::Win(Player::Black)));
    }

    #[test]
    pub fn example_win_dia2() {
        let mut pos = BitPosition::new();
        pos.make_move(3);
        pos.make_move(2);

        pos.make_move(2);
        pos.make_move(1);

        pos.make_move(0);
        pos.make_move(1);

        pos.make_move(1);
        pos.make_move(0);

        pos.make_move(0);
        pos.make_move(3);

        pos.make_move(0);

        assert!(pos.result() == Some(GameResult::Win(Player::Black)));
    }
}