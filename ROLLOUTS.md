## Light rollouts

```
$ cargo run --release -q -- --rollout 5000
performed 5000 rollouts [elapsed: 0.516s] [speed: 9K rps]
```

We can do around 10k rollouts per second

## Bitboard benchmark

wjzz:~/prog/projects/games/gomoku/five-in-a-rust$ cargo run --release -q -- --rollout 5000
performed 5000 rollouts [elapsed: 0.459s] [speed: 10K rps]

## with preallocated buffer

performed 5000 rollouts [elapsed: 0.379s] [speed: 13K rps]

## Time comparison: no randomization

$ cargo run --release -q -- --rollout 5000
performed 5000 rollouts [elapsed: 0.233] [speed: 21K rps]
