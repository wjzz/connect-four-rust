mod board;

use rand::prelude::*;
use board::*;

fn simulate_game(mv: Move, rng: &mut ThreadRng) -> GameResult {
    let mut pos = Position::new();
    pos.make_move(mv);

    loop {
        if let Some(result) = pos.is_finished() {
            return result;
        }
        let moves = pos.moves();

        pos.make_move(*moves.choose(rng).unwrap());
    }
}

fn get_win_percentage(mv: Move, rng: &mut ThreadRng, tries: usize) -> f64 {
    let mut draws = 0;
    let mut black = 0;
    let mut white = 0;

    for _i in 0..tries {
        match simulate_game(mv, rng) {
            GameResult::Draw => draws += 1,
            GameResult::Win(Player::Black) => black += 1,
            GameResult::Win(Player::White) => white += 1,
        }
    }
    100.0 * (black as f64) / (tries as f64)
}

fn find_best_move() {
    let mut rng = thread_rng();
    let mut win_ratios = vec![];

    for mv in 0..SIZE {
        let percentage = get_win_percentage(mv, &mut rng, 1000);
        win_ratios.push(percentage as usize);
    }

    for row in 0..ROWS {
        for col in 0..COLS {
            print!("{} ", win_ratios[rowcol2index(row, col)]);
        }
        println!();
    }
}

fn perft() {
    for i in 0 .. 5 {
        let mut pos = Position::new();
        let result = pos.perft(i);
        println!("{} -> {}", i, result);
    }
}

fn rand() -> f64 {
    rand::random::<f64>()
}

fn generate_weights_population(size: usize) -> Vec<AI> {
    let mut results = vec![];

    for i in 0 .. size {
        let mut v: Vec<f64> = vec![ rand(), rand(), rand(), rand()];
        let total: f64 = v.iter().sum();
        for a in v.iter_mut() {
            *a /= total;
        }
        println!("{} = [{:+.2}, {:+.2}, {:+.2}, {:+.2}]", i, v[0], v[1], v[2], v[3]);
        results.push(AI::Eval(v));
    }
    results
}

fn result2score(result: GameResult) -> (f64, f64) {
    match result {
        GameResult::Draw => (0.5, 0.5),
        GameResult::Win(player) =>
            match player {
                Player::Black => (1.0, 0.0),
                Player::White => (0.0, 1.0),
            }
    }
}

fn tournament(ais: Vec<AI>) {
    let mut scores = vec![0.0; ais.len()];

    for (i, first_ai) in ais.iter().enumerate() {
        for (j, second_ai) in ais.iter().enumerate() {
            if i < j {
                let first_before = scores[i];
                let second_before = scores[j];
                let tries = 2;
                for k in 0 .. tries {
                    if k % 2 == 0 {
                        let result = play_game(first_ai, second_ai);
                        let (score1, score2) = result2score(result);
                        scores[i] += score1;
                        scores[j] += score2;
                    } else {
                        let result = play_game(second_ai, first_ai);
                        let (score2, score1) = result2score(result);
                        scores[i] += score1;
                        scores[j] += score2;
                    }
                }
                println!("{:2} vs {:2}: {:2} - {:2}", i, j, scores[i] - first_before, scores[j] - second_before);
            }
        }
    }
    let mut v: Vec<(usize, usize)> = scores.iter().enumerate().map(|p| (*p.1 as usize, p.0)).collect();
    v.sort();
    for (score, i) in v {
        println!("{} -> {}", i, score);
    }
}

fn get_move(pos: &Position, ai: &AI) -> Move {
    if pos.to_play == Player::Black {
        loop {
            println!("Play a move (eg. A5)> ");
            let mut buffer = String::new();
            let stdin = std::io::stdin();
            stdin.read_line(&mut buffer).unwrap();
            if let Some(mv) = parse_move(buffer) {
                return mv;
            }
        }
    } else {
        bestmove(ai, pos)
    }
}

fn play() {
    println!("You play as black");

    let mut pos = Position::new();

    loop {
        println!("{}", pos.ascii());
        if let Some(result) = pos.is_finished() {
            let msg = match result {
                GameResult::Draw => "draw",
                GameResult::Win(Player::Black) => "black wins",
                GameResult::Win(Player::White) => "white wins",
            };
            println!("Game is finished! Result: {}", msg);
            break;
        }
        let ai = AI::Mater;
        let mv = get_move(&pos, &ai);
        // TODO: print move
        pos.make_move(mv);
    }
}

fn main() {
    // find_best_move();
    // perft();
    // let result = play_game(AI::Eval, AI::Eval);
    // println!("Result = {:?}", result);

    // let ais = generate_weights_population(10);
    // tournament(ais);

    play();
}
