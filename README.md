# Connect Four in Rust

An implementation of the Connect Four game in Rust.

## Goal

The goal of the project was to solve the game with 6 rows and 7 columns. The final algorithm is able to do it in 6 minutes (bitboards) or ~10 minutes (array).

## Remarks

The main feature is a solver of the using:
- alpha-beta negamax
- a transposition table (with a fixed size, as opposed to std lib collections)
- a nice move ordering heuristing
- a fast bitboard implementation

The transposition table and bitboard implementation are heavily based on
https://github.com/tromp/fhourstones88

There is a nice writeup here:
https://github.com/denkspuren/BitboardC4/blob/master/BitboardDesign.md

The hashtable size was taken from: https://planetmath.org/goodhashtableprimes

I've tried to find a better move ordering heuristic by assigning weights to various patterns of 4-men lines by using a simple evolutionary algorithm, but the weights for smaller boards (5x5, 6x5, 5x6) would not work well for other sizes.

## RUN TESTS

```sh
$ cargo test   # unit tests
$ make test    # verify that solve returns correct answers
```

## PERFT

```sh
$ cargo run --release -q -- --perft 4
```

Example output (fast = transposition table, slow = naive):
```
Fast array
nodes after 11 moves = 44,973,684 [elapsed: 40] [speed: 1,124M nps]
Fast bitboard
nodes after 11 moves = 44,973,684 [elapsed: 48] [speed: 936M nps]
Slow array
nodes after 11 moves = 44,973,684 [elapsed: 772] [speed: 58M nps]
Slow bitboard
nodes after 11 moves = 44,973,684 [elapsed: 466] [speed: 96M nps]
```

## BENCHMARKS

```sh
make bench
```

Example:
```
rows = 4, cols = 4 ==>   0 | 00:00:00.606 |               11,736 nodes
rows = 5, cols = 4 ==>   0 | 00:00:00.600 |               37,202 nodes
rows = 4, cols = 5 ==>   0 | 00:00:00.600 |               53,871 nodes
rows = 6, cols = 4 ==>   0 | 00:00:00.762 |              410,789 nodes
...
```


## SOLVE THE GAME

We use the sizes as defined in `src/types.rs`. Change the values manually to check other sizes.

```sh
make solve
```

Example:

```
ArrayPosition
5 x 5 | result = 0 | nodes = 648,163 | [elapsed: 753] | ordering: 92% | [speed: 860K nps]
BitPosition (bitboard)
5 x 5 | result = 0 | nodes = 768,536 | [elapsed: 752] | ordering: 92% | [speed: 1,021K nps]
```