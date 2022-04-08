use std::time::Instant;

use crate::bestmove::*;
use crate::board::*;
use crate::util::read_line;

fn get_move(pos: &Position, ai: &AI) -> Move {
    if pos.to_play == Player::White {
        loop {
            println!("Play a move (eg. A5)> ");
            let line = read_line();
            if let Some(mv) = parse_move(line) {
                return mv;
            }
        }
    } else {
        let now = Instant::now();

        let mv = ai.bestmove(pos);
        let elapsed_secs = now.elapsed().as_secs() as usize;
        println!(
            "played move {} after {} seconds",
            mv,
            elapsed_secs
        );
        return mv;
    }
}

pub fn play() {
    println!("You play as white");

    let mut pos = Position::new();

    loop {
        println!("{}", pos.ascii());
        if let Some(result) = pos.is_finished() {
            use GameResult::*;

            let msg = match result {
                Draw => "draw",
                Win(Player::Black) => "black wins",
                Win(Player::White) => "white wins",
            };
            println!("Game is finished! Result: {}", msg);
            break;
        }
        // let ai = AI::MinMax(0);
        let ai = AI::Rollout(50);
        let mv = get_move(&pos, &ai);
        // TODO: print move
        pos.make_move(mv);
    }
}
