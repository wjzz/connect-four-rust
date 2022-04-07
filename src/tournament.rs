use std::time::Instant;

use crate::bestmove::*;
use crate::board::*;

// fn generate_weights_population(size: usize) -> Vec<AI> {
//     let mut results = vec![];

//     for i in 0..size {
//         let mut v: Vec<f64> = vec![rand(), rand(), rand(), rand()];
//         let total: f64 = v.iter().sum();
//         for a in v.iter_mut() {
//             *a /= total;
//         }
//         println!(
//             "{} = [{:+.2}, {:+.2}, {:+.2}, {:+.2}]",
//             i, v[0], v[1], v[2], v[3]
//         );
//         results.push(AI::Eval(v));
//     }
//     results
// }

fn result2score(result: GameResult) -> (f64, f64) {
    match result {
        GameResult::Draw => (0.5, 0.5),
        GameResult::Win(player) => match player {
            Player::Black => (1.0, 0.0),
            Player::White => (0.0, 1.0),
        },
    }
}

fn tournament(ais: Vec<AI>, tries: usize) {
    let mut scores = vec![0.0; ais.len()];

    for (i, first_ai) in ais.iter().enumerate() {
        for (j, second_ai) in ais.iter().enumerate() {
            if i < j {
                let first_before = scores[i];
                let second_before = scores[j];
                let now = Instant::now();
                for k in 0..tries {
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
                let elapsed_millisecs = now.elapsed().as_millis() as usize;

                println!(
                    "{:2} vs {:2}: {:2} - {:2} [average time per game: {}s]",
                    ais[i].show(),
                    ais[j].show(),
                    scores[i] - first_before,
                    scores[j] - second_before,
                    elapsed_millisecs / tries
                );
            }
        }
    }

    // summarize scores

    let mut v: Vec<(usize, usize)> = scores
        .iter()
        .enumerate()
        .map(|p| (*p.1 as usize, p.0))
        .collect();
    v.sort();
    v.reverse();

    println!("\nFinal standings");
    for (j, &(score, i)) in v.iter().enumerate() {
        println!("{}. {} [{} points]", j+1, ais[i].show(), score);
    }
}

pub fn compare_ais(tries: usize) {
    let ais = vec![
        AI::Random,
        AI::Mater,
        AI::MinMax(0),
        // AI::MinMax(1),
        // AI::MinMax(2),
        AI::Rollout(10),
        AI::Rollout(100),
    ];
    tournament(ais, tries);
}
