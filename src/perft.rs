use std::time::Instant;
use thousands::Separable;

// use crate::board::Position;
use crate::bitboard::BitPosition;

pub fn perft(depth: usize) {
    let now = Instant::now();

    let mut pos = BitPosition::new();
    let node_count = pos.perft(depth);

    let mut elapsed_millisecs = now.elapsed().as_millis() as usize;
    if elapsed_millisecs == 0 {
        elapsed_millisecs = 1;
    }
    let nodes_per_sec = node_count / elapsed_millisecs / 1000;

    println!(
        "nodes after {} moves = {:} [elapsed: {}] [speed: {}M nps]",
        depth,
        node_count.separate_with_commas(),
        elapsed_millisecs,
        nodes_per_sec.separate_with_commas()
    );
}
