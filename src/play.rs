use std::time::Instant;

use crate::bestmove::*;
use crate::board::*;
use crate::types::*;
use crate::util::read_line;

fn get_move(pos: &ArrayPosition, ai: &AI) -> Move {
    if pos.to_play == Player::White {
        loop {
            println!("Play a move (eg. A5)> ");
            let line = read_line();
            return parse_move(line);
        }
    } else {
        let now = Instant::now();

        let mv = ai.bestmove(pos);
        let elapsed_secs = now.elapsed().as_secs() as usize;
        println!("played move {} after {} seconds", mv, elapsed_secs);
        return mv;
    }
}

pub fn play() {
    println!("You play as white");

    let mut pos = ArrayPosition::new();

    loop {
        println!("{}", pos.ascii());
        if let Some(result) = pos.result() {
            use GameResult::*;

            let msg = match result {
                Draw => "draw",
                Win(Player::Black) => "black wins",
                Win(Player::White) => "white wins",
            };
            println!("Game is finished! Result: {}", msg);
            break;
        }
        let ai = AI::MinMax(0);
        let mv = get_move(&pos, &ai);
        // TODO: print move
        pos.make_move(mv);
    }
}
