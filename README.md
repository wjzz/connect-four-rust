## PERFT

```sh
$ cargo run --release -q -- --perft 4
```

## ROLLOUTS

```sh
cargo run --release -q -- --rollout 5000
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