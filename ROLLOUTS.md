## Light rollouts

```
$ cargo run --release -q -- --rollout 5000
performed 5000 rollouts [elapsed: 0.516s] [speed: 9K rps]
```

We can do around 10k rollouts per second

## Time comparison: no randomization

$ cargo run --release -q -- --rollout 5000
performed 5000 rollouts [elapsed: 0.233] [speed: 21K rps]
